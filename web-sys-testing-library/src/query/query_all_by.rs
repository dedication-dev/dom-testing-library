use crate::dom::Document;
use crate::dom::VecElementExt as _;
use dom_testing_library::query::Matcher;
use dom_testing_library::query_all_by;

/// Used to extend various [web_sys] types.
pub trait QueryAllBy {
    /// Get all [Element]s in self matching [Matcher].
    ///
    /// # Examples
    /// ```no_run
    /// use wasm_bindgen_test::*;
    /// use web_sys_testing_library::{render_html, QueryAllBy, matcher::role};
    ///
    /// wasm_bindgen_test_configure!(run_in_browser);
    ///
    /// #[wasm_bindgen_test]
    /// fn test() {
    ///     let document = render_html(r#"<div><button role="button">Ok</button></div>"#);
    ///
    ///     let _button_elements = document.query_all_by(&role::button());
    /// }
    /// ```
    fn query_all_by(&self, matcher: &impl Matcher) -> Vec<web_sys::Element>;
}

impl QueryAllBy for web_sys::Document {
    fn query_all_by(&self, matcher: &impl Matcher) -> Vec<web_sys::Element> {
        query_all_by(&Document::from(self), matcher).into_web_sys_elements()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render_html;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    mod query_all_by {
        use super::*;
        use dom_testing_library::query::matcher::role;

        #[wasm_bindgen_test]
        fn without_elements_returns_no_elements() {
            let document = render_html("");

            let matching_elements = document.query_all_by(&role::button());

            assert!(matching_elements.is_empty());
        }

        #[wasm_bindgen_test]
        fn returns_matching_elements() {
            let document = render_html(
                r#"<div><button role="button">Ok</button><button role="button">Cancel</button></div>"#,
            );

            let matching_elements = document.query_all_by(&role::button());

            assert_eq!(matching_elements.len(), 2);
            matching_elements.iter().for_each(assert_has_role_button);
        }

        fn assert_has_role_button(element: &web_sys::Element) {
            assert_eq!(element.get_attribute("role"), Some("button".to_string()));
        }
    }
}
