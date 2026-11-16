pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum UserTypeId {
    #[default]
    Num0,
    Num1,
    Num2,
    Num3,
    Num4,
    Num99,
}

impl Serialize for UserTypeId {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Num0 => serializer.serialize_i64(0),
            Self::Num1 => serializer.serialize_i64(1),
            Self::Num2 => serializer.serialize_i64(2),
            Self::Num3 => serializer.serialize_i64(3),
            Self::Num4 => serializer.serialize_i64(4),
            Self::Num99 => serializer.serialize_i64(99),
        }
    }
}

impl<'de> Deserialize<'de> for UserTypeId {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <i64 as serde::Deserialize>::deserialize(deserializer)?;
        if value == 0 {
            return Ok(Self::Num0);
        }
        if value == 1 {
            return Ok(Self::Num1);
        }
        if value == 2 {
            return Ok(Self::Num2);
        }
        if value == 3 {
            return Ok(Self::Num3);
        }
        if value == 4 {
            return Ok(Self::Num4);
        }
        if value == 99 {
            return Ok(Self::Num99);
        }
        Err(serde::de::Error::custom(format!(
            "unknown UserTypeId value: {value}"
        )))
    }
}

impl fmt::Display for UserTypeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Num0 => write!(f, "0"),
            Self::Num1 => write!(f, "1"),
            Self::Num2 => write!(f, "2"),
            Self::Num3 => write!(f, "3"),
            Self::Num4 => write!(f, "4"),
            Self::Num99 => write!(f, "99"),
        }
    }
}
