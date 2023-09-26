//! Compile fluent files

use std::{
    env::current_dir,
    fs,
    path::{Path, PathBuf},
};

use html5ever::tendril::TendrilSink;

use crate::prelude::*;

mod computed_attribute;
mod conditional_attr;
mod custom_events;
mod data_and_props;
mod event_listen;
mod generics;
mod reactive_text;
mod style;
mod subcomponent;
mod utils;

use generics::Generics;
use utils::find_top_level_tag;

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

    clear_out_src_dir(&src)?;
    process_dir(&src_fluent, &src)?;

    Ok(())
}

/// Clear out the src directory to stop compilation errors from stopping trunk.
fn clear_out_src_dir(src: &PathBuf) -> CompilerResult<()> {
    fs::remove_dir_all(src)?;
    fs::create_dir_all(src)?;

    fs::File::create(src.join("main.rs"))?;

    Ok(())
}

/// Loop over all files in source directory and compile them into dst
fn process_dir(source: &Path, dst: &Path) -> CompilerResult<()> {
    for file in
        fs::read_dir(source)?.collect::<Result<Vec<_>, _>>()?
    {
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
            // This function should only be called with files
            #[allow(clippy::expect_used)]
            let component_name: &str = dst
                .file_stem()
                .expect("Expected file")
                .to_str()
                .expect("Valid utf8");

            let component_file = format!("{component_name}.rs");
            compile_fluent_file(
                source,
                dst.with_file_name(component_file),
            )?;
        }
        _ => (),
    }

    Ok(())
}

/// Get the html of a template.
fn get_html_body(
    source_content: &str,
) -> CompilerResult<kuchikiki::NodeRef> {
    let template_content =
        find_top_level_tag(source_content, "template").unwrap_or("");
    let parsed_html =
        kuchikiki::parse_html_with_options(kuchikiki::ParseOpts {
            tree_builder: html5ever::tree_builder::TreeBuilderOpts {
                drop_doctype: true,
                ..Default::default()
            },
            ..Default::default()
        })
        .from_utf8()
        .read_from(&mut template_content.as_bytes())?;

    // We know these are valid
    #[allow(clippy::expect_used)]
    Ok(parsed_html
        .select("body")
        .expect("A valid selector")
        .next()
        .expect("There to be a <body> tag")
        .as_node()
        .clone())
}

/// Compiler a fluent file to a rust file, this is the main block of code
fn compile_fluent_file(
    source: PathBuf,
    dst: PathBuf,
) -> CompilerResult<()> {
    let source_content = fs::read_to_string(source)?;

    let generics = generics::parse(&source_content)?;

    let prop_statements =
        data_and_props::parse_data_and_props_segement(
            find_top_level_tag(&source_content, "props")
                .unwrap_or(""),
            true,
        )?;
    let mut data_statements =
        data_and_props::parse_data_and_props_segement(
            find_top_level_tag(&source_content, "data").unwrap_or(""),
            false,
        )?;
    data_statements.extend(prop_statements.clone());
    let data = data_and_props::compile_unwraps(&data_statements);

    let define_parsed: syn::File = syn::parse_str(
        find_top_level_tag(&source_content, "define").unwrap_or(""),
    )?;

    let body_html = get_html_body(&source_content)?;

    let mut reactive_pairs = vec![];
    reactive_pairs
        .extend(reactive_text::compile(body_html.clone(), &data)?);
    reactive_pairs
        .extend(conditional_attr::compile(body_html.clone(), &data)?);
    reactive_pairs.extend(computed_attribute::compile(
        body_html.clone(),
        &data,
    )?);
    reactive_pairs.extend(style::compile(body_html.clone(), &data)?);

    let (reactive_defs, reactive_calls): (Vec<_>, Vec<_>) =
        reactive_pairs
            .into_iter()
            .map(|pair| (pair.def, pair.call))
            .unzip();

    let mut once_pairs = vec![];
    once_pairs.extend(subcomponent::compile(body_html.clone())?);
    once_pairs
        .extend(event_listen::compile(body_html.clone(), &data)?);

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

    html_content += &style::transform_css(
        find_top_level_tag(&source_content, "style").unwrap_or(""),
    )?;

    let component_source = quote!(
        #![allow(warnings)]
        use ::fluent_web_client::internal::web_sys::*;
        use ::fluent_web_client::internal::DomDisplay;
        use ::fluent_web_client::internal::UseInEvent;

        #define_parsed

        #{ data_and_props::compile_data_struct(&data_statements, &generics) }
        #{ data_and_props::compile_reactive_function_struct(&data_statements, &generics) }

        #[derive(::fluent_web_client::internal::Derivative)]
        #[derivative(Clone(bound = ""))]
        pub struct Component #{&generics.generic_def} {
            root_name: ::std::string::String,
            data: __Fluid_Data #{&generics.ty_generics},
            updates: ::std::rc::Rc<::std::cell::RefCell<__Fluid_Reactive_Functions #{&generics.ty_generics}>>,
        }

        #{custom_events::compile_events(&source_content, &generics)?}

        impl #{&generics.impl_generics} Component #{&generics.ty_generics} #{&generics.where_clauses} {
            #(#reactive_defs)*
            #(#once_defs)*

            #{data_and_props::compile_detect_reads(&data_statements, &generics, &data)}
            #{data_and_props::compile_update_changed_values(&data_statements, &generics, &data)}

            fn emit<E: ::fluent_web_client::internal::Event>(&self, event: &E) {
                ::fluent_web_client::internal::emit(&self.root_name, event);
            }
        }

        impl #{generics.impl_generics} ::fluent_web_client::internal::Component for Component #{&generics.ty_generics} #{generics.where_clauses} {
            fn render_init(&self) -> ::std::string::String {
                let root = &self.root_name;
                ::std::format!(#html_content)
            }

            #{data_and_props::compile_create(&data_statements)}
            #{data_and_props::compile_update_props(&prop_statements)}

            fn setup_onetime(&self, root: Option<String>) {
                #(#once_calls)*
            }

            fn update_all(&self, root: Option<String>) {
                self.update_props();
                #(#reactive_calls)*
            }
        }
    );
    let component_source: syn::File = syn::parse2(component_source)?;
    let component_source = prettyplease::unparse(&component_source);

    fs::write(dst, component_source)?;

    Ok(())
}
