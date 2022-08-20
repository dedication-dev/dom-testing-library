use Attribute::{Value, Valueless};

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

#[cfg(test)]
pub mod test_helper {
    use super::*;
    use std::collections::HashMap;

    #[derive(Clone, Debug, Default, PartialEq)]
    pub struct AttributeMap(pub HashMap<AttributeIdentifier, Attribute>);

    impl From<Vec<Attribute>> for AttributeMap {
        fn from(attributes: Vec<Attribute>) -> Self {
            Self(HashMap::from_iter(attributes.into_iter().map(
                |attribute| (attribute.identifier().clone(), attribute),
            )))
        }
    }
}
