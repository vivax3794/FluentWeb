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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_top_level_tag() {
        const CONTENT: &str = "<a>Hello World</a> <b>123</b>";

        assert_eq!(
            "Hello World",
            find_top_level_tag(CONTENT, "a")
                .expect("<a> to be in test content")
        );
        assert_eq!(
            "123",
            find_top_level_tag(CONTENT, "b")
                .expect("<b> to be in test content")
        );
    }
}
