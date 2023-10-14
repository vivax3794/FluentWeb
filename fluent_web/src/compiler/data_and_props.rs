//! <data> and <props> sections

use quote::ToTokens;

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
    let data_block_parsed: syn::Block = syn::parse_str(&format!("{{\n{data_section}\n}}"))?;

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
            stmt => Err(Compiler::WrongSyntaxInDataSection {
                src: data_section.to_owned(),
                err_span: procmacro_tokens_to_mietti_span(
                    data_section,
                    stmt.into_token_stream(),
                    1,
                ),
            }),
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
        let __Fluid_Data { #(ref mut #{&#data_statements.target},)* .. } = &mut self.data;
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

/// Create a macro to unpack a ident as the current component
pub fn compile_unwrap_macro(data_statements: &[DataStatement]) -> proc_macro2::TokenStream {
    quote!(
        macro_rules! unpack {
            ($component:expr, $($field:ident),*) => {
                // Drop order is very important
                let mut __Fluid_Comp = $component.clone().upgrade().unwrap();
                let mut __Fluid_Comp = __Fluid_Comp.borrow_mut();
                let __Fluid_Data { $(ref mut $field,)* .. } = __Fluid_Comp.data;
                $(let mut $field = $field.borrow_mut();)*
            };
        }
        macro_rules! update {
            ($component:expr) => {
                // Drop order is very important
                let mut __Fluid_Comp = $component.clone().upgrade().unwrap();
                let mut __Fluid_Comp = __Fluid_Comp.borrow_mut();
                __Fluid_Comp.update_changed_values();
            };
        }
    )
}

/// Compile definition for data struct
pub fn compile_data_struct(
    data: &[DataStatement],
    generics: &super::Generics,
) -> proc_macro2::TokenStream {
    quote!(
        struct __Fluid_Data #{&generics.generic_def} {
            #(for field in data) {
                #{&field.target}: ::fluent_web_runtime::internal::ChangeDetector<#{&field.type_}>,
            }
            _p: #{&generics.phantom}
        }
    )
}

/// Compile create function for component
pub fn compile_create(data: &[DataStatement]) -> proc_macro2::TokenStream {
    quote!(
        fn create(root_id: ::std::boxed::Box<str>) -> Self {
            Self {
                root_name: root_id,
                data: __Fluid_Data {
                    #(for field in data) {
                        #{&field.target}: ::fluent_web_runtime::internal::ChangeDetector::new(#{&field.init_value}),
                    }
                    _p: std::marker::PhantomData,
                },
                updates: __Fluid_Reactive_Functions::default(),
                subs: ::std::collections::HashMap::new(),
                weak: std::option::Option::None,
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
        #[derive(::fluent_web_runtime::internal::Derivative)]
        #[derivative(Default(bound = ""))]
        struct __Fluid_Reactive_Functions #{&generics.impl_generics} #{&generics.where_clauses} {
           #(for field in data) {
                #{&field.target}: ::std::collections::HashSet<(fn(&mut Component #{&generics.ty_generics}, Option<&str>), bool)>,
           }
            _p: #{&generics.phantom}
        }
    )
}

/// Compile `update_props` function which will read all the prop values
pub fn compile_update_props(props: &[DataStatement]) -> proc_macro2::TokenStream {
    quote!(
        fn update_props(&mut self) {
            let element = ::fluent_web_runtime::internal::get_by_id(
                &self.root_name,
            );
            #(for prop in props) {
                if let Some(value) = element.get_attribute(#{prop.target.to_string()}) {
                    use ::fluent_web_runtime::internal::base64::engine::Engine;
                    let decoded = ::fluent_web_runtime::internal::base64::engine::general_purpose::STANDARD_NO_PAD.decode(value).unwrap();
                    let deserialized = ::fluent_web_runtime::internal::bincode::deserialize(&decoded).unwrap();
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
        fn detect_reads(&mut self, f: fn(&mut Component #{&generics.ty_generics}, Option<&str>)) {
            #{&unwraps.unpack_change_detector}
            #(for field in data) {
                if #{&field.target}.was_read() {
                    self.updates.#{&field.target}.insert((f, false));
                    #{&field.target}.clear();
                }
            }
        }
        fn detect_reads_ifs(&mut self, f: fn(&mut Component #{&generics.ty_generics}, Option<&str>)) {
            #{&unwraps.unpack_change_detector}
            #(for field in data) {
                if #{&field.target}.was_read() {
                    self.updates.#{&field.target}.insert((f, true));
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
    quote!(
        fn update_changed_values(&mut self) {
            #{&unwraps.unpack_change_detector}

            let mut __Fluent_Functions: ::std::vec::Vec<(fn(&mut Component #{&generics.ty_generics}, Option<&str>), bool)> = ::std::vec::Vec::new();

            #(for field in data) {
                if #{&field.target}.was_written() {__Fluent_Functions.extend(self.updates.#{&field.target}.iter());}
                #{&field.target}.clear();
            }

            __Fluent_Functions.sort_by_key(|(_, x)| *x);

            for (func, _) in __Fluent_Functions.into_iter() {
                func(self, None);
            }
        }
    )
}
