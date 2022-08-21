use crate::dom::attribute::{AttributeIdentifier, AttributeValue};

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
    Valueless {
        identifier: AttributeIdentifier,
    },
    Exact {
        identifier: AttributeIdentifier,
        value: AttributeValue,
    },
}

impl CSSSelector {
    pub fn exact_attribute_selector(
        identifier: AttributeIdentifier,
        value: AttributeValue,
    ) -> Self {
        Self::AttributeSelector(AttributeSelector::Exact { identifier, value })
    }
}

impl From<AttributeSelector> for String {
    fn from(attribute_selector: AttributeSelector) -> Self {
        match attribute_selector {
            AttributeSelector::Valueless { identifier } => format!("[{}]", identifier.as_ref()),
            AttributeSelector::Exact { identifier, value } => {
                format!("[{}={}]", identifier.as_ref(), value.as_ref())
            }
        }
    }
}
