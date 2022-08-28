use crate::dom::{Attribute, AttributeIdentifier};

pub trait Element {
    fn attribute(&self, identifier: &AttributeIdentifier) -> Option<Attribute>;
}

#[cfg(test)]
pub mod test_helper {
    use super::*;
    use crate::dom::test_helper::AttributeMap;

    #[derive(Clone, Debug, Default, PartialEq, Eq)]
    pub struct FakeElement {
        attributes: AttributeMap,
    }

    impl FakeElement {
        pub fn new(attributes: AttributeMap) -> Self {
            Self { attributes }
        }
    }

    impl Element for FakeElement {
        fn attribute(&self, identifier: &AttributeIdentifier) -> Option<Attribute> {
            self.attributes.0.get(identifier).cloned()
        }
    }
}
