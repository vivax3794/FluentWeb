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

/// Info about the data fields to be used in other generation calls
pub struct Unwraps {
    /// Unpack and borrow all data fields (this assumes `&self`)
    pub unpack_ref: proc_macro2::TokenStream,
    /// Unpack and borrow mutable references to all data fields, (this assumes `&mut self`)
    pub unpack_mut: proc_macro2::TokenStream,
    /// Unpack just the raw change dectors
    pub unpack_change_detector: proc_macro2::TokenStream,
    /// Data names
    pub targets: Vec<syn::Ident>,
}

/// Create all the code gen for the data fields
pub fn compile_unwraps(data_statements: &[DataStatement]) -> Unwraps {
    let unpack_change_detector = quote!(
        let __Fluid_Data { #(#{&#data_statements.target},)* .. } = self.data.clone();
    );

    let unpack_ref = quote!(
        #unpack_change_detector
        #(for field in data_statements) {
            let #{&field.target} = #{&field.target}.borrow();
        }
    );

    let unpack_mut = quote!(
        #unpack_change_detector
        #(for field in data_statements) {
            #(if field.is_prop) {
                let #{&field.target} = #{&field.target}.borrow();
            }
            #(else) {
                let mut #{&field.target} = #{&field.target}.borrow_mut();
            }
        }
    );

    Unwraps {
        unpack_ref,
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

/// Compile create function for component
pub fn compile_create(
    data: &[DataStatement],
) -> proc_macro2::TokenStream {
    quote!(
        fn create(root_id: String) -> Self {
            Self {
                root_name: root_id,
                data: __Fluid_Data {
                    #(for field in data) {
                        #{&field.target}: ::fluent_web_client::internal::ChangeDetector::new(#{&field.init_value}),
                    }
                    _p: std::marker::PhantomData,
                },
                updates: ::std::rc::Rc::new(::std::cell::RefCell::new(__Fluid_Reactive_Functions::default())),
            }
        }
    )
}

/// Compile reactive callback function storing dict
pub fn compile_reactive_function_struct(
    data: &[DataStatement],
    generics: &super::Generics,
) -> proc_macro2::TokenStream {
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

/// Compiles the `detect_reads` function which updates the hashset with the correct functions
pub fn compile_detect_reads(
    data: &[DataStatement],
    generics: &super::Generics,
    unwraps: &Unwraps,
) -> proc_macro2::TokenStream {
    quote!(
        fn detect_reads(&self, f: fn(&Component #{&generics.ty_generics}, Option<String>)) {
            let mut __Fluent_Updates = self.updates.borrow_mut();
            #{&unwraps.unpack_change_detector}
            #(for field in data) {
                if #{&field.target}.was_read() {
                    __Fluent_Updates.#{&field.target}.insert(f);
                    #{&field.target}.clear();
                }
            }
        }
    )
}

/// Compiled the `update_changed_values` function which will call all functions depending on the changed values.
pub fn compile_update_changed_values(
    data: &[DataStatement],
    generics: &super::Generics,
    unwraps: &Unwraps,
) -> proc_macro2::TokenStream {
    // let write_updates = data.targets.iter().map(|target| {
    //     quote!(
    //         if #target.was_written() {__Fluent_Functions.extend(__Fluent_Updates.#target.iter());}
    //         #target.clear();
    //     )
    // }).collect::<Vec<_>>();

    quote!(
        fn update_changed_values(&self) {
            let mut __Fluent_Updates = self.updates.borrow_mut();
            #{&unwraps.unpack_change_detector}

            let mut __Fluent_Functions: ::std::collections::HashSet<fn(&Component #{&generics.ty_generics}, Option<String>)> = ::std::collections::HashSet::new();

            #(for field in data) {
                if #{&field.target}.was_written() {__Fluent_Functions.extend(__Fluent_Updates.#{&field.target}.iter());}
                #{&field.target}.clear();
            }

            ::std::mem::drop(__Fluent_Updates);
            for func in __Fluent_Functions.into_iter() {
                func(self, None);
            }
        }
    )
}
