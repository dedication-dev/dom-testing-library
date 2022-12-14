use std::fmt;
use Attribute::{Value, Valueless};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Attribute {
    Valueless {
        identifier: AttributeIdentifier,
    },
    Value {
        identifier: AttributeIdentifier,
        value: AttributeValue,
    },
}

impl Attribute {
    pub fn valueless<TAttributeIdentifier: Into<AttributeIdentifier>>(
        identifier: TAttributeIdentifier,
    ) -> Self {
        Valueless {
            identifier: identifier.into(),
        }
    }

    pub fn value<TId: Into<AttributeIdentifier>, TVal: Into<AttributeValue>>(
        identifier: TId,
        value: TVal,
    ) -> Self {
        Value {
            identifier: identifier.into(),
            value: value.into(),
        }
    }

    pub fn identifier(&self) -> &AttributeIdentifier {
        match self {
            Valueless { identifier } => identifier,
            Value { identifier, .. } => identifier,
        }
    }
}

impl fmt::Display for Attribute {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Valueless { identifier } => write!(f, "{}", identifier),
            Value { identifier, value } => write!(f, "{}={}", identifier, value),
        }
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct AttributeIdentifier(String);

impl AttributeIdentifier {
    pub fn into_inner(self) -> String {
        self.0
    }
}

impl fmt::Display for AttributeIdentifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0.clone())
    }
}

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

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AttributeValue(String);

impl AttributeValue {
    pub fn into_inner(self) -> String {
        self.0
    }
}

impl fmt::Display for AttributeValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0.clone())
    }
}

impl AsRef<String> for AttributeValue {
    fn as_ref(&self) -> &String {
        &self.0
    }
}

impl<T: Into<String>> From<T> for AttributeValue {
    fn from(string_like: T) -> Self {
        AttributeValue(string_like.into())
    }
}

#[cfg(test)]
pub mod test_helper {
    use super::*;
    use std::collections::HashMap;

    #[derive(Clone, Debug, Default, PartialEq, Eq)]
    pub struct AttributeMap(pub HashMap<AttributeIdentifier, Attribute>);

    impl From<Vec<Attribute>> for AttributeMap {
        fn from(attributes: Vec<Attribute>) -> Self {
            Self(HashMap::from_iter(attributes.into_iter().map(
                |attribute| (attribute.identifier().clone(), attribute),
            )))
        }
    }
}
