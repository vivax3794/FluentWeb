//! <generics> tag

use super::utils::find_top_level_tag;
use crate::prelude::*;

/// Generic component fields
pub struct Generics {
    /// Tokens that should go after impl
    pub impl_generics: proc_macro2::TokenStream,
    /// Tokens that go after a ident
    pub ty_generics: proc_macro2::TokenStream,
    /// The where clauses
    pub where_clauses: proc_macro2::TokenStream,
    /// The tokens used when defining a struct
    pub generic_def: proc_macro2::TokenStream,
    /// A `PhantomData` field with all generics
    pub phantom: proc_macro2::TokenStream,
}

/// Parse `<generics>`
pub fn parse(source_content: &str) -> CompilerResult<Generics> {
    let generic_content = find_top_level_tag(source_content, "generic");

    match generic_content {
        None => Ok(Generics {
            impl_generics: quote!(),
            ty_generics: quote!(),
            where_clauses: quote!(),
            generic_def: quote!(),
            phantom: quote!(::std::marker::PhantomData<()>),
        }),
        Some(generic_content) => {
            let fake_item: syn::ItemStruct =
                syn::parse_str(&format!("struct Fake{generic_content};"))?;

            let generics = fake_item.generics;

            let (impl_generic, ty_generics, where_clauses) = generics.split_for_impl();

            let phantom_args = generics
                .params
                .iter()
                .map(|param| match param {
                    syn::GenericParam::Lifetime(syn::LifetimeParam { lifetime, .. }) => {
                        quote!(#lifetime)
                    }
                    syn::GenericParam::Type(syn::TypeParam { ident, .. })
                    | syn::GenericParam::Const(syn::ConstParam { ident, .. }) => quote!(#ident),
                })
                .collect::<Vec<_>>();

            Ok(Generics {
                impl_generics: quote!(#impl_generic),
                ty_generics: quote!(#ty_generics),
                where_clauses: quote!(#where_clauses),
                generic_def: quote!(#generics #where_clauses),
                phantom: quote!(::std::marker::PhantomData<(#(#phantom_args),*)>),
            })
        }
    }
}
