use crate::dom::attribute::{AttributeIdentifier, AttributeValue};

#[derive(Clone, Debug, PartialEq)]
pub enum CSSSelector {
    ValuelessAttribute {
        identifier: AttributeIdentifier,
    },
    ExactAttribute {
        identifier: AttributeIdentifier,
        value: AttributeValue,
    },
}

impl CSSSelector {
    pub fn exact_attribute(identifier: AttributeIdentifier, value: AttributeValue) -> Self {
        Self::ExactAttribute { identifier, value }
    }
}

impl From<CSSSelector> for String {
    fn from(css_selector: CSSSelector) -> Self {
        match css_selector {
            CSSSelector::ValuelessAttribute { identifier } => format!("[{}]", identifier.as_ref()),
            CSSSelector::ExactAttribute { identifier, value } => {
                format!("[{}={}]", identifier.as_ref(), value.as_ref())
            }
        }
    }
}
