pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum EntityManagementActivityId {
    #[default]
    Num0,
    Num1,
    Num2,
    Num3,
    Num4,
    Num5,
    Num6,
    Num7,
    Num8,
    Num9,
    Num10,
    Num11,
    Num12,
    Num13,
    Num99,
}

impl Serialize for EntityManagementActivityId {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Num0 => serializer.serialize_i64(0),
            Self::Num1 => serializer.serialize_i64(1),
            Self::Num2 => serializer.serialize_i64(2),
            Self::Num3 => serializer.serialize_i64(3),
            Self::Num4 => serializer.serialize_i64(4),
            Self::Num5 => serializer.serialize_i64(5),
            Self::Num6 => serializer.serialize_i64(6),
            Self::Num7 => serializer.serialize_i64(7),
            Self::Num8 => serializer.serialize_i64(8),
            Self::Num9 => serializer.serialize_i64(9),
            Self::Num10 => serializer.serialize_i64(10),
            Self::Num11 => serializer.serialize_i64(11),
            Self::Num12 => serializer.serialize_i64(12),
            Self::Num13 => serializer.serialize_i64(13),
            Self::Num99 => serializer.serialize_i64(99),
        }
    }
}

impl<'de> Deserialize<'de> for EntityManagementActivityId {
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
        if value == 5 {
            return Ok(Self::Num5);
        }
        if value == 6 {
            return Ok(Self::Num6);
        }
        if value == 7 {
            return Ok(Self::Num7);
        }
        if value == 8 {
            return Ok(Self::Num8);
        }
        if value == 9 {
            return Ok(Self::Num9);
        }
        if value == 10 {
            return Ok(Self::Num10);
        }
        if value == 11 {
            return Ok(Self::Num11);
        }
        if value == 12 {
            return Ok(Self::Num12);
        }
        if value == 13 {
            return Ok(Self::Num13);
        }
        if value == 99 {
            return Ok(Self::Num99);
        }
        Err(serde::de::Error::custom(format!(
            "unknown EntityManagementActivityId value: {value}"
        )))
    }
}

impl fmt::Display for EntityManagementActivityId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Num0 => write!(f, "0"),
            Self::Num1 => write!(f, "1"),
            Self::Num2 => write!(f, "2"),
            Self::Num3 => write!(f, "3"),
            Self::Num4 => write!(f, "4"),
            Self::Num5 => write!(f, "5"),
            Self::Num6 => write!(f, "6"),
            Self::Num7 => write!(f, "7"),
            Self::Num8 => write!(f, "8"),
            Self::Num9 => write!(f, "9"),
            Self::Num10 => write!(f, "10"),
            Self::Num11 => write!(f, "11"),
            Self::Num12 => write!(f, "12"),
            Self::Num13 => write!(f, "13"),
            Self::Num99 => write!(f, "99"),
        }
    }
}
