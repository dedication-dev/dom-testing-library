use crate::dom::node::AttributeIdentifier;

#[derive(Clone, Debug, PartialEq)]
pub enum CSSSelector {
    AttributeSelector(AttributeSelector),
}

impl From<CSSSelector> for String {
    fn from(css_selector: CSSSelector) -> Self {
        match css_selector {
            CSSSelector::AttributeSelector(value) => value.into(),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum AttributeSelector {
    Valueless { identifier: AttributeIdentifier },
}

impl From<AttributeSelector> for String {
    fn from(attribute_selector: AttributeSelector) -> Self {
        match attribute_selector {
            AttributeSelector::Valueless { identifier } => format!("[{}]", identifier.0),
        }
    }
}
