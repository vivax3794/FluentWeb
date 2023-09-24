//! <data> and <props> sections

use crate::prelude::*;

/// A statement in the <data> block
#[derive(Clone)]
pub struct DataStatement {
    /// Name for the property
    pub target: syn::Ident,
    /// The type, this is used in the struct definition
    pub type_: syn::Type,
    /// Expression to set inital value, this is used in the ::new() method
    pub init_value: syn::Expr,
    /// Is this in <data> or <props>
    pub is_prop: bool,
}

/// Info about the data fields to be used in other generation calls
pub struct DataSectionInfo {
    /// Create instance of the struct
    pub create: Vec<proc_macro2::TokenStream>,
    /// Unpack and borrow all data fields (this assumes `&self`)
    pub unpack_ref: proc_macro2::TokenStream,
    /// Unpack and borrow mutable references to all data fields, (this assumes `&mut self`)
    pub unpack_mut: proc_macro2::TokenStream,
    /// Unpack just the raw change dectors
    pub unpack_change_detector: proc_macro2::TokenStream,
    /// Data names
    pub targets: Vec<syn::Ident>,
}

/// Parse the data block
pub fn parse_data_and_props_segement(
    data_section: &str,
    is_prop: bool,
) -> CompilerResult<Vec<DataStatement>> {
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
            }) => Ok(DataStatement {
                target,
                type_,
                init_value: expr,
                is_prop,
            }),
            _ => Err(Compiler::WrongSyntax(
                "expected let mut NAME: TYPE = VALUE;".to_owned(),
            )),
        })
        .collect()
}

/// Create all the code gen for the data fields
pub fn compile_data_section(
    data_statements: &[DataStatement],
) -> DataSectionInfo {
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

    DataSectionInfo {
        create: create_statements,
        unpack_ref: field_unpacking,
        unpack_mut,
        unpack_change_detector,
        targets: data_statements
            .iter()
            .map(|stmt| stmt.target.clone())
            .collect(),
    }
}

/// Compile definition for data struct
pub fn compile_data_struct(
    data: &[DataStatement],
    generics: &super::Generics,
) -> proc_macro2::TokenStream {
    quote!(
        #[derive(::fluent_web_client::internal::Derivative)]
        #[derivative(Clone(bound = ""))]
        struct __Fluid_Data #{&generics.generic_def} {
            #(for field in data) {
                #{&field.target}: ::fluent_web_client::internal::ChangeDetector<#{&field.type_}>,
            }
            _p: #{&generics.phantom}
        }
    )
}

/// Compile reactive callback function storing dict
pub fn compile_reactive_function_struct(
    data: &[DataStatement],
    generics: &super::Generics,
) -> proc_macro2::TokenStream {
    // let reactive_fields = data_statements
    //     .iter()
    //     .map(|stmt| {
    //         let target = &stmt.target;
    //         quote!(#target: ::std::collections::HashSet<fn(&Component #{&generics.ty_generics}, Option<String>)>,)
    //     })
    //     .collect::<Vec<_>>();
    quote!(
        #[derive(::fluent_web_client::internal::Derivative)]
        #[derivative(Default(bound = ""))]
        struct __Fluid_Reactive_Functions #{&generics.impl_generics} #{&generics.where_clauses} {
           #(for field in data) {
                #{&field.target}: ::std::collections::HashSet<fn(&Component #{&generics.ty_generics}, Option<String>)>,
           }
            _p: #{&generics.phantom}
        }
    )
}

/// Compile prop watcher
pub fn compile_setup_watcher() -> proc_macro2::TokenStream {
    quote!(
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
    )
}

/// Compile `update_props` function which will read all the prop values
pub fn compile_update_props(
    props: &[DataStatement],
) -> proc_macro2::TokenStream {
    quote!(
        fn update_props(&self) {
            let element = ::fluent_web_client::internal::get_by_id(
                &self.root_name,
            );
            #(for prop in props) {
                if let Some(value) = element.get_attribute(#{prop.target.to_string()}) {
                    use ::fluent_web_client::internal::base64::engine::Engine;
                    let decoded = ::fluent_web_client::internal::base64::engine::general_purpose::STANDARD_NO_PAD.decode(value).unwrap();
                    let deserialized = ::fluent_web_client::internal::bincode::deserialize(&decoded).unwrap();
                    * self.data.#{&prop.target}.borrow_mut() = deserialized;
                }
            }
            self.update_changed_values();
        }
    )
}
