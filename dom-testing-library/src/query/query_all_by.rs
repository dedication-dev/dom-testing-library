use crate::dom::node::Node;
use crate::dom::queryable::Queryable;
use crate::query::matcher::Matcher;

pub trait QueryAllBy<TNode: Node> {
    fn query_all_by<TMatcher: Matcher>(&self, matcher: &TMatcher) -> Vec<TNode>;
}

impl<TNode: Node, TQueryable: Queryable<TNode>> QueryAllBy<TNode> for TQueryable {
    fn query_all_by<TMatcher: Matcher>(&self, matcher: &TMatcher) -> Vec<TNode> {
        self.query_all(matcher.css_selectors())
            .into_iter()
            .filter(|it| matcher.matches(it))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod query_all_by {
        use super::*;
        use crate::dom::css_selector::CSSSelector;
        use crate::dom::node::{Attribute, AttributeIdentifier, Node};
        use crate::dom::queryable::MockQueryable;
        use crate::query::matcher::MockMatcher;
        use std::collections::HashMap;

        #[test]
        fn with_query_all_without_nodes_returns_no_nodes() {
            let nodes = vec![];
            let queryable = non_filtering_queryable(&nodes);

            let matching_nodes = queryable.query_all_by(&matching_matcher());

            assert_eq!(matching_nodes, nodes);
        }

        #[test]
        fn returns_matching_nodes() {
            let nodes = vec![FakeNode::default()];
            let queryable = non_filtering_queryable(&nodes);

            let matching_nodes = queryable.query_all_by(&matching_matcher());

            assert_eq!(matching_nodes, nodes);
        }

        #[test]
        fn does_not_return_non_matching_nodes() {
            let other_attribute = Attribute::Valueless {
                identifier: "attr1".into(),
            };
            let node1 = FakeNode::new(AttributeMap::from(vec![other_attribute]));
            let matching_attribute = Attribute::Valueless {
                identifier: "attr2".into(),
            };
            let node2 = FakeNode::new(AttributeMap::from(vec![matching_attribute.clone()]));
            let queryable = non_filtering_queryable(&[node1, node2.clone()]);

            let matcher = AttributeMatcher::new(matching_attribute);
            let matching_nodes = queryable.query_all_by(&matcher);

            assert_eq!(matching_nodes, vec![node2]);
        }

        fn non_filtering_queryable(nodes: &[FakeNode]) -> MockQueryable<FakeNode> {
            let mut queryable = MockQueryable::new();
            queryable.expect_query_all().return_const(nodes.to_vec());
            queryable
        }

        #[derive(Clone, Debug, Default, PartialEq)]
        struct AttributeMap(HashMap<AttributeIdentifier, Attribute>);

        impl From<Vec<Attribute>> for AttributeMap {
            fn from(attributes: Vec<Attribute>) -> Self {
                Self(HashMap::from_iter(attributes.into_iter().map(
                    |attribute| (attribute.identifier().clone(), attribute),
                )))
            }
        }

        #[derive(Clone, Debug, Default, PartialEq)]
        struct FakeNode {
            attributes: AttributeMap,
        }

        impl FakeNode {
            fn new(attributes: AttributeMap) -> Self {
                Self { attributes }
            }
        }

        impl Node for FakeNode {
            fn attribute(&self, identifier: &AttributeIdentifier) -> Option<Attribute> {
                self.attributes.0.get(identifier).cloned()
            }
        }

        fn matching_matcher() -> MockMatcher {
            let mut matcher = MockMatcher::new();
            matcher.expect_css_selectors().return_const(vec![]);
            matcher.expect_matches().return_const(true);
            matcher
        }

        struct AttributeMatcher {
            attribute: Attribute,
        }

        impl AttributeMatcher {
            fn new(attribute: Attribute) -> Self {
                Self { attribute }
            }
        }

        impl Matcher for AttributeMatcher {
            fn css_selectors(&self) -> Vec<CSSSelector> {
                vec![]
            }

            fn matches(&self, node: &dyn Node) -> bool {
                match node.attribute(self.attribute.identifier()) {
                    None => false,
                    Some(attribute) => self.attribute == attribute,
                }
            }
        }
    }
}
