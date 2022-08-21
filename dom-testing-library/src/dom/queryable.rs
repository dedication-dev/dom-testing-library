use crate::dom::{CSSSelector, Element};

pub trait Queryable<TElement: Element> {
    fn query_all(&self, selectors: Vec<CSSSelector>) -> Vec<TElement>;
}

#[cfg(test)]
pub mod test_helper {
    use super::*;
    use crate::dom::test_helper::FakeElement;

    pub struct NonFilteringQueryable(pub Vec<FakeElement>);

    impl Queryable<FakeElement> for NonFilteringQueryable {
        fn query_all(&self, _selectors: Vec<CSSSelector>) -> Vec<FakeElement> {
            self.0.to_vec()
        }
    }
}
