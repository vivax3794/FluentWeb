//! <style> tag for scoped style and css vars

use std::borrow::BorrowMut;
use std::convert::Infallible;

use lightningcss::visitor::Visit;

use super::utils::{add_class, extract_format_strings, uuid};
use crate::prelude::*;

/// This transforms the css with `__Fluent_UUID_XXX` where the root id should go.
struct CssTransformer {
    /// The UUID string that should be replaced with {root}
    replacement_string: String,
}

impl CssTransformer {
    /// Create a new transformer with a random uuid
    fn new() -> Self {
        let uuid = uuid();
        CssTransformer {
            replacement_string: uuid,
        }
    }
}

impl<'i> lightningcss::visitor::Visitor<'i> for CssTransformer {
    type Error = Infallible;

    fn visit_types(&self) -> lightningcss::visitor::VisitTypes {
        lightningcss::visit_types!(SELECTORS)
    }

    fn visit_selector(
        &mut self,
        selector: &mut lightningcss::selector::Selector<'i>,
    ) -> Result<(), Self::Error> {
        let mut segements = Vec::new();
        let mut components_iterator = selector.iter();

        let mut combiner = None;
        loop {
            segements.push((
                components_iterator
                    .borrow_mut()
                    .map(Clone::clone)
                    .collect::<Vec<_>>(),
                combiner,
            ));
            combiner = components_iterator.next_sequence();

            if combiner.is_none() {
                break;
            }
        }

        let where_clause =
            lightningcss::selector::Component::Where(Box::new([
                lightningcss::selector::Selector::from(vec![
                    lightningcss::selector::Component::ID(
                        self.replacement_string.clone().into(),
                    ),
                    lightningcss::selector::Component::Combinator(lightningcss::selector::Combinator::Descendant),
                    lightningcss::selector::Component::ExplicitUniversalType,
                    lightningcss::selector::Component::Negation(Box::new([
                        lightningcss::selector::Selector::from(vec![
                            lightningcss::selector::Component::ID(
                                self.replacement_string.clone().into(),
                            ),
                            lightningcss::selector::Component::Combinator(lightningcss::selector::Combinator::Descendant),
                            lightningcss::selector::Component::Class(
                                "__Fluent_Component".into(),
                            ),
                            lightningcss::selector::Component::Combinator(lightningcss::selector::Combinator::Descendant),
                            lightningcss::selector::Component::ExplicitUniversalType,
                            ]),
                        ])
                    )
                ]),
            ]));

        segements[0].0.push(where_clause.clone());

        if segements.len() > 1 {
            segements
                .last_mut()
                .expect("Vector containg >1 elements not to be empty")
                .0
                .push(where_clause);
        }

        let segements = segements
            .into_iter()
            .flat_map(|(mut components, combinator)| {
                if let Some(comb) = combinator {
                    components.insert(
                        0,
                        lightningcss::selector::Component::Combinator(
                            comb,
                        ),
                    );
                    components
                } else {
                    components
                }
            })
            .collect::<Vec<_>>();
        let new_selector: lightningcss::selector::Selector =
            segements.into();
        *selector = new_selector;

        Ok(())
    }
}

/// Transforms the css by adding the returned string as a placeholder for the rootname
/// This scopes the css to the specific component using the same selector as the `fluent_web_client`
fn transform_stylesheet(
    css: &mut lightningcss::stylesheet::StyleSheet,
) -> String {
    let mut trans = CssTransformer::new();
    css.visit(&mut trans).unwrap();
    trans.replacement_string
}

/// Transform the css into scoped css with {root} as a placeholder for the root id
pub fn transform_css(css_raw: &str) -> String {
    let mut css_parsed = lightningcss::stylesheet::StyleSheet::parse(
        css_raw,
        lightningcss::stylesheet::ParserOptions::default(),
    )
    .expect("<style> tag to be valid css");

    let replace_string = transform_stylesheet(&mut css_parsed);

    let css_content = css_parsed
        .to_css(lightningcss::stylesheet::PrinterOptions {
            minify: true,
            ..Default::default()
        })
        .expect("To be able to minify css")
        .code;

    let css_content = css_content
        .replace('{', "{{")
        .replace('}', "}}")
        .replace(&replace_string, "{root}");

    format!("<style>{css_content}</style>")
}
/// Data for a css var
pub struct ReactiveCssVar {
    /// Element Id
    id: String,
    /// Css variable
    var: String,
    /// The format string (with just {})
    format_string: String,
    /// Rust expressions to fill format string
    expressions: Vec<syn::Expr>,
}

/// Find css vars in the html and remove them, and then return the info
#[allow(clippy::needless_pass_by_value)]
pub fn find_and_remove_css_vars(
    node: kuchikiki::NodeRef,
) -> Vec<ReactiveCssVar> {
    use kuchikiki::NodeData;
    match node.data() {
        NodeData::Element(data) => {
            let mut attributes = data.attributes.borrow_mut();
            let vars = attributes
                .map
                .iter()
                .filter(|&(name, _)| name.local.starts_with("--"))
                .map(|(name, value)| {
                    (name.local.to_string(), value.value.clone())
                })
                .collect::<Vec<_>>();

            let mut result = if vars.is_empty() {
                vec![]
            } else {
                let id = uuid();
                add_class(&mut attributes, &id);

                vars.into_iter()
                    .map(|(name, value)| {
                        attributes.remove(name.clone());

                        let (text, expressions) =
                            extract_format_strings(&value);
                        ReactiveCssVar {
                            id: id.clone(),
                            var: name.clone(),
                            format_string: text,
                            expressions,
                        }
                    })
                    .collect()
            };

            result.extend(
                node.children().flat_map(find_and_remove_css_vars),
            );

            result
        }
        _ => vec![],
    }
}

/// Compile css vars into a update function, and a call to that update function
pub fn compile_css_vars(
    var: ReactiveCssVar,
    data: &super::data_and_props::Unwraps,
) -> (proc_macro2::TokenStream, proc_macro2::TokenStream) {
    let ReactiveCssVar {
        id,
        var,
        format_string,
        expressions,
    } = var;

    let function_name = quote::format_ident!("update_css_{}", uuid());
    let selector = format!(".{id}");

    let def = quote!(
        fn #function_name(&self, __Fluent_S: Option<String>) {
            #{&data.unpack_ref}

            let __Fluent_Elements = ::fluent_web_client::internal::get_elements(&self.root_name, #selector, __Fluent_S);
            for __Fluent_Element in __Fluent_Elements.into_iter() {
                let __Fluent_Value = ::std::format!(#format_string, #(#expressions),*);
                use fluent_web_client::internal::wasm_bindgen::JsCast;
                let __Fluent_Element = __Fluent_Element.dyn_into::<::fluent_web_client::internal::web_sys::HtmlElement>().unwrap();
                __Fluent_Element.style().set_property(#var, &__Fluent_Value).unwrap();
            }

            self.detect_reads(Component::#function_name);
        }
    );
    let call = quote!(self.#function_name(root.clone()););
    (def, call)
}
