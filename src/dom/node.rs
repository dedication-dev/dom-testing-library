pub trait Node {
    fn attribute(&self, identifier: &AttributeIdentifier) -> Option<Attribute>;
}

#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq)]
pub enum Attribute {
    Valueless {
        identifier: AttributeIdentifier,
    },
    Value {
        identifier: AttributeIdentifier,
        value: AttributeValue,
    },
}

#[allow(dead_code)]
impl Attribute {
    pub fn identifier(&self) -> &AttributeIdentifier {
        match self {
            Attribute::Valueless { identifier } => identifier,
            Attribute::Value { identifier, .. } => identifier,
        }
    }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct AttributeIdentifier(pub String);

impl<T: Into<String>> From<T> for AttributeIdentifier {
    fn from(string_like: T) -> Self {
        AttributeIdentifier(string_like.into())
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct AttributeValue(String);
