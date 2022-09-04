use crate::dom::NodeList;
use dom_testing_library::dom::{
    Attribute, AttributeIdentifier, CSSSelector, Element as TLElement, Queryable,
};
use wasm_bindgen::JsCast;

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

impl Queryable for Element {
    type Element = Element;

    fn query_all(&self, selectors: Vec<CSSSelector>) -> Vec<Self::Element> {
        let selectors = to_selectors_string(selectors);
        let elements: NodeList = self.0.query_selector_all(&selectors).unwrap().into();

        elements
            .into_iter()
            .map(|node| node.dyn_into::<web_sys::Element>().unwrap().into())
            .collect()
    }
}

fn to_selectors_string(selectors: Vec<CSSSelector>) -> String {
    selectors
        .into_iter()
        .map(|selector| selector.into())
        .collect::<Vec<String>>()
        .join(" ")
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
    use crate::render_html;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    mod attribute {
        use super::*;

        #[wasm_bindgen_test]
        fn without_attribute_returns_none() {
            let element = get_element(r#"<button></button>"#);

            let attribute = element.attribute(&"role".into());

            assert_eq!(attribute, None);
        }

        #[wasm_bindgen_test]
        fn returns_attribute() {
            let element = get_element(r#"<button id="element"></button>"#);

            let attribute = element.attribute(&"id".into());

            assert_eq!(attribute, Some(Attribute::value("id", "element")));
        }

        #[wasm_bindgen_test]
        fn with_valueless_attribute_returns_valueless_attribute() {
            let element = get_element(r#"<button disabled></button>"#);

            let attribute = element.attribute(&"disabled".into());

            assert_eq!(attribute, Some(Attribute::valueless("disabled")));
        }

        fn get_element(body: &str) -> Element {
            Element(
                render_html(body)
                    .into_inner()
                    .first_element_child()
                    .unwrap(),
            )
        }
    }

    mod query_all {
        use super::*;

        #[wasm_bindgen_test]
        fn returns_element_with_exact_attribute() {
            let body = r#"
                <div>
                    <a role="button"></a>
                </div>
            "#;

            let elements = query_all(
                body,
                vec![CSSSelector::exact_attribute("role".into(), "button".into())],
            );

            assert_eq!(elements.len(), 1);
            let element: &web_sys::Element = elements.first().unwrap().as_ref();
            assert_eq!(element.node_name(), "A");
        }

        fn query_all(body: &str, selectors: Vec<CSSSelector>) -> Vec<Element> {
            render_html(body).query_all(selectors)
        }
    }
}
