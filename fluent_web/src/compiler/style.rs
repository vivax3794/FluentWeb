//! <style> tag for scoped style and css vars

use std::borrow::BorrowMut;
use std::convert::Infallible;

use lightningcss::visitor::Visit;

use super::utils::{extract_format_strings, modify_html, uuid, ModifiedHtmlInfo};
use super::DefCallPair;
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

        let where_clause = lightningcss::selector::Component::Where(Box::new([
            lightningcss::selector::Selector::from(vec![
                lightningcss::selector::Component::ID(self.replacement_string.clone().into()),
                lightningcss::selector::Component::Combinator(
                    lightningcss::selector::Combinator::Descendant,
                ),
                lightningcss::selector::Component::ExplicitUniversalType,
                lightningcss::selector::Component::Negation(Box::new([
                    lightningcss::selector::Selector::from(vec![
                        lightningcss::selector::Component::ID(
                            self.replacement_string.clone().into(),
                        ),
                        lightningcss::selector::Component::Combinator(
                            lightningcss::selector::Combinator::Descendant,
                        ),
                        lightningcss::selector::Component::Class("__Fluent_Component".into()),
                        lightningcss::selector::Component::Combinator(
                            lightningcss::selector::Combinator::Descendant,
                        ),
                        lightningcss::selector::Component::ExplicitUniversalType,
                    ]),
                ])),
            ]),
        ]));

        segements[0].0.push(where_clause.clone());

        if let Some(last_value) = segements.last_mut() {
            last_value.0.push(where_clause);
        }

        let segements = segements
            .into_iter()
            .rev()
            .flat_map(|(mut components, combinator)| {
                if let Some(comb) = combinator {
                    components.push(lightningcss::selector::Component::Combinator(comb));
                    components
                } else {
                    components
                }
            })
            .collect::<Vec<_>>();

        let new_selector: lightningcss::selector::Selector = segements.into();
        *selector = new_selector;

        Ok(())
    }
}

/// Transforms the css by adding the returned string as a placeholder for the rootname
/// This scopes the css to the specific component using the same selector as the `fluent_web_runtime`
fn transform_stylesheet(css: &mut lightningcss::stylesheet::StyleSheet) -> String {
    let mut trans = CssTransformer::new();
    css.visit(&mut trans).unwrap();
    trans.replacement_string
}

/// Transform the css into scoped css with {root} as a placeholder for the root id
pub fn transform_css(css_raw: &str) -> CompilerResult<String> {
    let mut css_parsed = lightningcss::stylesheet::StyleSheet::parse(
        css_raw,
        lightningcss::stylesheet::ParserOptions::default(),
    )
    .map_err(|err| Compiler::CssPraseError(format!("{err}")))?;

    let replace_string = transform_stylesheet(&mut css_parsed);

    let css_content = css_parsed
        .to_css(lightningcss::stylesheet::PrinterOptions {
            minify: true,
            ..Default::default()
        })?
        .code;

    let css_content = css_content
        .replace('{', "{{")
        .replace('}', "}}")
        .replace(&replace_string, "{root}");

    Ok(format!("<style>{css_content}</style>"))
}

/// Compile css vars into a update function, and a call to that update function
fn compile_css_vars(
    var: &ModifiedHtmlInfo,
    data: &super::data_and_props::Unwraps,
) -> CompilerResult<DefCallPair> {
    let (format_string, expressions) = extract_format_strings(&var.value)?;

    let function_name = quote::format_ident!("update_css_{}", uuid());
    let selector = format!(".{}", var.id);

    let def = quote!(
        fn #function_name(&mut self, __Fluent_S: Option<&str>) {
            #{&data.unpack_ref}

            let __Fluent_Elements = ::fluent_web_runtime::internal::get_elements(&self.root_name, #selector, __Fluent_S);
            for __Fluent_Element in __Fluent_Elements.into_iter() {
                let __Fluent_Value = ::std::format!(#format_string #(,#expressions)*);
                use ::fluent_web_runtime::internal::wasm_bindgen::JsCast;
                let __Fluent_Element = __Fluent_Element.dyn_into::<::fluent_web_runtime::internal::web_sys::HtmlElement>().unwrap();
                __Fluent_Element.style().set_property(#{format!("--{}", var.attribute)}, &__Fluent_Value).unwrap();
            }

            self.detect_reads(Component::#function_name);
        }
    );
    let call = quote!(self.#function_name(root.clone()););
    Ok(DefCallPair { def, call })
}

/// Compile css vars
pub fn compile(
    html: &kuchikiki::NodeRef,
    data: &super::data_and_props::Unwraps,
) -> CompilerResult<Vec<DefCallPair>> {
    let nodes = modify_html(html, "--");
    nodes
        .into_iter()
        .map(|node| compile_css_vars(&node, data))
        .collect()
}
