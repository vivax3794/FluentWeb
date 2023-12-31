//! reactive template syntax

use super::utils::{extract_format_strings, uuid};
use super::DefCallPair;
use crate::compiler::utils::visit_html_nodes;
use crate::prelude::*;

/// Represents the reactive text in a <span>
struct ReactiveText {
    /// Id of the span element, used to update it later
    id: String,
    /// The format text
    text: String,
    /// A list of expressions for each {} in the tag.
    expressions: Vec<syn::Expr>,
}

/// Create the update functions for reactive <spans>
fn compile_stmt(
    reactive_text: &ReactiveText,
    data: &super::data_and_props::Unwraps,
) -> DefCallPair {
    let function_name = quote::format_ident!("update_element_{}", reactive_text.id);

    let selector = format!(".{}", reactive_text.id);
    let def = quote! {
        fn #function_name(&mut self, __Fluent_S: Option<&str>) {
            #{&data.unpack_ref}

            let __Fluent_Elements = ::fluent_web_runtime::internal::get_elements(&self.root_name, #selector, __Fluent_S);
            for __Fluent_Element in __Fluent_Elements.into_iter() {
                let __Fluent_Text = &::std::format!(
                    #{&reactive_text.text},
                    #(for expr in &reactive_text.expressions) , {
                        ::fluent_web_runtime::internal::display(&(#expr))
                    }
                );
                __Fluent_Element.set_text_content(::std::option::Option::Some(__Fluent_Text));
            }

            self.detect_reads(Component::#function_name);
        }
    };
    let call = quote!(self.#function_name(root.clone()););

    DefCallPair { def, call }
}

/// Find all text with {} and replace the text with a <span> that can be targeted by code.
fn modify_html(node: &kuchikiki::NodeRef) -> CompilerResult<Vec<ReactiveText>> {
    use kuchikiki::NodeData;
    use markup5ever::namespace_url;

    visit_html_nodes(node, |node: &kuchikiki::NodeRef| match node.data() {
        NodeData::Text(text) => {
            let text = text.borrow();
            let (format_string, expressions) = match extract_format_strings(&text) {
                Ok(val) => val,
                Err(err) => return vec![Err(err)],
            };

            if expressions.is_empty() {
                return vec![];
            }

            let id = uuid();

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

            vec![Ok(ReactiveText {
                id,
                text: format_string,
                expressions,
            })]
        }
        _ => vec![],
    })
    .into_iter()
    .collect::<CompilerResult<Vec<_>>>()
}

/// Modify html and create update functions
pub fn compile(
    html: &kuchikiki::NodeRef,
    data: &super::data_and_props::Unwraps,
) -> CompilerResult<Vec<DefCallPair>> {
    let nodes = modify_html(html)?;
    Ok(nodes
        .into_iter()
        .map(|node| compile_stmt(&node, data))
        .collect())
}
