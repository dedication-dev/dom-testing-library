use crate::dom::CSSSelector;

pub trait Queryable {
    type Element;

    fn query_all(&self, selectors: Vec<CSSSelector>) -> Vec<Self::Element>;
}

#[cfg(test)]
pub mod test_helper {
    use super::*;
    use crate::dom::test_helper::FakeElement;

    pub struct NonFilteringQueryable(pub Vec<FakeElement>);

    impl Queryable for NonFilteringQueryable {
        type Element = FakeElement;

        fn query_all(&self, _selectors: Vec<CSSSelector>) -> Vec<Self::Element> {
            self.0.to_vec()
        }
    }
}
