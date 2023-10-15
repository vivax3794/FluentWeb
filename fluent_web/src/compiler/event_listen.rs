//! Compiler event listeners

use super::utils::{modify_html_code, uuid, ModifiedHtmlInfoWithCode};
use super::DefCallPair;
use crate::prelude::*;

/// Create the event listeners
fn compile_native_listener(
    mut event: ModifiedHtmlInfoWithCode<syn::ExprClosure>,
    data: &super::data_and_props::Unwraps,
) -> CompilerResult<DefCallPair> {
    let function_name = quote::format_ident!("set_event_{}", uuid());
    let function_name_internal = quote::format_ident!("{}_internal", function_name);

    let mut c = event.element.chars();
    let mut element_name_cap = c
        .next()
        .ok_or_else(|| Compiler::WrongSyntax("Event handler cant be empty name"))?
        .to_ascii_uppercase()
        .to_string()
        + c.as_str();

    if element_name_cap == "A" {
        element_name_cap = String::from("Anchor");
    }

    let element_type = quote::format_ident!("Html{}Element", element_name_cap);

    let event_reading = {
        // Just ask people to downcast it themself?
        let first = &event.code.inputs[0];
        let event_type = if let syn::Pat::Type(syn::PatType {
            ty: box syn::Type::Reference(syn::TypeReference { elem: box ty, .. }),
            ..
        }) = first
        {
            quote!(#ty)
        } else {
            quote!(::fluent_web_runtime::internal::web_sys::Event)
        };
        quote!(let __Fluent_Event = __Fluent_Event.dyn_ref::<#event_type>().unwrap();)
    };

    let second = &mut event.code.inputs[1];
    *second = syn::Pat::Type(syn::PatType {
        attrs: vec![],
        pat: Box::new(second.clone()),
        colon_token: <syn::Token![:]>::default(),
        ty: Box::new(syn::parse_quote!(&#element_type)),
    });

    let selector = format!(".{}", event.id);
    let set_event_handler = quote!(
        fn #function_name_internal(
                &mut self,
                __Fluent_Event: ::fluent_web_runtime::internal::web_sys::Event,
                __Fluent_Element: &::fluent_web_runtime::internal::web_sys::#element_type
            ) {
            use ::fluent_web_runtime::internal::wasm_bindgen::JsCast;
            #event_reading
            {
                #{&data.unpack_mut}
                (#{&event.code})(__Fluent_Event, __Fluent_Element);
            }
            self.update_changed_values();
        }

        fn #function_name(&mut self, __Fluent_S: Option<&str>) {
            use ::fluent_web_runtime::internal::Component;
            let __Fluent_Elements = ::fluent_web_runtime::internal::get_elements(self.root(), #selector, __Fluent_S);

            for __Fluent_Element in __Fluent_Elements.into_iter() {
                use ::fluent_web_runtime::internal::wasm_bindgen::JsCast;

                let __Fluent_Element_Typed = __Fluent_Element.dyn_ref::<::fluent_web_runtime::internal::web_sys::#element_type>().unwrap().to_owned();

                let comp = self.weak.clone().unwrap();
                let __Fluent_Function = ::fluent_web_runtime::internal::wasm_bindgen::closure::Closure::<dyn Fn(_)>::new(move |event: ::fluent_web_runtime::internal::web_sys::Event| {
                    comp.upgrade().unwrap().borrow_mut().#function_name_internal(event, &__Fluent_Element_Typed);
                });

                __Fluent_Element.add_event_listener_with_callback(#{&event.attribute}, __Fluent_Function.as_ref().unchecked_ref()).unwrap();
                __Fluent_Function.forget();
            }
        }
    );

    let call = quote!(self.#function_name(root.clone()););

    Ok(DefCallPair {
        def: set_event_handler,
        call,
    })
}

/// Compile custom event listeners
fn compile_custom_listener(
    mut event: ModifiedHtmlInfoWithCode<syn::ExprClosure, proc_macro2::TokenStream>,
    data: &super::data_and_props::Unwraps,
) -> CompilerResult<DefCallPair> {
    let function_name = quote::format_ident!("set_event_{}", uuid());
    let function_name_internal = quote::format_ident!("{}_internal", function_name);

    let event_reading = {
        let component_path: syn::Path = syn::parse2(event.src.clone())?;
        let mut segments = component_path.segments.into_iter();
        let last = segments
            .next_back()
            .ok_or_else(|| Compiler::WrongSyntax("Invalid component path"))?;
        let mut segments = segments.map(|p| quote!(#p)).collect::<Vec<_>>();

        let syn::PathSegment {
            ident, arguments, ..
        } = last;
        segments.push(quote!(#ident));

        let event_name = quote::format_ident!("{}", event.attribute);
        // WORKAROUND: template_quote does not support longer than 1 seperators

        let segments_combined = quote::quote!(#(#segments)::*);
        let event_type = quote!(
            <#segments_combined ::__Fluent_Events:: #event_name #arguments
                as ::fluent_web_runtime::internal::EventWrapper>
            ::Real
        );

        let first = event
            .code
            .inputs
            .first_mut()
            .ok_or_else(|| Compiler::WrongSyntax("Expected handler to have one attribute"))?;
        *first = syn::Pat::Type(syn::PatType {
            attrs: vec![],
            pat: Box::new(first.clone()),
            colon_token: <syn::Token![:]>::default(),
            ty: Box::new(syn::parse_quote!(#event_type)),
        });

        quote!(
            let Some(__Fluent_Custom_Event) = __Fluent_Event.dyn_ref::<::fluent_web_runtime::internal::web_sys::CustomEvent>() else {return;};
            let __Fluent_Details = __Fluent_Custom_Event.detail();
            let __Fluent_Details = __Fluent_Details.dyn_ref::<::fluent_web_runtime::internal::js_sys::Uint8Array>().unwrap();
            let __Fluent_Event: #event_type = ::fluent_web_runtime::internal::bincode::deserialize(&__Fluent_Details.to_vec()).unwrap();
        )
    };

    let selector = format!(".{}", event.id);
    let set_event_handler = quote!(
        fn #function_name_internal(
                &mut self,
                __Fluent_Event: ::fluent_web_runtime::internal::web_sys::Event
            ) {
            use ::fluent_web_runtime::internal::wasm_bindgen::JsCast;
            #event_reading
            {
                #{&data.unpack_mut}
                (#{&event.code})(__Fluent_Event);
            }
            self.update_changed_values();
        }

        fn #function_name(&mut self, __Fluent_S: Option<&str>) {
            use ::fluent_web_runtime::internal::Component;
            let __Fluent_Elements =
                ::fluent_web_runtime::internal
                ::get_elements(self.root(), #selector, __Fluent_S);

            for __Fluent_Element in __Fluent_Elements.into_iter() {
                use ::fluent_web_runtime::internal::wasm_bindgen::JsCast;

                let comp = self.weak.clone().unwrap();
                let __Fluent_Function = ::fluent_web_runtime::internal::wasm_bindgen::closure::Closure::<dyn Fn(_)>::new(move |event: ::fluent_web_runtime::internal::web_sys::Event| {
                    comp.upgrade().unwrap().borrow_mut().#function_name_internal(event);
                });

                __Fluent_Element.add_event_listener_with_callback(#{&event.attribute}, __Fluent_Function.as_ref().unchecked_ref()).unwrap();
                __Fluent_Function.forget();
            }
        }
    );

    let call = quote!(self.#function_name(root.clone()););

    Ok(DefCallPair {
        def: set_event_handler,
        call,
    })
}

/// Compiler custom and native events
pub fn compile(
    html: &kuchikiki::NodeRef,
    data: &super::data_and_props::Unwraps,
) -> CompilerResult<Vec<DefCallPair>> {
    let native_nodes = modify_html_code(html, ":")?;
    let custom_nodes = modify_html_code(html, ";")?;

    let native = native_nodes
        .into_iter()
        .map(|node| compile_native_listener(node, data));
    let custom = custom_nodes
        .into_iter()
        .map(|node| compile_custom_listener(node, data));

    Ok(native
        .chain(custom)
        .collect::<CompilerResult<Vec<_>>>()
        .into_iter()
        .flatten()
        .collect())
}
