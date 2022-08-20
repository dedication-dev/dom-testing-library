#[cfg(test)]
mod tests {
    use crate::util::test::ParseDocument;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    mod query_all_by {
        use super::*;
        use crate::dom::element::Element;
        use crate::dom::queryable::Document;
        use dom_testing_library::dom::attribute::Attribute;
        use dom_testing_library::dom::element::Element as TlElement;
        use dom_testing_library::query::matcher::role;
        use dom_testing_library::query::query_all_by::QueryAllBy;

        #[wasm_bindgen_test]
        fn without_elements_returns_no_elements() {
            let document = "".parse_document();

            let matching_elements = Document::from(document).query_all_by(&role::button());

            assert!(matching_elements.is_empty());
        }

        #[wasm_bindgen_test]
        fn returns_matching_elements() {
            let document = r#"<div><button role="button">Ok</button><button role="button">Cancel</button></div>"#.parse_document();

            let matching_elements = Document::from(document).query_all_by(&role::button());

            assert_eq!(matching_elements.len(), 2);
            matching_elements.iter().for_each(assert_has_role_button);
        }

        fn assert_has_role_button(element: &Element) {
            assert_eq!(
                element.attribute(&"role".into()),
                Some(Attribute::value("role", "button"))
            );
        }
    }
}
