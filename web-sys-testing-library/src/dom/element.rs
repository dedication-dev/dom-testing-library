use dom_testing_library::dom::{Attribute, AttributeIdentifier, Element as TLElement};

/// Wraps [web_sys::Element].
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Element(web_sys::Element);

impl Element {
    /// Extracts the underlying [web_sys::Element].
    pub fn into_inner(self) -> web_sys::Element {
        self.0
    }
}

impl TLElement for Element {
    fn attribute(&self, identifier: &AttributeIdentifier) -> Option<Attribute> {
        self.0.get_attribute(identifier.as_ref()).map(|value| {
            if value.is_empty() {
                Attribute::valueless(identifier.clone())
            } else {
                Attribute::value(identifier.clone(), value)
            }
        })
    }
}

impl From<web_sys::Element> for Element {
    fn from(element: web_sys::Element) -> Self {
        Self(element)
    }
}

impl AsRef<web_sys::Element> for Element {
    fn as_ref(&self) -> &web_sys::Element {
        &self.0
    }
}

impl From<Element> for web_sys::Element {
    fn from(element: Element) -> Self {
        element.0
    }
}

/// Extension trait for [Vec<Element>].
pub trait VecElementExt {
    /// Turns [Self] into [Vec<web_sys::Element>].
    fn into_web_sys_elements(self) -> Vec<web_sys::Element>;
}

impl VecElementExt for Vec<Element> {
    fn into_web_sys_elements(self) -> Vec<web_sys::Element> {
        self.into_iter()
            .map(|element| element.into_inner())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render_html;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    mod attribute {
        use super::*;

        #[wasm_bindgen_test]
        fn without_attribute_returns_none() {
            let element = get_element(r#"<button id="element"></button>"#, "element");

            let attribute = element.attribute(&"role".into());

            assert_eq!(attribute, None);
        }

        #[wasm_bindgen_test]
        fn returns_attribute() {
            let element = get_element(r#"<button id="element"></button>"#, "element");

            let attribute = element.attribute(&"id".into());

            assert_eq!(attribute, Some(Attribute::value("id", "element")));
        }

        #[wasm_bindgen_test]
        fn with_valueless_attribute_returns_valueless_attribute() {
            let element = get_element(r#"<button id="element" disabled></button>"#, "element");

            let attribute = element.attribute(&"disabled".into());

            assert_eq!(attribute, Some(Attribute::valueless("disabled")));
        }

        fn get_element(body: &str, element_id: &str) -> Element {
            Element(
                render_html(body)
                    .into_inner()
                    .get_element_by_id(element_id)
                    .unwrap(),
            )
        }
    }
}
