//! Compile ?abc="something" attributes

use crate::prelude::*;

use super::utils::{
    modify_html_code, uuid, ModifiedHtmlInfoWithCode,
};
use super::DefCallPair;

/// Compile a conditional attribute
fn compile_stmt(
    attribute: &ModifiedHtmlInfoWithCode<syn::Expr>,
    data: &super::data_and_props::Unwraps,
) -> DefCallPair {
    let function_name =
        quote::format_ident!("update_attribute_{}", uuid());

    let selector = format!(".{}", attribute.id);
    let update_def = quote!(
        fn #function_name(&self, __Fluent_S: Option<String>) {
            #{&data.unpack_ref}

            let __Fluent_Elements = ::fluent_web_client::internal::get_elements(&self.root_name, #selector, __Fluent_S);
            for __Fluent_Element in __Fluent_Elements.into_iter() {
                if #{&attribute.code} {
                    __Fluent_Element.set_attribute(#{&attribute.attribute}, "").unwrap();
                } else {
                    __Fluent_Element.remove_attribute(#{&attribute.attribute}).unwrap();
                }
            }
            self.detect_reads(Component::#function_name);
        }
    );
    let update_call = quote!(self.#function_name(root.clone()););

    DefCallPair {
        def: update_def,
        call: update_call,
    }
}

/// Compile conditional attribute
pub fn compile(
    html: kuchikiki::NodeRef,
    data: &super::data_and_props::Unwraps,
) -> CompilerResult<Vec<DefCallPair>> {
    let nodes = modify_html_code(html, "?")?;
    Ok(nodes
        .into_iter()
        .map(|node| compile_stmt(&node, data))
        .collect())
}
