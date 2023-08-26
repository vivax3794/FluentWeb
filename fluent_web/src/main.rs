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

fn compile_fluent_file(source: PathBuf, dst: PathBuf) -> anyhow::Result<()> {
    let source_content = fs::read_to_string(source)?;

    let data_content = find_top_level_tag(&source_content, "data").unwrap_or("");

    struct Statement {
        target: syn::Ident,
        type_: syn::Type,
        init_value: syn::Expr,
    }
    let data_block_parsed: syn::Block = syn::parse_str(&format!("{{{data_content}}}")).unwrap();
    let data_statements: Vec<Statement> = data_block_parsed
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
            }) => Statement {
                target,
                type_,
                init_value: expr,
            },
            _ => panic!("expected let mut NAME: TYPE = VALUE;"),
        })
        .collect();

    let struct_fields: Vec<_> = data_statements
        .iter()
        .map(|stmt| {
            let name = &stmt.target;
            let type_ = &stmt.type_;
            quote!(#name: #type_)
        })
        .collect();
    let init_bindings: Vec<_> = data_statements
        .iter()
        .map(|stmt| {
            let name = &stmt.target;
            let expr = &stmt.init_value;
            quote!(#name: #expr)
        })
        .collect();

    let define_content = find_top_level_tag(&source_content, "define").unwrap_or("");
    let define_parsed: syn::File = syn::parse_str(define_content)?;

    let template_content = find_top_level_tag(&source_content, "template").unwrap_or("");
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

    let body_html = parsed_html
        .select("body")
        .unwrap()
        .next()
        .unwrap()
        .as_node()
        .clone();

    let reactive = find_reactive_nodes(body_html.clone());

    let mut html_content = Vec::with_capacity(template_content.len());
    html5ever::serialize(&mut html_content, &body_html, Default::default()).unwrap();
    let html_content = String::from_utf8(html_content).unwrap();

    let field_getters = data_statements
        .iter()
        .map(|stmt| &stmt.target)
        .collect::<Vec<_>>();
    let field_unpacking = quote!(let __Fluid_Data { #(#field_getters),* } = &self.data);

    let element_update_functions = reactive
        .iter()
        .map(|value| {
            let id = &value.id;
            let format_text = &value.text;
            let expressions = &value.expressions;

            let function_name = quote::format_ident!("update_element_{}", id);

            quote! {
                fn #function_name(&self) {
                    #field_unpacking;

                    let __Fluent_Selector = ::std::format!("#{} #{}:not(#{} .__Fluent_Component *)", self.root_name, #id, self.root_name);

                    let __Fluent_Window = ::fluent_web_client::internal::web_sys::window().unwrap();
                    let __Fluent_Document = __Fluent_Window.document().unwrap();

                    let __Fluent_Element = __Fluent_Document.query_selector(&__Fluent_Selector).unwrap().unwrap();
                    __Fluent_Element.set_text_content(::std::option::Option::Some(&::std::format!(#format_text, #(#expressions),*)));
                }
            }
        })
        .collect::<Vec<_>>();
    let update_function_calls = reactive
        .iter()
        .map(|element| {
            let id = &element.id;
            let function_name = quote::format_ident!("update_element_{}", id);
            quote!(self.#function_name();)
        })
        .collect::<Vec<_>>();

    let component_source: syn::File = syn::parse_quote!(
        #define_parsed

        struct __Fluid_Data {
            #(#struct_fields),*
        }

        struct __Fluid_Sub_Components {

        }

        pub struct Component {
            root_name: ::std::string::String,
            sub_components: __Fluid_Sub_Components,
            data: __Fluid_Data,
        }

        impl Component {
            #(#element_update_functions)*
        }

        impl ::fluent_web_client::internal::Component for Component {
            fn render_init(&self) -> ::std::string::String {
                #html_content.into()
            }

            fn create(root_id: String) -> Self {
                Self {
                    root_name: root_id,
                    sub_components: __Fluid_Sub_Components {},
                    data: __Fluid_Data {
                        #(#init_bindings),*
                    }
                }
            }

            fn update_all(&self) {
                #(#update_function_calls)*
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

struct ReactiveText {
    id: String,
    text: String,
    expressions: Vec<syn::Expr>,
}

fn find_reactive_nodes(node: kuchikiki::NodeRef) -> Vec<ReactiveText> {
    use kuchikiki::NodeData;
    match node.data() {
        NodeData::Element(_) => node.children().flat_map(find_reactive_nodes).collect(),
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
