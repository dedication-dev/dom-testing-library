use crate::dom::{CSSSelector, Element};

#[cfg_attr(test, mockall::automock)]
pub trait Queryable<TElement: Element> {
    fn query_all(&self, selectors: Vec<CSSSelector>) -> Vec<TElement>;
}

#[cfg(test)]
pub mod test_helper {
    use super::*;
    use crate::dom::test_helper::FakeElement;

    pub fn non_filtering_queryable(elements: &[FakeElement]) -> MockQueryable<FakeElement> {
        let mut queryable = MockQueryable::new();
        queryable.expect_query_all().return_const(elements.to_vec());
        queryable
    }
}
