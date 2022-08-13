use crate::dom::element::Element;
use crate::dom::node_list::NodeList;
use dom_testing_library::{CSSSelector, Queryable};
use wasm_bindgen::JsCast;

pub struct Document(web_sys::Document);

impl Queryable<Element> for Document {
    fn query_all(&self, selectors: Vec<CSSSelector>) -> Vec<Element> {
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

impl From<web_sys::Document> for Document {
    fn from(document: web_sys::Document) -> Self {
        Self(document)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use dom_testing_library::CSSSelector;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    mod query_all_with_document {
        use super::*;
        use crate::util::test::ParseDocument;

        #[wasm_bindgen_test]
        fn returns_element_with_exact_attribute() {
            let body = r#"
                <div>
                    <a role="button"></a>
                </div>
            "#;

            let elements = query_all(
                body,
                vec![CSSSelector::exact_attribute_selector(
                    "role".into(),
                    "button".into(),
                )],
            );

            assert_eq!(elements.len(), 1);
            let element: &web_sys::Element = elements.first().unwrap().as_ref();
            assert_eq!(element.node_name(), "A");
        }

        fn query_all(body: &str, selectors: Vec<CSSSelector>) -> Vec<Element> {
            Document::from(body.parse_document()).query_all(selectors)
        }
    }
}
