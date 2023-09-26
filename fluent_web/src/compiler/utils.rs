//! Common util methods

use crate::error::{Compiler, CompilerResult};

/// Find top level tags as a proper html parser would "corrupt" the content of the rust tags
#[must_use]
pub fn find_top_level_tag<'a>(
    document: &'a str,
    tag: &str,
) -> Option<&'a str> {
    let open_tag = format!("<{tag}>");
    let close_tag = format!("</{tag}>");

    let index_first = document.find(&open_tag)?;
    let region_start = index_first + open_tag.len();
    let region_end = document.find(&close_tag)?;

    Some(&document[region_start..region_end])
}

/// Create a UUID in the format for fluent web
#[must_use]
pub fn uuid() -> String {
    let id = uuid::Uuid::new_v4().to_string().replace('-', "_");
    format!("__Fluent_UUID_{id}")
}

/// Add a class to the `class` attribute on a node
/// This also creates the class attribute if it is not present.
#[allow(clippy::expect_used)]
pub fn add_class(
    attributes: &mut kuchikiki::Attributes,
    class: &str,
) {
    let current_class =
        if let Some(value) = attributes.get_mut("class") {
            value
        } else {
            attributes.insert("class", String::new());
            attributes
                .get_mut("class")
                .expect("Newly inserted class to be there")
        };

    current_class.push(' ');
    current_class.push_str(class);
}

/// Extract rust code from format strings returning the string with just {} and a vector of the expressions
pub fn extract_format_strings(
    text: &str,
) -> CompilerResult<(String, Vec<syn::Expr>)> {
    let mut format_string = String::with_capacity(text.len());
    let mut expressions = Vec::new();

    let mut current_str = String::new();
    let mut in_template = false;

    // Find all {} pairs in the text.
    for c in text.chars() {
        match c {
            '{' => {
                in_template = true;
                format_string += "{";
                current_str.clear();
            }
            '}' => {
                in_template = false;
                format_string += "}";
                expressions.push(syn::parse_str(&current_str)?);
                current_str.clear();
            }
            c => {
                if in_template {
                    current_str.push(c);
                } else {
                    format_string.push(c);
                }
            }
        }
    }
    Ok((format_string, expressions))
}

/// Info about the modified html
pub struct ModifiedHtmlInfo {
    /// Id of the element
    pub id: String,
    /// The attribute (without prefixc)
    pub attribute: String,
    /// The attribute value
    pub value: String,
    /// Element name
    pub element: String,
    /// Src element, used by custom event listener
    pub src: Option<String>,
}

/// Find conditional attributes
#[allow(clippy::needless_pass_by_value)]
pub fn modify_html(
    node: kuchikiki::NodeRef,
    prefix: &str,
) -> Vec<ModifiedHtmlInfo> {
    use kuchikiki::NodeData;
    match node.data() {
        NodeData::Element(data) => {
            let mut attributes = data.attributes.borrow_mut();
            let prefixed_attributes = attributes
                .map
                .iter()
                .filter(|&(name, _)| name.local.starts_with(prefix))
                .map(|(name, content)| {
                    (name.local.to_string(), content.value.clone())
                })
                .collect::<Vec<_>>();

            let mut this_element = if prefixed_attributes.is_empty() {
                vec![]
            } else {
                let id = uuid();
                add_class(&mut attributes, &id);

                prefixed_attributes
                    .into_iter()
                    .map(|(name, value)| {
                        attributes.remove(name.clone());

                        // The filter has made sure this is safe
                        #[allow(clippy::expect_used)]
                        let name = name
                            .strip_prefix(prefix)
                            .expect("Name to start with ?")
                            .to_owned();

                        ModifiedHtmlInfo {
                            id: id.clone(),
                            attribute: name,
                            value,
                            element: data.name.local.to_string(),
                            src: attributes
                                .get("src")
                                .map(std::borrow::ToOwned::to_owned),
                        }
                    })
                    .collect()
            };

            this_element.extend(
                node.children().flat_map(|x| modify_html(x, prefix)),
            );
            this_element
        }
        _ => vec![],
    }
}

/// Same as `ModifiedHtmlInfo` but with a parsed code object
pub struct ModifiedHtmlInfoWithCode<T, S = ()> {
    /// Id of the element
    pub id: String,
    /// The attribute (without prefixc)
    pub attribute: String,
    /// The attribute value
    pub code: T,
    /// Element name
    pub element: String,
    /// The src attribute, used by custom event listener
    pub src: S,
}

/// Helper trait to parse `src` only in some cases
pub trait GetSrc: Sized {
    /// Parse `src` into the input type
    fn parse(src: Option<String>) -> CompilerResult<Self>;
}

impl GetSrc for () {
    fn parse(_: Option<String>) -> CompilerResult<Self> {
        Ok(())
    }
}

impl GetSrc for proc_macro2::TokenStream {
    fn parse(src: Option<String>) -> CompilerResult<Self> {
        match src {
            None => Err(Compiler::MissingSrc),
            Some(val) => Ok(syn::parse_str(&val)?),
        }
    }
}

/// Same as `modify_html` but parses the attribute value into a `syn` ast node.
#[must_use = "No reason to use this instead of `modify_html` if you dont use the result"]
pub fn modify_html_code<T: syn::parse::Parse, S: GetSrc>(
    html: kuchikiki::NodeRef,
    prefix: &str,
) -> CompilerResult<Vec<ModifiedHtmlInfoWithCode<T, S>>> {
    modify_html(html, prefix)
        .into_iter()
        .map(|node| {
            Ok(ModifiedHtmlInfoWithCode {
                id: node.id,
                attribute: node.attribute,
                code: syn::parse_str(&node.value)?,
                element: node.element,
                src: S::parse(node.src)?,
            })
        })
        .collect()
}
