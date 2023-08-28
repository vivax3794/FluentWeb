#![feature(box_patterns)]

use std::{env::current_dir, fs, path::PathBuf};

use html5ever::tendril::TendrilSink;
use quote::quote;

fn main() -> anyhow::Result<()> {
    let root_dir = current_dir()?;
    let src_fluent = root_dir.join("src_fluent");
    let src = root_dir.join("src");

    process_dir(src_fluent, src)?;

    Ok(())
}

fn process_dir(source: PathBuf, dst: PathBuf) -> anyhow::Result<()> {
    for file in fs::read_dir(&source)?.collect::<Result<Vec<_>, _>>()? {
        let name = file.file_name().to_str().unwrap().to_string();
        if file.file_type()?.is_dir() {
            let dst = dst.join(name.clone());

            if !dst.exists() {
                fs::create_dir(&dst)?;
            }

            process_dir(source.join(name), dst)?;
        } else {
            process_file(source.join(&name), dst.join(name))?;
        }
    }

    Ok(())
}

fn process_file(source: PathBuf, dst: PathBuf) -> anyhow::Result<()> {
    match source.extension().unwrap().to_str().unwrap() {
        "rs" => {
            fs::copy(source, dst)?;
        }
        "fluent" => {
            let name = dst.file_name().unwrap().to_str().unwrap().to_string();
            let component_name = name.split('.').next().unwrap();
            let component_file = format!("{component_name}Module.rs");
            compile_fluent_file(source, dst.with_file_name(component_file))?;
        }
        _ => (),
    }

    Ok(())
}

fn find_top_level_tag<'a>(document: &'a str, tag: &str) -> Option<&'a str> {
    let open_tag = format!("<{tag}>");
    let close_tag = format!("</{tag}>");

    let index_first = document.find(&open_tag)?;
    let region_start = index_first + open_tag.len();
    let region_end = document.find(&close_tag)?;

    Some(&document[region_start..region_end])
}

struct DataStatement {
    target: syn::Ident,
    type_: syn::Type,
    init_value: syn::Expr,
}

struct DataSectionInfo {
    struct_fields: Vec<proc_macro2::TokenStream>,
    create: Vec<proc_macro2::TokenStream>,
    import_struct_to_local_scope: proc_macro2::TokenStream,
    unpack_mut: proc_macro2::TokenStream,
}

fn parse_data_segement(data_section: &str) -> Vec<DataStatement> {
    let data_block_parsed: syn::Block = syn::parse_str(&format!("{{{data_section}}}")).unwrap();

    data_block_parsed
        .stmts
        .into_iter()
        .map(|stmt| match stmt {
            syn::Stmt::Local(syn::Local {
                pat:
                    syn::Pat::Type(syn::PatType {
                        pat:
                            box syn::Pat::Ident(syn::PatIdent {
                                ident: target,
                                by_ref: None,
                                mutability: Some(..),
                                ..
                            }),
                        ty: box type_,
                        ..
                    }),
                init:
                    Some(syn::LocalInit {
                        expr: box expr,
                        diverge: None,
                        ..
                    }),
                ..
            }) => DataStatement {
                target,
                type_,
                init_value: expr,
            },
            _ => panic!("expected let mut NAME: TYPE = VALUE;"),
        })
        .collect()
}

fn compile_data_section(source_content: &str) -> DataSectionInfo {
    let data_content = find_top_level_tag(source_content, "data").unwrap_or("");
    let data_statements = parse_data_segement(data_content);

    let struct_fields: Vec<_> = data_statements
        .iter()
        .map(|stmt| {
            let name = &stmt.target;
            let type_ = &stmt.type_;
            quote!(#name: ::std::rc::Rc<::std::cell::RefCell<#type_>>)
        })
        .collect();
    let create_statements: Vec<_> = data_statements
        .iter()
        .map(|stmt| {
            let name = &stmt.target;
            let expr = &stmt.init_value;
            quote!(#name: ::std::rc::Rc::new(::std::cell::RefCell::new(#expr)))
        })
        .collect();
    let field_getters = data_statements
        .iter()
        .map(|stmt| &stmt.target)
        .collect::<Vec<_>>();
    let field_borrows = data_statements
        .iter()
        .map(|stmt| {
            let target = &stmt.target;
            quote!(let #target = #target.borrow();)
        })
        .collect::<Vec<_>>();
    let field_unpacking = quote!(
        let __Fluid_Data { #(#field_getters),* } = &self.data;
        #(#field_borrows)*
    );
    let field_borrows_mut = data_statements
        .iter()
        .map(|stmt| {
            let target = &stmt.target;
            quote!(let mut #target = #target.borrow_mut();)
        })
        .collect::<Vec<_>>();
    let unpack_mut = quote!(
        let __Fluid_Data { #(#field_getters),* } = &__Fluent_Component.data;
        #(#field_borrows_mut)*
    );

    DataSectionInfo {
        struct_fields,
        create: create_statements,
        import_struct_to_local_scope: field_unpacking,
        unpack_mut,
    }
}

fn get_html_body(source_content: &str) -> kuchikiki::NodeRef {
    let template_content = find_top_level_tag(source_content, "template").unwrap_or("");
    let parsed_html = kuchikiki::parse_html_with_options(kuchikiki::ParseOpts {
        tree_builder: html5ever::tree_builder::TreeBuilderOpts {
            drop_doctype: true,
            ..Default::default()
        },
        ..Default::default()
    })
    .from_utf8()
    .read_from(&mut template_content.as_bytes())
    .unwrap();

    parsed_html
        .select("body")
        .unwrap()
        .next()
        .unwrap()
        .as_node()
        .clone()
}

fn create_reactive_update_function(
    reactive_text: &ReactiveText,
    unpack_data: &proc_macro2::TokenStream,
) -> (proc_macro2::TokenStream, proc_macro2::TokenStream) {
    let ReactiveText {
        expressions,
        id,
        text,
    } = reactive_text;

    let function_name = quote::format_ident!("update_element_{}", id);

    let function_def = quote! {
        fn #function_name(&self) {
            #unpack_data

            let __Fluent_Element = ::fluent_web_client::internal::get_element(&self.root_name, #id);
            __Fluent_Element.set_text_content(::std::option::Option::Some(&::std::format!(#text, #(::fluent_web_client::internal::display(&(#expressions))),*)));
        }
    };
    let call = quote!(self.#function_name(););

    (function_def, call)
}

fn get_or_set_id(attributes: &mut kuchikiki::Attributes, new_id: String) -> String {
    if let Some(id) = attributes.get("id") {
        id.to_string()
    } else {
        attributes.insert("id", new_id.clone());
        new_id
    }
}

struct SubComponentData {
    id: String,
    component_name: syn::Path,
}

fn find_sub_components_and_replace_with_div(node: kuchikiki::NodeRef) -> Vec<SubComponentData> {
    use kuchikiki::NodeData;
    match node.data() {
        NodeData::Element(data) if &data.name.local == "component" => {
            let attributes = data.attributes.borrow();
            let component_name = attributes
                .get("src")
                .expect("component tag needs a src as a rust path to point to the sub component");
            let component_name = syn::parse_str(component_name).unwrap();
            let id = uuid();

            use markup5ever::namespace_url;
            let mut attributes = attributes.clone();

            attributes.remove("src");
            let id = get_or_set_id(&mut attributes, id);

            let div = kuchikiki::NodeRef::new_element(
                html5ever::QualName {
                    prefix: None,
                    ns: markup5ever::ns!(html),
                    local: markup5ever::local_name!("div"),
                },
                attributes.map,
            );

            node.insert_before(div);
            node.detach();

            vec![SubComponentData { id, component_name }]
        }
        NodeData::Element(_) => node
            .children()
            .flat_map(find_sub_components_and_replace_with_div)
            .collect(),
        _ => vec![],
    }
}

fn create_spawn_and_spawn_call_for_subcomponent(
    data: &SubComponentData,
) -> (proc_macro2::TokenStream, proc_macro2::TokenStream) {
    let SubComponentData { id, component_name } = data;

    let function_name = quote::format_ident!("spawn_component_{id}");

    let function_def = quote!(
        fn #function_name(&self) {
            ::fluent_web_client::render_component::<#component_name>(#id);
        }
    );
    let function_call = quote!(self.#function_name(););

    (function_def, function_call)
}
struct ReactiveText {
    id: String,
    text: String,
    expressions: Vec<syn::Expr>,
}

fn find_reactive_nodes_and_replace_with_span(node: kuchikiki::NodeRef) -> Vec<ReactiveText> {
    use kuchikiki::NodeData;
    match node.data() {
        NodeData::Element(_) => node
            .children()
            .flat_map(find_reactive_nodes_and_replace_with_span)
            .collect(),
        NodeData::Text(text) => {
            let text = text.borrow();
            let mut format_string = String::with_capacity(text.len());
            let mut expressions: Vec<String> = Vec::new();

            let mut current_str = String::new();
            let mut in_template = false;
            for c in text.chars() {
                match c {
                    '{' => {
                        in_template = true;
                        format_string += "{";
                        current_str.clear();
                    }
                    '}' => {
                        in_template = false;
                        format_string += "}";
                        expressions.push(current_str.clone());
                        current_str.clear()
                    }
                    c => {
                        if in_template {
                            current_str.push(c);
                        } else {
                            format_string.push(c);
                        }
                    }
                }
            }

            if expressions.is_empty() {
                return vec![];
            }

            let id = uuid();

            use markup5ever::namespace_url;
            let new_text = kuchikiki::NodeRef::new_element(
                html5ever::QualName::new(
                    None,
                    markup5ever::ns!(html),
                    markup5ever::local_name!("span"),
                ),
                [(
                    kuchikiki::ExpandedName::new(
                        markup5ever::ns!(),
                        markup5ever::local_name!("id"),
                    ),
                    kuchikiki::Attribute {
                        prefix: None,
                        value: id.clone(),
                    },
                )],
            );

            node.insert_after(new_text);
            node.detach();

            let expressions = expressions
                .into_iter()
                .map(|raw| syn::parse_str(&raw).unwrap())
                .collect();

            vec![ReactiveText {
                id,
                text: format_string,
                expressions,
            }]
        }
        _ => vec![],
    }
}

struct EventListener {
    id: String,
    handler: String,
    code: syn::Block,
    element: String,
}

fn find_event_listeners_and_set_id(node: kuchikiki::NodeRef) -> Vec<EventListener> {
    use kuchikiki::NodeData;
    match node.data() {
        NodeData::Element(data) => {
            let element_name = data.name.local.to_string();

            let mut attributes = data.attributes.borrow_mut();
            let events = attributes
                .map
                .iter()
                .filter_map(|(name, code)| {
                    if name.local.starts_with(':') {
                        let event = name.local.strip_prefix(':').unwrap().to_string();
                        let code = code.value.clone();
                        Some((event, code))
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>();

            let mut events_for_this = if !events.is_empty() {
                let id = uuid();
                let id = get_or_set_id(&mut attributes, id);

                events
                    .into_iter()
                    .map(|(event, code)| {
                        attributes.remove(format!(":{event}"));

                        let code = syn::parse_str(&format!("{{{code}}}")).unwrap();
                        EventListener {
                            id: id.clone(),
                            handler: event,
                            code,
                            element: element_name.clone(),
                        }
                    })
                    .collect()
            } else {
                vec![]
            };

            events_for_this.extend(node.children().flat_map(find_event_listeners_and_set_id));
            events_for_this
        }
        _ => vec![],
    }
}

fn compile_event_listener(
    event: &EventListener,
    unpack_mut: &proc_macro2::TokenStream,
) -> (proc_macro2::TokenStream, proc_macro2::TokenStream) {
    let EventListener {
        id,
        handler,
        code,
        element,
    } = event;
    let function_name = quote::format_ident!("set_event_{}", id);

    let mut c = element.chars();
    let element_name_cap = c.next().unwrap().to_ascii_uppercase().to_string() + c.as_str();
    let element_type = quote::format_ident!("Html{}Element", element_name_cap);

    let event_type = match handler.as_str() {
        "keydown" => "Keyboard",
        "keyup" => "Keyboard",
        "keypress" => "Keyboard",
        "click" => "Mouse",
        "dblclick" => "Mouse",
        "mousedown" => "Mouse",
        "mouseup" => "Mouse",
        "mousemove" => "Mouse",
        "mouseover" => "Mouse",
        "mouseout" => "Mouse",
        "mouseenter" => "Mouse",
        "mouseleave" => "Mouse",
        "contextmenu" => "Mouse",
        "wheel" => "Mouse",
        "submit" => "Form",
        "change" => "Form",
        "focus" => "Form",
        "blur" => "Form",
        "input" => "Form",
        "reset" => "Form",
        "select" => "Form",
        "load" => "Window",
        "beforeunload" => "Window",
        "unload" => "Window",
        "resize" => "Window",
        "scroll" => "Window",
        "drag" => "DragDrop",
        "dragstart" => "DragDrop",
        "dragend" => "DragDrop",
        "dragenter" => "DragDrop",
        "dragleave" => "DragDrop",
        "dragover" => "DragDrop",
        "drop" => "DragDrop",
        "cut" => "Clipboard",
        "copy" => "Clipboard",
        "paste" => "Clipboard",
        "play" => "Media",
        "pause" => "Media",
        "ended" => "Media",
        "timeupdate" => "Media",
        "canplay" => "Media",
        "canplaythrough" => "Media",
        "volumechange" => "Media",
        "seeked" => "Media",
        "seeking" => "Media",
        "durationchange" => "Media",
        "loadedmetadata" => "Media",
        "loadeddata" => "Media",
        "progress" => "Media",
        "ratechange" => "Media",
        "stalled" => "Media",
        "suspend" => "Media",
        "waiting" => "Media",
        "error" => "Other",
        "online" => "Other",
        "offline" => "Other",
        "message" => "Other",
        "popstate" => "Other",
        "storage" => "Other",
        _ => panic!(),
    };
    let event_type = quote::format_ident!("{}Event", event_type);

    let set_event_handler = quote!(
        fn #function_name(&self) {
            let __Fluent_Component = self.clone();

            use ::fluent_web_client::internal::wasm_bindgen::JsCast;
            let __Fluent_Element = ::fluent_web_client::internal::get_element(&self.root_name, #id);
            let __Fluent_Element: &::fluent_web_client::internal::web_sys::#element_type = __Fluent_Element.dyn_ref().unwrap();

            let element = __Fluent_Element.clone();

            let __Fluent_Function = ::fluent_web_client::internal::wasm_bindgen::closure::Closure::<dyn Fn(_)>::new(move |event: ::fluent_web_client::internal::web_sys::Event| {
                let event = event.dyn_ref::<::fluent_web_client::internal::web_sys::#event_type>().unwrap();
                {
                    #unpack_mut
                    #code;
                }
                use ::fluent_web_client::internal::Component;
                __Fluent_Component.update_all();
            });
            __Fluent_Element.add_event_listener_with_callback(#handler, __Fluent_Function.as_ref().unchecked_ref()).unwrap();
            __Fluent_Function.forget();
        }
    );

    let call = quote!(self.#function_name(););

    (set_event_handler, call)
}

struct ConditionalAttribute {
    id: String,
    attribute: String,
    condition: syn::Expr,
}

fn find_conditional_attributes_and_set_id(node: kuchikiki::NodeRef) -> Vec<ConditionalAttribute> {
    use kuchikiki::NodeData;
    match node.data() {
        NodeData::Element(data) => {
            let mut attributes = data.attributes.borrow_mut();
            let conditionals = attributes
                .map
                .iter()
                .filter_map(|(name, content)| {
                    if name.local.starts_with('?') {
                        Some((name.local.to_string(), content.value.clone()))
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>();

            let mut this_element = if conditionals.is_empty() {
                vec![]
            } else {
                let id = uuid();
                let id = get_or_set_id(&mut attributes, id);

                conditionals
                    .into_iter()
                    .map(|(name, code)| {
                        attributes.remove(name.clone());

                        let code = syn::parse_str(&code).unwrap();
                        let name = name.strip_prefix('?').unwrap().to_string();

                        ConditionalAttribute {
                            id: id.clone(),
                            attribute: name,
                            condition: code,
                        }
                    })
                    .collect()
            };

            this_element.extend(
                node.children()
                    .flat_map(find_conditional_attributes_and_set_id),
            );
            this_element
        }
        _ => vec![],
    }
}

fn compile_conditional_stmt(
    attribute: &ConditionalAttribute,
    unpack: &proc_macro2::TokenStream,
) -> (proc_macro2::TokenStream, proc_macro2::TokenStream) {
    let ConditionalAttribute {
        id,
        attribute,
        condition,
    } = attribute;

    let function_name = quote::format_ident!("update_attribute_{}", id);

    let update_def = quote!(
        fn #function_name(&self) {
            #unpack

            let __Fluent_Element = ::fluent_web_client::internal::get_element(&self.root_name, #id);
            if #condition {
                __Fluent_Element.set_attribute(#attribute, "").unwrap();
            } else {
                __Fluent_Element.remove_attribute(#attribute).unwrap();
            }
        }
    );
    let update_call = quote!(self.#function_name(););

    (update_def, update_call)
}

fn compile_fluent_file(source: PathBuf, dst: PathBuf) -> anyhow::Result<()> {
    let source_content = fs::read_to_string(source)?;

    let DataSectionInfo {
        struct_fields: data_struct_fields,
        create: data_create_fields,
        import_struct_to_local_scope: unpack_data,
        unpack_mut,
    } = compile_data_section(&source_content);

    let define_content = find_top_level_tag(&source_content, "define").unwrap_or("");
    let define_parsed: syn::File = syn::parse_str(define_content)?;

    let body_html = get_html_body(&source_content);

    let reactive_element_info = find_reactive_nodes_and_replace_with_span(body_html.clone());
    let (reactive_update_defs, reactive_update_calls): (Vec<_>, Vec<_>) = reactive_element_info
        .iter()
        .map(|info| create_reactive_update_function(info, &unpack_data))
        .unzip();

    let subcomponent_info = find_sub_components_and_replace_with_div(body_html.clone());
    let (subcomponent_defs, subcomponent_calls): (Vec<_>, Vec<_>) = subcomponent_info
        .iter()
        .map(create_spawn_and_spawn_call_for_subcomponent)
        .unzip();

    let event_info = find_event_listeners_and_set_id(body_html.clone());
    let (event_set_defs, event_set_calls): (Vec<_>, Vec<_>) = event_info
        .iter()
        .map(|event| compile_event_listener(event, &unpack_mut))
        .unzip();

    let conditional_info = find_conditional_attributes_and_set_id(body_html.clone());
    let (conditional_defs, conditional_calls): (Vec<_>, Vec<_>) = conditional_info
        .iter()
        .map(|info| compile_conditional_stmt(info, &unpack_data))
        .unzip();

    let mut html_content = Vec::new();
    html5ever::serialize(&mut html_content, &body_html, Default::default()).unwrap();
    let html_content = String::from_utf8(html_content).unwrap();
    dbg!(html_content.clone());

    let component_source: syn::File = syn::parse_quote!(
        use ::fluent_web_client::internal::web_sys::*;

        #define_parsed

        #[derive(Clone)]
        struct __Fluid_Data {
            #(#data_struct_fields),*
        }

        #[derive(Clone)]
        pub struct Component {
            root_name: ::std::string::String,
            data: __Fluid_Data,
        }

        impl Component {
            #(#subcomponent_defs)*
            #(#reactive_update_defs)*
            #(#conditional_defs)*
            #(#event_set_defs)*
        }

        impl ::fluent_web_client::internal::Component for Component {
            fn render_init(&self) -> ::std::string::String {
                #html_content.into()
            }

            fn create(root_id: String) -> Self {
                Self {
                    root_name: root_id,
                    data: __Fluid_Data {
                        #(#data_create_fields),*
                    }
                }
            }

            fn setup_events(&self) {
                #(#event_set_calls)*
            }

            fn update_all(&self) {
                #(#reactive_update_calls)*
                #(#conditional_calls)*
            }

            fn spawn_sub(&self) {
                #(#subcomponent_calls)*
            }
        }
    );
    let component_source = prettyplease::unparse(&component_source);

    fs::write(dst, component_source)?;

    Ok(())
}

fn uuid() -> String {
    let id = uuid::Uuid::new_v4().to_string().replace('-', "_");
    format!("__Fluent_UUID_{id}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find() {
        const CONTENT: &str = "<a>Hello World</a> <b>123</b>";

        assert_eq!("Hello World", find_top_level_tag(CONTENT, "a").unwrap());
        assert_eq!("123", find_top_level_tag(CONTENT, "b").unwrap())
    }
}
