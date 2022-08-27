use crate::dom::document::Document;
use crate::dom::element::Element;
use dom_testing_library::query::matcher::Matcher;
use dom_testing_library::query::query_all_by;

pub trait QueryAllBy {
    fn query_all_by(&self, matcher: &impl Matcher) -> Vec<Element>;
}

impl QueryAllBy for Document {
    fn query_all_by(&self, matcher: &impl Matcher) -> Vec<Element> {
        query_all_by(self, matcher)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    mod query_all_by {
        use super::*;
        use crate::dom::element::Element;
        use dom_testing_library::dom::{Attribute, Element as TlElement};
        use dom_testing_library::query::matcher::role;

        #[wasm_bindgen_test]
        fn without_elements_returns_no_elements() {
            let document = render("");

            let matching_elements = document.query_all_by(&role::button());

            assert!(matching_elements.is_empty());
        }

        #[wasm_bindgen_test]
        fn returns_matching_elements() {
            let document = render(
                r#"<div><button role="button">Ok</button><button role="button">Cancel</button></div>"#,
            );

            let matching_elements = document.query_all_by(&role::button());

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
