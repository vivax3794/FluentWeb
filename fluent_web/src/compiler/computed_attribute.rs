//! =abc and @abd

// TODO: Combine `modify_html` functions

use super::utils::{modify_html_code, uuid, ModifiedHtmlInfoWithCode};
use super::DefCallPair;
use crate::prelude::*;

/// Attribute formatting for normal computed attributes
fn normal_attribute_value(code: &syn::Expr) -> proc_macro2::TokenStream {
    quote!(&::std::format!("{}", #code))
}

/// Serialize attribute such that it can be read by sub component
fn prop_attribute_value(code: &syn::Expr) -> proc_macro2::TokenStream {
    quote!(
        {
        let __Fluent_Value = #code;
        let __Fluent_Bytes = ::fluent_web_runtime::internal
            ::bincode::serialize(&__Fluent_Value).unwrap();
        use ::fluent_web_runtime::internal::base64::engine::Engine;
        &::fluent_web_runtime::internal
            ::base64::engine::general_purpose
            ::STANDARD_NO_PAD.encode(__Fluent_Bytes)
        }
    )
}

/// Compile attribute update function
fn compile_stmt(
    attribute: &ModifiedHtmlInfoWithCode<syn::Expr>,
    attribute_value: &proc_macro2::TokenStream,
    data: &super::data_and_props::Unwraps,
) -> DefCallPair {
    let function_name = quote::format_ident!("update_attribute_{}", uuid());

    let selector = format!(".{}", attribute.id);
    let function_def = quote! {
        fn #function_name(&mut self, __Fluent_S: Option<&str>) {
            #{&data.unpack_ref}

            let __Fluent_Elements = ::fluent_web_runtime::internal::get_elements(&self.root_name, #selector, __Fluent_S);
            for __Fluent_Element in __Fluent_Elements.into_iter() {
                __Fluent_Element.set_attribute(#{&attribute.attribute}, #attribute_value).unwrap();
            }

            self.detect_reads(Component::#function_name);
        }
    };
    let call = quote!(self.#function_name(root.clone()););

    DefCallPair {
        def: function_def,
        call,
    }
}

/// Compiler normal and prop computed attributes
pub fn compile(
    html: kuchikiki::NodeRef,
    data: &super::data_and_props::Unwraps,
) -> CompilerResult<Vec<DefCallPair>> {
    let normal_nodes = modify_html_code(html.clone(), "=")?;
    let prop_nodes = modify_html_code(html, "@")?;

    let normal = normal_nodes
        .into_iter()
        .map(|node| compile_stmt(&node, &normal_attribute_value(&node.code), data));
    let prop = prop_nodes
        .into_iter()
        .map(|node| compile_stmt(&node, &prop_attribute_value(&node.code), data));

    Ok(normal.chain(prop).collect())
}
