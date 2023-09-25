//! Define custom events in <events>

use super::utils::find_top_level_tag;
use crate::error;

use crate::prelude::*;

/// Parse custom events
fn parse_events(
    source_content: &str,
) -> error::CompilerResult<Vec<syn::ItemStruct>> {
    let event_content =
        find_top_level_tag(source_content, "events").unwrap_or("");
    let block: syn::File = syn::parse_str(event_content)
        .expect("<events> to be valid top level");

    block
        .items
        .into_iter()
        .map(|item| match item {
            syn::Item::Struct(struct_) => Ok(struct_),
            _ => Err(error::Compiler::WrongSyntax(
                "<events> to only contain structs".into(),
            )),
        })
        .collect()
}

/// Compile custom events
fn compile_events_internal(
    events: &[syn::ItemStruct],
    generics: &super::Generics,
) -> proc_macro2::TokenStream {
    quote!(
        #(for event in events) {
            #(let (used_generics, _, _) = event.generics.split_for_impl()) {
                #[derive(
                    ::fluent_web_client::internal::serde::Serialize,
                    ::fluent_web_client::internal::serde::Deserialize
                )]
                #[serde(crate="::fluent_web_client::internal::serde")]
                #event
                impl #{&generics.impl_generics} ::fluent_web_client::internal::Event for #{&event.ident} #used_generics {
                    const NAME: &'static str = #{event.ident.to_string()};
                }
            }
        }
        pub mod __Fluent_Events {
            #(for event in events) {
                #(let (used_generics, _, _) = event.generics.split_for_impl()) {
                    #[derive(
                        ::fluent_web_client::internal::serde::Serialize,
                        ::fluent_web_client::internal::serde::Deserialize
                    )]
                    #[serde(crate="::fluent_web_client::internal::serde")]
                    pub struct #{&event.ident} #{&generics.ty_generics} (pub super::#{&event.ident} #used_generics, pub #{&generics.phantom});
                    impl #{&generics.ty_generics} ::fluent_web_client::internal::EventWrapper for #{&event.ident} #{&generics.ty_generics} #{&generics.where_clauses} {
                        type Real = super::#{&event.ident} #used_generics;
                    }
                }
            }
        }
    )
}

/// Compile the <events> section
pub fn compile_events(
    source_content: &str,
    generics: &super::Generics,
) -> error::CompilerResult<proc_macro2::TokenStream> {
    Ok(compile_events_internal(
        &parse_events(source_content)?,
        generics,
    ))
}
