//! Compile fluent files

mod computed_attribute;
mod conditional_attr;
mod custom_events;
mod data_and_props;
mod event_listen;
mod generics;
mod ifs;
mod reactive_text;
mod style;
mod subcomponent;
mod utils;

use std::env::current_dir;
use std::fs;
use std::path::{Path, PathBuf};

use generics::Generics;
use html5ever::tendril::TendrilSink;
use utils::find_top_level_tag;

use crate::prelude::*;

/// A pair of a function def and a call to that function
struct DefCallPair {
    /// Function definition
    def: proc_macro2::TokenStream,
    /// Call the defined function
    call: proc_macro2::TokenStream,
}

/// Compiles all files in `./src_fluent` into `./src`
pub fn compile_project() -> CompilerResult<()> {
    let root_dir = current_dir()?;
    let src_fluent = root_dir.join("src_fluent");
    let src = root_dir.join("src");

    clear_out_src_dir(&src_fluent, &src)?;
    process_dir(&src_fluent, &src)?;

    Ok(())
}

/// Clear out the src directory to stop compilation errors from stopping trunk.
fn clear_out_src_dir(fluent: &Path, src: &Path) -> CompilerResult<()> {
    fs::remove_dir_all(src)?;
    fs::create_dir_all(src)?;

    let filename = if fluent.join("main.rs").exists() {
        "main.rs"
    } else {
        "lib.rs"
    };

    fs::File::create(src.join(filename))?;

    Ok(())
}

/// Loop over all files in source directory and compile them into dst
fn process_dir(source: &Path, dst: &Path) -> CompilerResult<()> {
    for file in fs::read_dir(source)?.collect::<Result<Vec<_>, _>>()? {
        let name = file.file_name();
        if file.file_type()?.is_dir() {
            let dst = dst.join(name.clone());

            if !dst.exists() {
                fs::create_dir_all(&dst)?;
            }

            process_dir(&source.join(name), &dst)?;
        } else {
            process_file(source.join(&name), dst.join(name))?;
        }
    }

    Ok(())
}

/// Copy rust files unchanged, and run the compilation for fluent files
fn process_file(source: PathBuf, dst: PathBuf) -> CompilerResult<()> {
    match source.extension().and_then(|ext| ext.to_str()) {
        Some("rs") => {
            fs::copy(source, dst)?;
        }
        Some("fluent") => {
            #[expect(clippy::expect_used, reason = "This should only be called with files.")]
            let component_name: &str = dst
                .file_stem()
                .expect("Expected file")
                .to_str()
                .expect("Valid utf8");

            let component_file = format!("{component_name}.rs");
            compile_fluent_file(source, dst.with_file_name(component_file))?;
        }
        _ => (),
    }

    Ok(())
}

/// Get the html of a template.
fn get_html_body(source_content: &str) -> kuchikiki::NodeRef {
    let template_content = find_top_level_tag(source_content, "template").unwrap_or("");

    #[expect(clippy::expect_used, reason = "Rust strings are always utf-8")]
    let parsed_html = kuchikiki::parse_html_with_options(kuchikiki::ParseOpts {
        tree_builder: html5ever::tree_builder::TreeBuilderOpts {
            drop_doctype: true,
            ..Default::default()
        },
        ..Default::default()
    })
    .from_utf8()
    .read_from(&mut template_content.as_bytes())
    .expect("Rust strings are always utf-8");

    #[expect(
        clippy::expect_used,
        reason = "'body' is a valid tag selector, and the parsed html will always have a inserted body element"
    )]
    parsed_html
        .select("body")
        .expect("A valid selector")
        .next()
        .expect("There to be a <body> tag")
        .as_node()
        .clone()
}

/// Compile Component struct
fn compile_component_struct(generics: &Generics) -> proc_macro2::TokenStream {
    quote!(
    pub struct Component #{&generics.generic_def} {
        root_name: Box<str>,
        data: __Fluid_Data #{&generics.ty_vars},
        updates: __Fluid_Reactive_Functions #{&generics.ty_vars},
        subs: ::std::collections::HashMap<Box<str>, ::std::rc::Rc<dyn std::any::Any>>,
        weak: ::std::option::Option<::fluent_web_runtime::internal::WeakRef<Component #{&generics.ty_vars}>>,
        obs: Option<::fluent_web_runtime::internal::web_sys::MutationObserver>,
        _f:  Option<::fluent_web_runtime::internal::wasm_bindgen::closure::Closure::<dyn Fn(Vec<::fluent_web_runtime::internal::web_sys::MutationRecord>)>>
    }
    )
}

/// Compiler a fluent file to a rust file, this is the main block of code
fn compile_fluent_file(source: PathBuf, dst: PathBuf) -> CompilerResult<()> {
    let source_content = fs::read_to_string(source)?;

    let generics = generics::parse(&source_content)?;

    let prop_statements = data_and_props::parse_data_and_props_segement(
        find_top_level_tag(&source_content, "props").unwrap_or(""),
        true,
    )?;
    let mut data_statements = data_and_props::parse_data_and_props_segement(
        find_top_level_tag(&source_content, "data").unwrap_or(""),
        false,
    )?;
    data_statements.extend(prop_statements.clone());
    let unwraps = data_and_props::compile_unwraps(&data_statements);

    let define_parsed: syn::File =
        syn::parse_str(find_top_level_tag(&source_content, "define").unwrap_or(""))?;
    let setup_parsed: syn::Expr = syn::parse_str(&format!(
        "{{{}}}",
        find_top_level_tag(&source_content, "setup").unwrap_or("")
    ))?;

    let body_html = get_html_body(&source_content);

    let mut reactive_pairs = vec![];
    reactive_pairs.extend(reactive_text::compile(&body_html, &unwraps)?);
    reactive_pairs.extend(conditional_attr::compile(&body_html, &unwraps)?);
    reactive_pairs.extend(computed_attribute::compile(&body_html, &unwraps)?);
    reactive_pairs.extend(style::compile(&body_html, &unwraps)?);

    let mut once_pairs = vec![];
    once_pairs.extend(subcomponent::compile(&body_html)?);
    once_pairs.extend(event_listen::compile(&body_html, &unwraps)?);

    // Important that this is last
    // TODO: This is gonna be a issue when we get to loops and multiple types of ifs.
    reactive_pairs.extend(ifs::compile(&body_html, &unwraps)?);

    let (reactive_defs, reactive_calls): (Vec<_>, Vec<_>) = reactive_pairs
        .into_iter()
        .map(|pair| (pair.def, pair.call))
        .unzip();
    let (once_defs, once_calls): (Vec<_>, Vec<_>) = once_pairs
        .into_iter()
        .map(|pair| (pair.def, pair.call))
        .unzip();

    let mut html_content = Vec::new();
    html5ever::serialize(
        &mut html_content,
        &body_html,
        html5ever::serialize::SerializeOpts::default(),
    )?;
    let mut html_content = String::from_utf8(html_content)?;
    html_content +=
        &style::transform_css(find_top_level_tag(&source_content, "style").unwrap_or(""))?;

    let component_source = quote!(
        // @generated
        #![allow(warnings)]
        use ::fluent_web_runtime::{CompRef, internal::{web_sys::*, DomDisplay, UseInEvent, Component as __Fluent_Comp_Trait}};

        #{ data_and_props::compile_unwrap_macro(&data_statements) }
        #define_parsed
        #{ data_and_props::compile_data_struct(&data_statements, &generics) }
        #{ data_and_props::compile_reactive_function_struct(&data_statements, &generics) }
        #{ compile_component_struct(&generics) }
        #{ custom_events::compile_events(&source_content, &generics)? }

        impl #{&generics.impl_vars} Component #{&generics.ty_vars} #{&generics.where_clauses} {
            #(#reactive_defs)*
            #(#once_defs)*

            #{data_and_props::compile_detect_reads(&data_statements, &generics, &unwraps)}

            fn emit<E: ::fluent_web_runtime::internal::Event>(&self, event: &E) {
                ::fluent_web_runtime::internal::emit(&self.root_name, event);
            }
            fn weak(&self) -> ::fluent_web_runtime::internal::WeakRef<Self> {
                self.weak.clone().unwrap()
            }
        }

        impl #{&generics.impl_vars} ::fluent_web_runtime::internal::Component for Component #{&generics.ty_vars} #{&generics.where_clauses} {
            fn setup_watcher(&mut self) {
                use ::fluent_web_runtime::internal::wasm_bindgen::JsCast;

                let component = self.weak();
                let function = move |mutations: Vec<::fluent_web_runtime::internal::web_sys::MutationRecord>| {
                    if let Some(comp) = component.clone().upgrade() {
                       let mut comp = comp.borrow_mut();
                       for mutation in mutations.into_iter() {
                           let nodes = mutation.removed_nodes();
                           for i in 0..nodes.length() {
                               let node = nodes.get(i).unwrap();
                               if let Some(element) = node.dyn_ref::<::fluent_web_runtime::internal::web_sys::Element>() {
                                   let mut stack = vec![element.clone()];
                                   while let Some(element) = stack.pop() {
                                       if let Some(id) = element.get_attribute("id") {
                                           let sub = comp.subs.remove(&*id);
                                           if let Some(sub) = sub {
                                               drop(sub);
                                           }
                                       }
                                       let children = element.children();
                                       for j in 0..children.length() {
                                           stack.push(children.item(j).unwrap());
                                       }
                                   }
                               }
                           }
                       }
                    }
                };
                let function = ::fluent_web_runtime::internal::wasm_bindgen::closure::Closure::<dyn Fn(Vec<::fluent_web_runtime::internal::web_sys::MutationRecord>)>::new(function);
                let js_function = function.as_ref().unchecked_ref();
                let observer = ::fluent_web_runtime::internal::web_sys::MutationObserver::new(js_function).unwrap();

                let element = ::fluent_web_runtime::internal::get_by_id(self.root());

                let mut options = ::fluent_web_runtime::internal::web_sys::MutationObserverInit::new();
                options.child_list(true);
                options.subtree(true);
                observer.observe_with_options(&element, &options).unwrap();

                self.obs = Some(observer);
                self._f = Some(function);
            }

            fn render_init(&self) -> ::std::string::String {
                let root = &self.root_name;
                ::std::format!(#html_content)
            }

            #{data_and_props::compile_create(&data_statements)}

            fn root(&self) -> &str {
                &self.root_name
            }
            fn set_weak(&mut self, weak: ::fluent_web_runtime::internal::WeakRef<Self>) {
                self.weak = Some(weak);
            }

            #{data_and_props::compile_update_changed_values(&data_statements, &generics, &unwraps)}
            #{data_and_props::compile_update_props(&prop_statements)}

            fn setup_onetime(&mut self, root: Option<&str>) {
                #(#once_calls)*
            }

            fn update_all(&mut self, root: Option<&str>) {
                self.update_props();
                #(#reactive_calls)*
            }

            fn setup(&mut self) {
                #{&unwraps.unpack_mut}
                #setup_parsed
                self.update_changed_values();
            }
        }

        impl #{&generics.impl_vars} Drop for Component #{&generics.ty_vars} {
            fn drop(&mut self) {
                if let Some(obs) = self.obs.take() {
                    obs.disconnect();
                    drop(obs);
                }
            }
        }
    );
    let component_source: syn::File = syn::parse2(component_source)?;
    let component_source = prettyplease::unparse(&component_source);

    fs::write(dst, component_source)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use proptest::prelude::*;

    use super::*;

    proptest! {
        #[test]
        fn parse_html_doesnt_crash(s in ".*") {
            get_html_body(&s);
        }

        #[test]
        fn parse_html_doesnt_crash_with_template(s in "<template>.*</template>") {
            get_html_body(&s);
        }
    }
}
