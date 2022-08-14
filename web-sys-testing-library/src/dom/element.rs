use dom_testing_library::dom::element::{Attribute, AttributeIdentifier, Element as TLElement};

pub struct Element(web_sys::Element);

impl TLElement for Element {
    fn attribute(&self, identifier: &AttributeIdentifier) -> Option<Attribute> {
        self.0.get_attribute(identifier.as_ref()).map(|value| {
            if value.is_empty() {
                Attribute::valueless(identifier.clone())
            } else {
                Attribute::value(identifier.clone(), value.into())
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

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    mod attribute {
        use super::*;
        use crate::util::test::ParseDocument;

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

            assert_eq!(
                attribute,
                Some(Attribute::value("id".into(), "element".into()))
            );
        }

        #[wasm_bindgen_test]
        fn with_valueless_attribute_returns_valueless_attribute() {
            let element = get_element(r#"<button id="element" disabled></button>"#, "element");

            let attribute = element.attribute(&"disabled".into());

            assert_eq!(attribute, Some(Attribute::valueless("disabled".into())));
        }

        fn get_element(body: &str, element_id: &str) -> Element {
            Element(body.parse_document().get_element_by_id(element_id).unwrap())
        }
    }
}
