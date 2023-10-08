//! Conditional rendering

use super::utils::visit_html_nodes;
use super::DefCallPair;
use crate::compiler::utils::{add_class, uuid};
use crate::prelude::*;

/// Information needed to compile if statemt
struct IfInfo {
    /// Element id
    id: String,
    /// Condition expression
    expression: syn::Expr,
    /// HTML string for the true case
    true_case: String,
}

/// Used for `parse_and_modify_html` because the error handling in there is a bit complex
/// In a okay case we just want the value
/// In a error case we wanna return Some(Err(...))
macro_rules! try_complex {
    ($expr:expr) => {
        match $expr {
            Ok(val) => val,
            Err(err) => return Some(Err(err.into())),
        }
    };
}

/// Parse the ifs and replace their tree with a div.
fn parse_and_modify_html(node: &kuchikiki::NodeRef) -> CompilerResult<Vec<IfInfo>> {
    use kuchikiki::NodeData;
    use markup5ever::namespace_url;

    visit_html_nodes(node, |node: &kuchikiki::NodeRef| {
        // To allow us to use ?
        (|| match node.data() {
            NodeData::Element(data) => {
                let mut attributes = data.attributes.borrow_mut();

                let text = attributes
                    .map
                    .iter()
                    .find_map(|(k, v)| (&*k.local == "$if").then_some(&v.value))?;

                let expression = try_complex!(syn::parse_str(text));

                let id = uuid();
                attributes.remove("$if");
                add_class(&mut attributes, &id);

                let false_case = kuchikiki::NodeRef::new_element(
                    html5ever::QualName::new(
                        None,
                        markup5ever::ns!(html),
                        markup5ever::local_name!("div"),
                    ),
                    [(
                        kuchikiki::ExpandedName::new(
                            markup5ever::ns!(),
                            markup5ever::local_name!("class"),
                        ),
                        kuchikiki::Attribute {
                            prefix: None,
                            value: id.clone(),
                        },
                    )],
                );

                node.insert_after(false_case);
                node.detach();

                let temp_parent = kuchikiki::NodeRef::new_element(
                    html5ever::QualName::new(
                        None,
                        markup5ever::ns!(html),
                        markup5ever::local_name!("div"),
                    ),
                    [],
                );
                temp_parent.append(node.clone());

                drop(attributes);

                let mut true_case = Vec::new();
                try_complex!(html5ever::serialize(
                    &mut true_case,
                    &temp_parent,
                    html5ever::serialize::SerializeOpts::default(),
                ));
                let true_case = try_complex!(String::from_utf8(true_case));

                Some(Ok(IfInfo {
                    id,
                    expression,
                    true_case,
                }))
            }
            _ => None,
        })()
        .into_iter()
        .collect()
    })
    .into_iter()
    .collect::<CompilerResult<Vec<_>>>()
}

/// Compile a if statement
fn compile_stmt(stmt: &IfInfo, data: &super::data_and_props::Unwraps) -> DefCallPair {
    let function_name = quote::format_ident!("__update_if_{}", stmt.id);

    DefCallPair {
        def: quote!(
            fn #function_name(&mut self, parent_id: Option<&str>) {
                #{&data.unpack_ref}
                ::fluent_web_runtime::internal::do_if(
                        #{&stmt.id},
                        &#{&stmt.true_case},
                        #{&stmt.expression},
                        parent_id,
                        self,
                );
                self.detect_reads(Self::#function_name);
            }
        ),
        call: quote!(self.#function_name(root.clone());),
    }
}

/// Compile ifs
pub fn compile(
    html: &kuchikiki::NodeRef,
    data: &super::data_and_props::Unwraps,
) -> CompilerResult<Vec<DefCallPair>> {
    let nodes = parse_and_modify_html(html)?;
    // feels hacky
    Ok(nodes
        .into_iter()
        .map(|node| compile_stmt(&node, data))
        .collect())
}
