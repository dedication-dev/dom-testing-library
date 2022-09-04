use crate::dom::{Document, Element};
use dom_testing_library::query::Matcher;
use std::error::Error;
use std::fmt;

/// Used to extend various [web_sys] types.
impl Document {
    /// Get all [Element]s in self matching [Matcher].
    ///
    /// Returns [GetAllByError::NoMatchingElement] when no [Element] matches.
    ///
    /// # Examples
    /// ```no_run
    /// use wasm_bindgen_test::*;
    /// use web_sys_testing_library::{render_html, matcher::role};
    ///
    /// wasm_bindgen_test_configure!(run_in_browser);
    ///
    /// #[wasm_bindgen_test]
    /// fn test() {
    ///     let document = render_html(r#"<div><button role="button">Ok</button></div>"#);
    ///
    ///     let _button_elements = document.get_all_by(&role::button()).unwrap();
    /// }
    /// ```
    pub fn get_all_by(&self, matcher: &impl Matcher) -> GetAllByResult {
        let elements = self.query_all_by(matcher);

        if !elements.is_empty() {
            return Ok(elements);
        }

        Err(GetAllByError::NoMatchingElement)
    }
}

/// Result returned from [Document::get_all_by].
pub type GetAllByResult = Result<Vec<Element>, GetAllByError>;

/// Error type to represent [Document::get_all_by] errors.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum GetAllByError {
    /// Returned when no matching [web_sys::Element] is found.
    NoMatchingElement,
}

impl fmt::Display for GetAllByError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GetAllByError::NoMatchingElement => write!(f, "no matching element found"),
        }
    }
}

impl Error for GetAllByError {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render_html;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    mod get_all_by {
        use super::*;
        use dom_testing_library::query::matcher::role;

        #[wasm_bindgen_test]
        fn without_matching_elements_returns_error() {
            let document = render_html("<div></div>");

            let result = document.get_all_by(&role::button());

            assert_eq!(result, Err(GetAllByError::NoMatchingElement));
        }

        #[wasm_bindgen_test]
        fn returns_matching_elements() {
            let document = render_html(
                r#"<div><button role="button">Ok</button><button role="button">Cancel</button></div>"#,
            );

            let matching_elements = document.get_all_by(&role::button()).unwrap();

            assert_eq!(matching_elements.len(), 2);
            matching_elements.iter().for_each(assert_has_role_button);
        }

        fn assert_has_role_button(element: &Element) {
            assert_eq!(
                element.as_ref().get_attribute("role"),
                Some("button".to_string())
            );
        }
    }
}
