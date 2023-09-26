//! Subcompoent <component> syntax

use super::{
    utils::{add_class, uuid},
    DefCallPair,
};
use crate::prelude::*;

/// Data used to create sub components
struct SubComponentData {
    /// Subcomponent id
    id: String,
    /// Path to component with generics
    component_name: syn::Path,
}

/// This finds <componet> tags, parses and stores its `src` and then replaces it with a <div>
#[allow(clippy::needless_pass_by_value)]
fn find_subcomponents(
    node: kuchikiki::NodeRef,
) -> CompilerResult<Vec<SubComponentData>> {
    use kuchikiki::NodeData;
    use markup5ever::namespace_url;

    match node.data() {
        NodeData::Element(data)
            if &data.name.local == "component" =>
        {
            let attributes = data.attributes.borrow();
            let component_name =
                attributes.get("src").ok_or(Compiler::MissingSrc)?;
            let component_name = syn::parse_str(component_name)?;
            let id = uuid();

            let mut attributes = attributes.clone();

            add_class(&mut attributes, &id);
            add_class(&mut attributes, "__Fluent_Needs_Init");

            let div = kuchikiki::NodeRef::new_element(
                html5ever::QualName {
                    prefix: None,
                    ns: markup5ever::ns!(html),
                    local: markup5ever::local_name!("div"),
                },
                attributes.map,
            );

            node.insert_before(div);
            node.detach();

            Ok(vec![SubComponentData { id, component_name }])
        }
        NodeData::Element(_) => Ok(node
            .children()
            .map(find_subcomponents)
            .collect::<CompilerResult<Vec<_>>>()?
            .into_iter()
            .flatten()
            .collect()),
        _ => Ok(vec![]),
    }
}

/// Create the calls used for creating sub components
fn compile_stmt(data: SubComponentData) -> DefCallPair {
    let function_name =
        quote::format_ident!("spawn_component_{}", data.id);

    let selector = format!(".{}.__Fluent_Needs_Init", data.id);
    let function_def = quote!(
        fn #function_name(&self, __Fluent_S: Option<String>) {
            let __Fluent_Elements = ::fluent_web_client::internal::get_elements(&self.root_name, #selector, __Fluent_S);
            for __Fluent_Element in __Fluent_Elements.into_iter() {
                let __Fluent_Id = ::fluent_web_client::internal::uuid();
                __Fluent_Element.set_id(&__Fluent_Id);
                ::fluent_web_client::render_component!(#{data.component_name}, &__Fluent_Id);
            }
        }
    );
    let function_call = quote!(self.#function_name(root.clone()););

    DefCallPair {
        def: function_def,
        call: function_call,
    }
}

/// Compile subcomponents inits
pub fn compile(
    html: kuchikiki::NodeRef,
) -> CompilerResult<Vec<DefCallPair>> {
    let nodes = find_subcomponents(html)?;
    Ok(nodes.into_iter().map(compile_stmt).collect())
}
