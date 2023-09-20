#![feature(box_patterns)]
#![warn(clippy::unwrap_used)]

use std::{
    borrow::BorrowMut, convert::Infallible, env::current_dir, fs,
    path::PathBuf,
};

use html5ever::tendril::TendrilSink;
use lightningcss::visitor::Visit;
use quote::quote;

/// Compiles all files in ./src_fluent into ./src
fn main() -> anyhow::Result<()> {
    let root_dir = current_dir()?;
    let src_fluent = root_dir.join("src_fluent");
    let src = root_dir.join("src");

    clear_out_src_dir(&src)?;
    process_dir(src_fluent, src)?;

    Ok(())
}

/// Clear out the src directory to stop compilation errors from stopping trunk.
fn clear_out_src_dir(src: &PathBuf) -> anyhow::Result<()> {
    fs::remove_dir_all(src)?;
    fs::create_dir(src)?;

    fs::File::create(src.join("main.rs"))?;

    Ok(())
}

/// Loop over all files in source directory and compile them into dst
fn process_dir(source: PathBuf, dst: PathBuf) -> anyhow::Result<()> {
    for file in
        fs::read_dir(&source)?.collect::<Result<Vec<_>, _>>()?
    {
        let name = file
            .file_name()
            .to_str()
            .expect("Could not convert file name to string")
            .to_string();
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

/// Copy rust files unchanged, and run the compilation for fluent files
fn process_file(source: PathBuf, dst: PathBuf) -> anyhow::Result<()> {
    match source
        .extension()
        .expect("Could not find extension")
        .to_str()
        .expect("Could not convert extension to string")
    {
        "rs" => {
            fs::copy(source, dst)?;
        }
        "fluent" => {
            // App.fluent => AppModule.rs
            let name = dst
                .file_name()
                .expect("Could not find filename")
                .to_str()
                .expect("Could not convert filename to string")
                .to_string();
            let component_name = name
                .split('.')
                .next()
                .expect("Expected dot in filename");
            let component_file = format!("{component_name}Module.rs");
            compile_fluent_file(
                source,
                dst.with_file_name(component_file),
            )?;
        }
        _ => (),
    }

    Ok(())
}

/// Find top level tags as a proper html parser would "corrupt" the content of the rust tags
fn find_top_level_tag<'a>(
    document: &'a str,
    tag: &str,
) -> Option<&'a str> {
    let open_tag = format!("<{tag}>");
    let close_tag = format!("</{tag}>");

    let index_first = document.find(&open_tag)?;
    let region_start = index_first + open_tag.len();
    let region_end = document.find(&close_tag)?;

    Some(&document[region_start..region_end])
}

/// A statement in the <data> block
struct DataStatement {
    /// Name for the property
    target: syn::Ident,
    /// The type, this is used in the struct definition
    type_: syn::Type,
    /// Expression to set inital value, this is used in the ::new() method
    init_value: syn::Expr,
    is_prop: bool,
}

/// Info about the data fields to be used in other generation calls
struct DataSectionInfo {
    /// Fields used to generate struct
    struct_fields: Vec<proc_macro2::TokenStream>,
    /// Create instance of the struct
    create: Vec<proc_macro2::TokenStream>,
    /// Unpack and borrow all data fields (this assumes `&self`)
    import_struct_to_local_scope: proc_macro2::TokenStream,
    /// Unpack and borrow mutable references to all data fields, (this assumes `&mut self`)
    unpack_mut: proc_macro2::TokenStream,
    unpack_change_detector: proc_macro2::TokenStream,
    reactive_updates: Vec<proc_macro2::TokenStream>,
    targets: Vec<syn::Ident>,
}

/// Parse the data block
fn parse_data_and_props_segement(
    data_section: &str,
    is_prop: bool,
) -> Vec<DataStatement> {
    let data_block_parsed: syn::Block = syn::parse_str(&format!(
        "{{{data_section}}}"
    ))
    .expect("Valid statements in block should still be valid");

    data_block_parsed
        .stmts
        .into_iter()
        .map(|stmt| match stmt {
            // Match agains a:
            // let mut TARGET: _TYPE = EXPR;
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
                is_prop,
            },
            _ => panic!("expected let mut NAME: TYPE = VALUE;"),
        })
        .collect()
}

/// Create all the code gen for the data fields
fn compile_data_section(
    data_statements: &Vec<DataStatement>,
    ty_generics: &proc_macro2::TokenStream,
) -> DataSectionInfo {
    let struct_fields: Vec<_> = data_statements
        .iter()
        .map(|stmt| {
            let name = &stmt.target;
            let type_ = &stmt.type_;
            quote!(#name: ::fluent_web_client::internal::ChangeDetector<#type_>,)
        })
        .collect();
    let create_statements: Vec<_> = data_statements
        .iter()
        .map(|stmt| {
            let name = &stmt.target;
            let expr = &stmt.init_value;
            quote!(#name: ::fluent_web_client::internal::ChangeDetector::new(#expr),)
        })
        .collect();
    let field_getters = data_statements
        .iter()
        .map(|stmt| &stmt.target)
        .collect::<Vec<_>>();

    let borrow = data_statements
        .iter()
        .map(|stmt| {
            let target = &stmt.target;
            quote!(let #target = #target.borrow();)
        })
        .collect::<Vec<_>>();
    let borrow_mut = data_statements
        .iter()
        .map(|stmt| {
            let target = &stmt.target;
            if stmt.is_prop {
                quote!(let #target = #target.borrow();)
            } else {
                quote!(let mut #target = #target.borrow_mut();)
            }
        })
        .collect::<Vec<_>>();

    let unpack_change_detector = quote!(
        let __Fluid_Data { #(#field_getters,)* .. } = self.data.clone();
    );

    let field_unpacking = quote!(
        #unpack_change_detector
        #(#borrow)*
    );

    let unpack_mut = quote!(
        #unpack_change_detector
        #(#borrow_mut)*
    );

    let reactive_fields = data_statements
        .iter()
        .map(|stmt| {
            let target = &stmt.target;
            quote!(#target: ::std::collections::HashSet<fn(&Component #ty_generics)>,)
        })
        .collect::<Vec<_>>();

    DataSectionInfo {
        struct_fields,
        create: create_statements,
        import_struct_to_local_scope: field_unpacking,
        unpack_mut,
        unpack_change_detector,
        reactive_updates: reactive_fields,
        targets: data_statements
            .iter()
            .map(|stmt| stmt.target.clone())
            .collect(),
    }
}

struct PropInfo {
    watcher: proc_macro2::TokenStream,
    update: proc_macro2::TokenStream,
}

// You will be able to modify the props.
// Flag to say they are props.

fn compile_props(props: &Vec<DataStatement>) -> PropInfo {
    let props = props
        .iter()
        .map(|prop| {
            let ident = &prop.target;
            let attribute = ident.to_string();

            quote!(
                if let Some(value) = element.get_attribute(#attribute) {
                    use ::fluent_web_client::internal::base64::engine::Engine;
                    let decoded = ::fluent_web_client::internal::base64::engine::general_purpose::STANDARD_NO_PAD.decode(value).unwrap();
                    let deserialized = ::fluent_web_client::internal::bincode::deserialize(&decoded).unwrap();
                    * self.data.#ident.borrow_mut() = deserialized;
                }
            )
        })
        .collect::<Vec<_>>();

    let update = quote!(
        fn update_props(&self) {
            let element = ::fluent_web_client::internal::get_by_id(
                &self.root_name,
            );
            #(#props)*
            self.update_changed_values();
        }
    );
    let watcher = quote!(
        fn setup_watcher(&self) {
            use ::fluent_web_client::internal::wasm_bindgen::JsCast;
            let component = self.clone();
            let function = move || component.update_props();
            let function = ::fluent_web_client::internal::wasm_bindgen::closure::Closure::<dyn Fn()>::new(function);
            let js_function = function.as_ref().unchecked_ref();
            let observer = ::fluent_web_client::internal::web_sys::MutationObserver::new(js_function).unwrap();
            function.forget();

            let element = ::fluent_web_client::internal::get_by_id(
                &self.root_name,
            );

            let mut options = ::fluent_web_client::internal::web_sys::MutationObserverInit::new();
            options.attributes(true);
            observer.observe_with_options(&element, &options);
        }
    );

    PropInfo { watcher, update }
}

/// Get the html of a template.
fn get_html_body(source_content: &str) -> kuchikiki::NodeRef {
    let template_content =
        find_top_level_tag(source_content, "template").unwrap_or("");
    let parsed_html =
        kuchikiki::parse_html_with_options(kuchikiki::ParseOpts {
            tree_builder: html5ever::tree_builder::TreeBuilderOpts {
                drop_doctype: true,
                ..Default::default()
            },
            ..Default::default()
        })
        .from_utf8()
        .read_from(&mut template_content.as_bytes())
        .expect("Valid html");

    parsed_html
        .select("body")
        .expect("A valid selector")
        .next()
        .expect("There to be a <body> tag")
        .as_node()
        .clone()
}

/// Create the update functions for reactive <spans>
/// returns: (function_def, call)
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

    let selector = format!(".{}", id);
    let function_def = quote! {
        fn #function_name(&self) {
            #unpack_data

            let __Fluent_Elements = ::fluent_web_client::internal::get_elements(&self.root_name, #selector);
            for __Fluent_Element in __Fluent_Elements.into_iter() {
                let __Fluent_Text = &::std::format!(#text, #(::fluent_web_client::internal::display(&(#expressions))),*);
                __Fluent_Element.set_text_content(::std::option::Option::Some(__Fluent_Text));
            }

            self.detect_reads(Component::#function_name);
        }
    };
    let call = quote!(self.#function_name(););

    (function_def, call)
}

/// Add a class to the `class` attribute on a node
/// This also creates the class attribute if it is not present.
fn add_class(attributes: &mut kuchikiki::Attributes, class: &str) {
    let current_class =
        if let Some(value) = attributes.get_mut("class") {
            value
        } else {
            attributes.insert("class", String::new());
            attributes
                .get_mut("class")
                .expect("Newly inserted class to be there")
        };

    current_class.push(' ');
    current_class.push_str(class);
}

/// Data used to create sub components
struct SubComponentData {
    id: String,
    component_name: syn::Path,
}

/// This finds <componet> tags, parses and stores its `src` and then replaces it with a <div>
fn find_sub_components_and_replace_with_div(
    node: kuchikiki::NodeRef,
) -> Vec<SubComponentData> {
    use kuchikiki::NodeData;
    match node.data() {
        NodeData::Element(data)
            if &data.name.local == "component" =>
        {
            let attributes = data.attributes.borrow();
            let component_name = attributes
                .get("src")
                .expect("component tag needs a src as a rust path to point to the sub component");
            let component_name = syn::parse_str(component_name)
                .expect("Component src to be valid rust path");
            let id = uuid();

            use markup5ever::namespace_url;
            let mut attributes = attributes.clone();

            attributes.remove("src");
            add_class(&mut attributes, &id);
            add_class(&mut attributes, "__Fluent_Needs_Init");

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

/// Create the calls used for creating sub components
/// returns (function_def, call)
fn create_spawn_and_spawn_call_for_subcomponent(
    data: &SubComponentData,
) -> (proc_macro2::TokenStream, proc_macro2::TokenStream) {
    let SubComponentData { id, component_name } = data;

    let function_name = quote::format_ident!("spawn_component_{id}");

    let selector = format!(".{}.__Fluent_Needs_Init", id);
    let function_def = quote!(
        fn #function_name(&self) {
            let __Fluent_Elements = ::fluent_web_client::internal::get_elements(&self.root_name, #selector);
            for __Fluent_Element in __Fluent_Elements.into_iter() {
                let __Fluent_Id = ::fluent_web_client::internal::uuid();
                __Fluent_Element.set_id(&__Fluent_Id);
                ::fluent_web_client::render_component::<#component_name>(&__Fluent_Id);
            }
        }
    );
    let function_call = quote!(self.#function_name(););

    (function_def, function_call)
}

/// Represents the reactive text in a <span>
struct ReactiveText {
    /// Id of the span element, used to update it later
    id: String,
    /// The format text
    text: String,
    /// A list of expressions for each {} in the tag.
    expressions: Vec<syn::Expr>,
}

/// Find all text with {} and replace the text with a <span> that can be targeted by code.
fn find_reactive_nodes_and_replace_with_span(
    node: kuchikiki::NodeRef,
) -> Vec<ReactiveText> {
    use kuchikiki::NodeData;
    match node.data() {
        NodeData::Element(_) => node
            .children()
            .flat_map(find_reactive_nodes_and_replace_with_span)
            .collect(),
        NodeData::Text(text) => {
            let text = text.borrow();
            let (format_string, expressions) =
                extract_format_strings(&text);

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
                        markup5ever::local_name!("class"),
                    ),
                    kuchikiki::Attribute {
                        prefix: None,
                        value: id.clone(),
                    },
                )],
            );

            node.insert_after(new_text);
            node.detach();

            vec![ReactiveText {
                id,
                text: format_string,
                expressions,
            }]
        }
        _ => vec![],
    }
}

fn extract_format_strings(text: &str) -> (String, Vec<syn::Expr>) {
    let mut format_string = String::with_capacity(text.len());
    let mut expressions = Vec::new();

    let mut current_str = String::new();
    let mut in_template = false;

    // Find all {} pairs in the text.
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
                expressions.push(
                    syn::parse_str(&current_str).expect(
                        "format content to be valid expression",
                    ),
                );
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
    (format_string, expressions)
}

/// Represents the info for a event callback
struct EventListener {
    /// Element id
    id: String,
    /// The event to handle
    handler: String,
    /// Code block for the event handler
    code: syn::ExprClosure,
    /// The type of element
    element: String,
    event_component: Option<proc_macro2::TokenStream>,
}

/// Find all event listeners, set an ID so they can be found and then parse and store their info.
fn find_event_listeners_and_set_class(
    node: kuchikiki::NodeRef,
) -> Vec<EventListener> {
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
                        let event = name
                            .local
                            .strip_prefix(':')
                            .expect("String starting with : to start with :")
                            .to_string();
                        let code = code.value.clone();
                        Some((event, code, false))
                    } else if name.local.starts_with(';') {
                        let event = name
                            .local
                            .strip_prefix(';')
                            .expect("String starting with ; to start with ;")
                            .to_string();
                        let code = code.value.clone();
                        Some((event, code, true))
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>();

            let mut events_for_this = if !events.is_empty() {
                let id = uuid();
                add_class(&mut attributes, &id);

                events
                    .into_iter()
                    .map(|(event, code, component)| {
                        attributes.remove(format!(":{event}"));

                        let code =
                            syn::parse_str(&code)
                                .expect(
                                    "event handler to be valid code.",
                                );

                        let event_component = if component {
                            Some(
                                syn::parse_str(
                                    attributes.get("src").expect(
                                        "@Event to have a src attribute",
                                    ),
                                )
                                .expect("src to be valid rust path")
                            )
                        } else {
                            None
                        };

                        EventListener {
                            id: id.clone(),
                            handler: event,
                            code,
                            element: element_name.clone(),
                            event_component,
                        }
                    })
                    .collect()
            } else {
                vec![]
            };

            events_for_this.extend(
                node.children()
                    .flat_map(find_event_listeners_and_set_class),
            );
            events_for_this
        }
        _ => vec![],
    }
}

/// Create the event listeners
fn compile_event_listener(
    event: &EventListener,
    unpack_mut: &proc_macro2::TokenStream,
) -> (proc_macro2::TokenStream, proc_macro2::TokenStream) {
    let EventListener {
        id,
        handler,
        code,
        element,
        event_component,
    } = event;
    let mut code = code.clone();

    let function_name = quote::format_ident!("set_event_{}", uuid());
    let function_name_internal =
        quote::format_ident!("{}_interal", function_name);

    let element = element.clone();
    let element = if event_component.is_none() {
        element
    } else {
        String::from("div")
    };
    let mut c = element.chars();
    let element_name_cap = c
        .next()
        .expect("Event name to have at least one letter")
        .to_ascii_uppercase()
        .to_string()
        + c.as_str();
    let element_type =
        quote::format_ident!("Html{}Element", element_name_cap);

    let event_reading = if let Some(component_path) = event_component
    {
        let event_name =
            quote::format_ident!("__Fluent_Event_{}", handler);
        let event_type = quote!(#component_path::#event_name);

        let first = code.inputs.first_mut().expect("argument");
        *first = syn::Pat::Type(syn::PatType {
            attrs: vec![],
            pat: Box::new(first.clone()),
            colon_token: <syn::Token![:]>::default(),
            ty: Box::new(syn::parse_quote!(#event_type::Parent)),
        });

        quote!(
            let __Fluent_Custom_Event = event.dyn_ref::<::fluent_web_client::internal::web_sys::CustomEvent>().unwrap();
            let __Fluent_Details = __Fluent_Custom_Event.detail();
            let __Fluent_Details = __Fluent_Details.dyn_ref::<::fluent_web_client::internal::js_sys::Uint8Array>().unwrap();
            let __Fluent_Event: #event_type = ::fluent_web_client::internal::bincode::deserialize(&__Fluent_Details.to_vec()).unwrap();
            let __Fluent_Event = __Fluent_Event.0;
        )
    } else {
        // Just ask people to downcast it themself?
        let event_type = quote::format_ident!("Event");
        quote!(let __Fluent_Event = event.dyn_ref::<::fluent_web_client::internal::web_sys::#event_type>().unwrap();)
    };

    let second = &mut code.inputs[1];
    *second = syn::Pat::Type(syn::PatType {
        attrs: vec![],
        pat: Box::new(second.clone()),
        colon_token: <syn::Token![:]>::default(),
        ty: Box::new(syn::parse_quote!(&#element_type)),
    });

    let selector = format!(".{}", id);
    let set_event_handler = quote!(
        fn #function_name_internal(self, __Fluent_Element: ::fluent_web_client::internal::web_sys::Element) {
            use ::fluent_web_client::internal::wasm_bindgen::JsCast;

            let __Fluent_Element: &::fluent_web_client::internal::web_sys::#element_type = __Fluent_Element.dyn_ref().unwrap();
            let __Fluent_Element_Clone = __Fluent_Element.clone();

            let __Fluent_Function = ::fluent_web_client::internal::wasm_bindgen::closure::Closure::<dyn Fn(_)>::new(move |event: ::fluent_web_client::internal::web_sys::Event| {
                #event_reading
                {
                    #unpack_mut
                    (#code)(__Fluent_Event, &__Fluent_Element_Clone);
                }
                use ::fluent_web_client::internal::Component;
                self.update_changed_values();
            });
            __Fluent_Element.add_event_listener_with_callback(#handler, __Fluent_Function.as_ref().unchecked_ref()).unwrap();
            __Fluent_Function.forget();
        }

        fn #function_name(&self) {
            let __Fluent_Elements = ::fluent_web_client::internal::get_elements(&self.root_name, #selector);

            for __Fluent_Element in __Fluent_Elements.into_iter() {
                self.clone().#function_name_internal(__Fluent_Element);
            }
        }
    );

    let call = quote!(self.#function_name(););

    (set_event_handler, call)
}

/// Info for a attribute that is added and removed reactivly.
struct ConditionalAttribute {
    id: String,
    attribute: String,
    condition: syn::Expr,
}

/// Find conditional attributes
fn find_conditional_attributes_and_set_id(
    node: kuchikiki::NodeRef,
) -> Vec<ConditionalAttribute> {
    use kuchikiki::NodeData;
    match node.data() {
        NodeData::Element(data) => {
            let mut attributes = data.attributes.borrow_mut();
            let conditionals = attributes
                .map
                .iter()
                .filter_map(|(name, content)| {
                    if name.local.starts_with('?') {
                        Some((
                            name.local.to_string(),
                            content.value.clone(),
                        ))
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>();

            let mut this_element = if conditionals.is_empty() {
                vec![]
            } else {
                let id = uuid();
                add_class(&mut attributes, &id);

                conditionals
                    .into_iter()
                    .map(|(name, code)| {
                        attributes.remove(name.clone());

                        let code = syn::parse_str(&code).expect("Conditional expression to be valid expression");
                        let name = name
                            .strip_prefix('?')
                            .expect("Name to start with ?")
                            .to_string();

                        ConditionalAttribute {
                            id: id.clone(),
                            attribute: name,
                            condition: code,
                        }
                    })
                    .collect()
            };

            this_element
                .extend(node.children().flat_map(
                    find_conditional_attributes_and_set_id,
                ));
            this_element
        }
        _ => vec![],
    }
}

/// Compile a conditional attribute
fn compile_conditional_stmt(
    attribute: &ConditionalAttribute,
    unpack: &proc_macro2::TokenStream,
) -> (proc_macro2::TokenStream, proc_macro2::TokenStream) {
    let ConditionalAttribute {
        id,
        attribute,
        condition,
    } = attribute;

    let function_name =
        quote::format_ident!("update_attribute_{}", uuid());

    let selector = format!(".{}", id);
    let update_def = quote!(
        fn #function_name(&self) {
            #unpack

            let __Fluent_Elements = ::fluent_web_client::internal::get_elements(&self.root_name, #selector);
            for __Fluent_Element in __Fluent_Elements.into_iter() {
                if #condition {
                    __Fluent_Element.set_attribute(#attribute, "").unwrap();
                } else {
                    __Fluent_Element.remove_attribute(#attribute).unwrap();
                }
            }
            self.detect_reads(Component::#function_name);
        }
    );
    let update_call = quote!(self.#function_name(););

    (update_def, update_call)
}

struct Generics {
    impl_generics: proc_macro2::TokenStream,
    ty_generics: proc_macro2::TokenStream,
    where_clauses: proc_macro2::TokenStream,
    generic_def: proc_macro2::TokenStream,
    phantom: proc_macro2::TokenStream,
}

fn parse_generics(source_content: &str) -> Generics {
    let generic_content =
        find_top_level_tag(source_content, "generic");

    match generic_content {
        None => Generics {
            impl_generics: quote!(),
            ty_generics: quote!(),
            where_clauses: quote!(),
            generic_def: quote!(),
            phantom: quote!(::std::marker::PhantomData<()>),
        },
        Some(generic_content) => {
            let fake_item: syn::ItemStruct = syn::parse_str(
                &format!("struct Fake{generic_content};"),
            )
            .expect("Valid generics");

            let generics = fake_item.generics;

            let (impl_generic, ty_generics, where_clauses) =
                generics.split_for_impl();

            let phantom_args = generics
                .params
                .iter()
                .map(|param| match param {
                    syn::GenericParam::Const(syn::ConstParam {
                        ident,
                        ..
                    }) => quote!(#ident),
                    syn::GenericParam::Lifetime(
                        syn::LifetimeParam { lifetime, .. },
                    ) => quote!(#lifetime),
                    syn::GenericParam::Type(syn::TypeParam {
                        ident,
                        ..
                    }) => quote!(#ident),
                })
                .collect::<Vec<_>>();

            Generics {
                impl_generics: quote!(#impl_generic),
                ty_generics: quote!(#ty_generics),
                where_clauses: quote!(#where_clauses),
                generic_def: quote!(#generics #where_clauses),
                phantom: quote!(::std::marker::PhantomData<(#(#phantom_args),*)>),
            }
        }
    }
}

fn parse_events(source_content: &str) -> Vec<syn::ItemStruct> {
    let event_content =
        find_top_level_tag(source_content, "events").unwrap_or("");
    let block: syn::File = syn::parse_str(event_content)
        .expect("<events> to be valid top level");

    block
        .items
        .into_iter()
        .map(|item| match item {
            syn::Item::Struct(struct_) => struct_,
            _ => panic!("<events> to only contain structs"),
        })
        .collect()
}

fn compile_events(
    events: Vec<syn::ItemStruct>,
    generics_impl: &proc_macro2::TokenStream,
    generics_ty: &proc_macro2::TokenStream,
    generics_where: &proc_macro2::TokenStream,
    phantom: &proc_macro2::TokenStream,
) -> (
    proc_macro2::TokenStream,
    proc_macro2::TokenStream,
    proc_macro2::TokenStream,
) {
    let (events, wrappers, wrapper_types): (Vec<_>, Vec<_>, Vec<_>) = itertools::multiunzip(events
        .into_iter()
        .map(|item| {
            let (used_generics, _, _) = item.generics.split_for_impl();
            let ident = &item.ident;
            let ident_string = ident.to_string();
            let main = quote!(
                #[derive(
                    ::fluent_web_client::internal::serde::Serialize,
                    ::fluent_web_client::internal::serde::Deserialize
                )]
                #[serde(crate="::fluent_web_client::internal::serde")]
                #item
                impl #generics_impl __Fluent_Event #generics_ty for #ident #used_generics #generics_where {
                    const NAME: &'static str = #ident_string;
                    type Wrapper = __Fluent_Events::#ident #generics_ty;
                    fn wrap(self) -> Self::Wrapper {
                        __Fluent_Events::#ident(
                            self,
                            ::std::marker::PhantomData,
                        )
                    }
                }
            );
            let wrapper = quote!(
                #[derive(
                    ::fluent_web_client::internal::serde::Serialize,
                    ::fluent_web_client::internal::serde::Deserialize
                )]
                #[serde(crate="::fluent_web_client::internal::serde")]
                pub struct #ident #generics_ty (pub super::#ident #used_generics, pub #phantom);
                impl #generics_ty #ident #generics_ty #generics_where {
                    pub type Parent = super::#ident #used_generics;
                }
            );
            let event_name = quote::format_ident!("__Fluent_Event_{}", ident);
            let wrapper_type = quote!(pub type #event_name = __Fluent_Events::#ident #generics_ty;);
            (main, wrapper, wrapper_type)
        }));

    (
        quote!(#(#events)*),
        quote!(#(#wrappers)*),
        quote!(#(#wrapper_types)*),
    )
}

struct ReactiveAttributeInfo {
    target_element: String,
    target_attribute: String,
    serialize: bool,
    code: syn::Expr,
}

fn find_and_transform_reactive_attributes(
    node: kuchikiki::NodeRef,
) -> Vec<ReactiveAttributeInfo> {
    use kuchikiki::NodeData;
    match node.data() {
        NodeData::Element(data) => {
            let mut attributes = data.attributes.borrow_mut();
            let computed_attributes = attributes
                .map
                .iter()
                .filter_map(|(name, value)| {
                    if name.local.starts_with('=') {
                        Some((
                            name.local.to_string(),
                            value.value.clone(),
                            false,
                        ))
                    } else if name.local.starts_with('@') {
                        Some((
                            name.local.to_string(),
                            value.value.clone(),
                            true,
                        ))
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>();

            let mut nodes = if computed_attributes.is_empty() {
                vec![]
            } else {
                let id = uuid();
                add_class(&mut attributes, &id);

                computed_attributes
                    .into_iter()
                    .map(|(name, code, serialize)| {
                        attributes.remove(name.clone());

                        let code = syn::parse_str(&code)
                            .expect("Valid expression");
                        let name = name
                            .strip_prefix(if serialize {
                                '@'
                            } else {
                                '='
                            })
                            .expect("Prefix to be present")
                            .to_string();

                        ReactiveAttributeInfo {
                            target_element: id.clone(),
                            target_attribute: name,
                            serialize,
                            code,
                        }
                    })
                    .collect::<Vec<_>>()
            };

            nodes
                .extend(node.children().flat_map(
                    find_and_transform_reactive_attributes,
                ));

            nodes
        }
        _ => vec![],
    }
}

fn compile_reactive_attribute(
    attributes: ReactiveAttributeInfo,
    unpack_data: &proc_macro2::TokenStream,
) -> (proc_macro2::TokenStream, proc_macro2::TokenStream) {
    let ReactiveAttributeInfo {
        target_element,
        target_attribute,
        serialize,
        code,
    } = attributes;

    let function_name =
        quote::format_ident!("update_attribute_{}", uuid());

    let selector = format!(".{}", target_element);

    let attribute_value = if serialize {
        quote!({
            let __Fluent_Value = #code;
            let __Fluent_Bytes = ::fluent_web_client::internal::bincode::serialize(&__Fluent_Value).unwrap();
            use ::fluent_web_client::internal::base64::engine::Engine;
            &::fluent_web_client::internal::base64::engine::general_purpose::STANDARD_NO_PAD.encode(__Fluent_Bytes)
        })
    } else {
        quote!(&::std::format!("{}", #code))
    };

    let function_def = quote! {
        fn #function_name(&self) {
            #unpack_data

            let __Fluent_Elements = ::fluent_web_client::internal::get_elements(&self.root_name, #selector);
            for __Fluent_Element in __Fluent_Elements.into_iter() {
                __Fluent_Element.set_attribute(#target_attribute, #attribute_value).unwrap();
            }

            self.detect_reads(Component::#function_name);
        }
    };
    let call = quote!(self.#function_name(););

    (function_def, call)
}

struct ReactiveCssVar {
    id: String,
    var: String,
    format_string: String,
    expressions: Vec<syn::Expr>,
}

fn find_and_remove_css_vars(
    node: kuchikiki::NodeRef,
) -> Vec<ReactiveCssVar> {
    use kuchikiki::NodeData;
    match node.data() {
        NodeData::Element(data) => {
            let mut attributes = data.attributes.borrow_mut();
            let vars = attributes
                .map
                .iter()
                .filter_map(|(name, value)| {
                    if name.local.starts_with("--") {
                        Some((
                            name.local.to_string(),
                            value.value.clone(),
                        ))
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>();

            let mut result = if vars.is_empty() {
                vec![]
            } else {
                let id = uuid();
                add_class(&mut attributes, &id);

                vars.into_iter()
                    .map(|(name, value)| {
                        attributes.remove(name.clone());

                        let (text, expressions) =
                            extract_format_strings(&value);
                        ReactiveCssVar {
                            id: id.clone(),
                            var: name.clone(),
                            format_string: text,
                            expressions,
                        }
                    })
                    .collect()
            };

            result.extend(
                node.children().flat_map(find_and_remove_css_vars),
            );

            result
        }
        _ => vec![],
    }
}

fn compile_css_vars(
    var: ReactiveCssVar,
    unpack: &proc_macro2::TokenStream,
) -> (proc_macro2::TokenStream, proc_macro2::TokenStream) {
    let ReactiveCssVar {
        id,
        var,
        format_string,
        expressions,
    } = var;

    let function_name = quote::format_ident!("update_css_{}", uuid());
    let selector = format!(".{}", id);

    let def = quote!(
        fn #function_name(&self) {
            #unpack

            let __Fluent_Elements = ::fluent_web_client::internal::get_elements(&self.root_name, #selector);
            for __Fluent_Element in __Fluent_Elements.into_iter() {
                let __Fluent_Value = ::std::format!(#format_string, #(#expressions),*);
                use fluent_web_client::internal::wasm_bindgen::JsCast;
                let __Fluent_Element = __Fluent_Element.dyn_into::<::fluent_web_client::internal::web_sys::HtmlElement>().unwrap();
                __Fluent_Element.style().set_property(#var, &__Fluent_Value).unwrap();
            }

            self.detect_reads(Component::#function_name);
        }
    );
    let call = quote!(self.#function_name(););
    (def, call)
}

struct CssTransformer {
    replacement_string: String,
}

impl CssTransformer {
    fn new() -> Self {
        let uuid = uuid();
        CssTransformer {
            replacement_string: uuid,
        }
    }
}

impl<'i> lightningcss::visitor::Visitor<'i> for CssTransformer {
    type Error = Infallible;

    fn visit_types(&self) -> lightningcss::visitor::VisitTypes {
        lightningcss::visit_types!(SELECTORS)
    }

    fn visit_selector(
        &mut self,
        selector: &mut lightningcss::selector::Selector<'i>,
    ) -> Result<(), Self::Error> {
        let mut segements = Vec::new();
        let mut components_iterator = selector.iter();

        let mut combiner = None;
        loop {
            segements.push((
                components_iterator
                    .borrow_mut()
                    .map(Clone::clone)
                    .collect::<Vec<_>>(),
                combiner,
            ));
            combiner = components_iterator.next_sequence();

            if combiner.is_none() {
                break;
            }
        }

        let where_clause =
            lightningcss::selector::Component::Where(Box::new([
                lightningcss::selector::Selector::from(vec![
                    lightningcss::selector::Component::ID(
                        self.replacement_string.clone().into(),
                    ),
                    lightningcss::selector::Component::Combinator(lightningcss::selector::Combinator::Descendant),
                    lightningcss::selector::Component::ExplicitUniversalType,
                    lightningcss::selector::Component::Negation(Box::new([
                        lightningcss::selector::Selector::from(vec![
                            lightningcss::selector::Component::ID(
                                self.replacement_string.clone().into(),
                            ),
                            lightningcss::selector::Component::Combinator(lightningcss::selector::Combinator::Descendant),
                            lightningcss::selector::Component::Class(
                                "__Fluent_Component".into(),
                            ),
                            lightningcss::selector::Component::Combinator(lightningcss::selector::Combinator::Descendant),
                            lightningcss::selector::Component::ExplicitUniversalType,
                            ]),
                        ])
                    )
                ]),
            ]));

        segements[0].0.push(where_clause.clone());

        if segements.len() > 1 {
            segements
                .last_mut()
                .expect("Vector containg >1 elements not to be empty")
                .0
                .push(where_clause);
        }

        let segements = segements
            .into_iter()
            .flat_map(|(mut components, combinator)| {
                if let Some(comb) = combinator {
                    components.insert(
                        0,
                        lightningcss::selector::Component::Combinator(
                            comb,
                        ),
                    );
                    components
                } else {
                    components
                }
            })
            .collect::<Vec<_>>();
        let new_selector: lightningcss::selector::Selector =
            segements.into();
        *selector = new_selector;

        Ok(())
    }
}

/// Transforms the css by adding the returned string as a placeholder for the rootname
/// This scopes the css to the specific component using the same selector as the fluent_web_client
fn transform_css(
    css: &mut lightningcss::stylesheet::StyleSheet,
) -> String {
    let mut trans = CssTransformer::new();
    css.visit(&mut trans).unwrap();
    trans.replacement_string
}

/// Compiler a fluent file to a rust file, this is the main block of code
fn compile_fluent_file(
    source: PathBuf,
    dst: PathBuf,
) -> anyhow::Result<()> {
    let source_content = fs::read_to_string(source)?;

    let Generics {
        impl_generics,
        ty_generics,
        where_clauses,
        generic_def,
        phantom,
    } = parse_generics(&source_content);

    let prop_statements = parse_data_and_props_segement(
        find_top_level_tag(&source_content, "props").unwrap_or(""),
        true,
    );
    let PropInfo {
        watcher: prop_watcher,
        update: update_props,
    } = compile_props(&prop_statements);

    let data_content =
        find_top_level_tag(&source_content, "data").unwrap_or("");
    let mut data_statements =
        parse_data_and_props_segement(data_content, false);

    data_statements.extend(prop_statements);
    let DataSectionInfo {
        struct_fields: data_struct_fields,
        create: data_create_fields,
        import_struct_to_local_scope: unpack_data,
        unpack_mut,
        unpack_change_detector,
        reactive_updates,
        targets: data_targets,
    } = compile_data_section(&data_statements, &ty_generics);

    let (events, wrappers, wrappers_type) = compile_events(
        parse_events(&source_content),
        &impl_generics,
        &ty_generics,
        &where_clauses,
        &phantom,
    );

    let define_content =
        find_top_level_tag(&source_content, "define").unwrap_or("");
    let define_parsed: syn::File = syn::parse_str(define_content)?;

    let body_html = get_html_body(&source_content);

    let reactive_element_info =
        find_reactive_nodes_and_replace_with_span(body_html.clone());
    let (reactive_update_defs, reactive_update_calls): (
        Vec<_>,
        Vec<_>,
    ) = reactive_element_info
        .iter()
        .map(|info| {
            create_reactive_update_function(info, &unpack_data)
        })
        .unzip();

    let event_info =
        find_event_listeners_and_set_class(body_html.clone());
    let (event_set_defs, event_set_calls): (Vec<_>, Vec<_>) =
        event_info
            .iter()
            .map(|event| compile_event_listener(event, &unpack_mut))
            .unzip();

    let subcomponent_info =
        find_sub_components_and_replace_with_div(body_html.clone());
    let (subcomponent_defs, subcomponent_calls): (Vec<_>, Vec<_>) =
        subcomponent_info
            .iter()
            .map(create_spawn_and_spawn_call_for_subcomponent)
            .unzip();

    let conditional_info =
        find_conditional_attributes_and_set_id(body_html.clone());
    let (conditional_defs, conditional_calls): (Vec<_>, Vec<_>) =
        conditional_info
            .iter()
            .map(|info| compile_conditional_stmt(info, &unpack_data))
            .unzip();

    let (reactive_attribute_defs, reactive_attribute_calls): (
        Vec<_>,
        Vec<_>,
    ) = find_and_transform_reactive_attributes(body_html.clone())
        .into_iter()
        .map(|info| compile_reactive_attribute(info, &unpack_data))
        .unzip();

    let (css_defs, css_calls): (Vec<_>, Vec<_>) =
        find_and_remove_css_vars(body_html.clone())
            .into_iter()
            .map(|info| compile_css_vars(info, &unpack_data))
            .unzip();

    let mut html_content = Vec::new();
    html5ever::serialize(
        &mut html_content,
        &body_html,
        Default::default(),
    )
    .expect("<template> body to be valid html");
    let mut html_content = String::from_utf8(html_content)
        .expect("<template> to be valid utf8");

    let css_raw =
        find_top_level_tag(&source_content, "style").unwrap_or("");
    let mut css = lightningcss::stylesheet::StyleSheet::parse(
        css_raw,
        lightningcss::stylesheet::ParserOptions::default(),
    )
    .expect("<style> tag to be valid css");

    let replace_string = transform_css(&mut css);

    let css_content = css
        .to_css(lightningcss::stylesheet::PrinterOptions {
            minify: true,
            ..Default::default()
        })
        .expect("To be able to minify css")
        .code;

    let css_content = css_content
        .replace('{', "{{")
        .replace('}', "}}")
        .replace(&replace_string, "{root}");

    html_content += &format!("<style>{css_content}</style>");

    let read_updates = data_targets.iter().map(|target| {
        quote!(
            if #target.was_read() {__Fluent_Updates.#target.insert(f);}
            #target.clear();
        )
    }).collect::<Vec<_>>();
    let write_updates = data_targets.iter().map(|target| {
        quote!(
            if #target.was_written() {__Fluent_Functions.extend(__Fluent_Updates.#target.iter());}
            #target.clear();
        )
    }).collect::<Vec<_>>();

    let component_source: syn::File = syn::parse_quote!(
        #![allow(warnings)]
        use ::fluent_web_client::internal::web_sys::*;
        use ::fluent_web_client::internal::DomDisplay;
        use ::fluent_web_client::internal::UseInEvent;

        #define_parsed

        #[derive(::fluent_web_client::internal::Derivative)]
        #[derivative(Clone(bound = ""))]
        struct __Fluid_Data #ty_generics {
            #(#data_struct_fields)*
            _p: #phantom
        }

        #[derive(::fluent_web_client::internal::Derivative)]
        #[derivative(Default(bound = ""))]
        struct __Fluid_Reactive_Functions #impl_generics #where_clauses {
            #(#reactive_updates)*
            _p: #phantom
        }

        #[derive(::fluent_web_client::internal::Derivative)]
        #[derivative(Clone(bound = ""))]
        pub struct Component #generic_def {
            root_name: ::std::string::String,
            data: __Fluid_Data #ty_generics,
            updates: ::std::rc::Rc<::std::cell::RefCell<__Fluid_Reactive_Functions #ty_generics>>,
        }

        trait __Fluent_Event #ty_generics : ::fluent_web_client::internal::serde::Serialize + for<'a> ::fluent_web_client::internal::serde::Deserialize<'a> {
            const NAME: &'static str;
            type Wrapper : ::fluent_web_client::internal::serde::Serialize + for<'a> ::fluent_web_client::internal::serde::Deserialize<'a>;
            fn wrap(self) -> Self::Wrapper;
        }

        #events

        pub mod __Fluent_Events {
            #wrappers
        }

        impl #impl_generics Component #ty_generics #where_clauses {
            #wrappers_type

            #(#subcomponent_defs)*
            #(#reactive_update_defs)*
            #(#conditional_defs)*
            #(#reactive_attribute_defs)*
            #(#css_defs)*
            #(#event_set_defs)*

            fn detect_reads(&self, f: fn(&Component #ty_generics)) {
                let mut __Fluent_Updates = self.updates.borrow_mut();
                #unpack_change_detector
                #(#read_updates)*
            }

            fn update_changed_values(&self) {
                let mut __Fluent_Updates = self.updates.borrow_mut();
                #unpack_change_detector

                let mut __Fluent_Functions: ::std::collections::HashSet<fn(&Component #ty_generics)> = ::std::collections::HashSet::new();
                #(#write_updates)*

                ::std::mem::drop(__Fluent_Updates);

                for func in __Fluent_Functions.into_iter() {
                    func(self);
                }
            }

            #prop_watcher
            #update_props

            // It should be fine with mutliple borrows as parent components will be the ones borrowing.
            fn emit<__Fluent_E: __Fluent_Event #ty_generics>(&self, event: __Fluent_E) {
                use ::fluent_web_client::internal::web_sys;

                let root_element = ::fluent_web_client::internal::get_by_id(&self.root_name);
                let data = ::fluent_web_client::internal::bincode::serialize(&event.wrap()).unwrap();
                let data = ::fluent_web_client::internal::js_sys::Uint8Array::from(data.as_slice());
                let event = web_sys::CustomEvent::new_with_event_init_dict(
                    __Fluent_E::NAME,
                    &web_sys::CustomEventInit::new().detail(&data)
                ).unwrap();
                root_element.dispatch_event(&event).unwrap();
            }
        }

        impl #impl_generics ::fluent_web_client::internal::Component for Component #ty_generics #where_clauses {
            fn render_init(&self) -> ::std::string::String {
                let root = &self.root_name;
                ::std::format!(#html_content)
            }

            fn create(root_id: String) -> Self {
                Self {
                    root_name: root_id,
                    data: __Fluid_Data {
                        #(#data_create_fields)*
                        _p: std::marker::PhantomData,
                    },
                    updates: ::std::rc::Rc::new(::std::cell::RefCell::new(__Fluid_Reactive_Functions::default())),
                }
            }

            fn setup_events(&self) {
                #(#event_set_calls)*
                self.update_props();
                self.setup_watcher();
            }

            fn update_all(&self) {
                #(#reactive_update_calls)*
                #(#conditional_calls)*
                #(#reactive_attribute_calls)*
                #(#css_calls)*
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
    fn test_find_top_level_tag() {
        const CONTENT: &str = "<a>Hello World</a> <b>123</b>";

        assert_eq!(
            "Hello World",
            find_top_level_tag(CONTENT, "a")
                .expect("<a> to be in test content")
        );
        assert_eq!(
            "123",
            find_top_level_tag(CONTENT, "b")
                .expect("<b> to be in test content")
        )
    }
}
