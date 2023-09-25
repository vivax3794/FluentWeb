//! Common util methods

/// Find top level tags as a proper html parser would "corrupt" the content of the rust tags
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
pub fn uuid() -> String {
    let id = uuid::Uuid::new_v4().to_string().replace('-', "_");
    format!("__Fluent_UUID_{id}")
}

/// Add a class to the `class` attribute on a node
/// This also creates the class attribute if it is not present.
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
) -> (String, Vec<syn::Expr>) {
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
                expressions.push(
                    syn::parse_str(&current_str).expect(
                        "format content to be valid expression",
                    ),
                );
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
    (format_string, expressions)
}
