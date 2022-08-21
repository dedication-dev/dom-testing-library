use crate::dom::{Element, Queryable};
use crate::query::matcher::Matcher;

pub trait QueryAllBy<TElement: Element> {
    fn query_all_by<TMatcher: Matcher>(&self, matcher: &TMatcher) -> Vec<TElement>;
}

impl<TElement: Element, TQueryable: Queryable<Element = TElement>> QueryAllBy<TElement>
    for TQueryable
{
    fn query_all_by<TMatcher: Matcher>(&self, matcher: &TMatcher) -> Vec<TElement> {
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
        use crate::dom::test_helper::{AttributeMap, FakeElement, NonFilteringQueryable};
        use crate::dom::Attribute;
        use crate::query::matcher::test_helper::{AttributeMatcher, MatchingMatcher};

        #[test]
        fn with_query_all_without_elements_returns_no_elements() {
            let elements = vec![];
            let queryable = NonFilteringQueryable(elements.clone());

            let matching_elements = queryable.query_all_by(&MatchingMatcher);

            assert_eq!(matching_elements, elements);
        }

        #[test]
        fn returns_matching_elements() {
            let elements = vec![FakeElement::default()];
            let queryable = NonFilteringQueryable(elements.clone());

            let matching_elements = queryable.query_all_by(&MatchingMatcher);

            assert_eq!(matching_elements, elements);
        }

        #[test]
        fn does_not_return_non_matching_elements() {
            let other_attribute = Attribute::Valueless {
                identifier: "attr1".into(),
            };
            let element1 = FakeElement::new(AttributeMap::from(vec![other_attribute]));
            let matching_attribute = Attribute::Valueless {
                identifier: "attr2".into(),
            };
            let element2 = FakeElement::new(AttributeMap::from(vec![matching_attribute.clone()]));
            let queryable = NonFilteringQueryable(vec![element1, element2.clone()]);

            let matcher = AttributeMatcher::new(matching_attribute);
            let matching_elements = queryable.query_all_by(&matcher);

            assert_eq!(matching_elements, vec![element2]);
        }
    }
}
