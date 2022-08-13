pub trait Element {
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
    pub fn valueless(identifier: AttributeIdentifier) -> Self {
        Self::Valueless { identifier }
    }

    pub fn value(identifier: AttributeIdentifier, value: AttributeValue) -> Self {
        Self::Value { identifier, value }
    }

    pub fn identifier(&self) -> &AttributeIdentifier {
        match self {
            Attribute::Valueless { identifier } => identifier,
            Attribute::Value { identifier, .. } => identifier,
        }
    }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct AttributeIdentifier(pub String);

impl AsRef<String> for AttributeIdentifier {
    fn as_ref(&self) -> &String {
        &self.0
    }
}

impl<T: Into<String>> From<T> for AttributeIdentifier {
    fn from(string_like: T) -> Self {
        AttributeIdentifier(string_like.into())
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct AttributeValue(pub String);

impl<T: Into<String>> From<T> for AttributeValue {
    fn from(string_like: T) -> Self {
        AttributeValue(string_like.into())
    }
}
