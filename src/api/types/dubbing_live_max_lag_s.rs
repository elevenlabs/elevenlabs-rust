pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum DubbingLiveMaxLagS {
    #[default]
    Num10,
    Num12,
    Num15,
    Num18,
    Num20,
    Num25,
    Num30,
}

impl Serialize for DubbingLiveMaxLagS {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Num10 => serializer.serialize_i64(10),
            Self::Num12 => serializer.serialize_i64(12),
            Self::Num15 => serializer.serialize_i64(15),
            Self::Num18 => serializer.serialize_i64(18),
            Self::Num20 => serializer.serialize_i64(20),
            Self::Num25 => serializer.serialize_i64(25),
            Self::Num30 => serializer.serialize_i64(30),
        }
    }
}

impl<'de> Deserialize<'de> for DubbingLiveMaxLagS {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <i64 as serde::Deserialize>::deserialize(deserializer)?;
        if value == 10 {
            return Ok(Self::Num10);
        }
        if value == 12 {
            return Ok(Self::Num12);
        }
        if value == 15 {
            return Ok(Self::Num15);
        }
        if value == 18 {
            return Ok(Self::Num18);
        }
        if value == 20 {
            return Ok(Self::Num20);
        }
        if value == 25 {
            return Ok(Self::Num25);
        }
        if value == 30 {
            return Ok(Self::Num30);
        }
        Err(serde::de::Error::custom(format!(
            "unknown DubbingLiveMaxLagS value: {value}"
        )))
    }
}

impl fmt::Display for DubbingLiveMaxLagS {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Num10 => write!(f, "10"),
            Self::Num12 => write!(f, "12"),
            Self::Num15 => write!(f, "15"),
            Self::Num18 => write!(f, "18"),
            Self::Num20 => write!(f, "20"),
            Self::Num25 => write!(f, "25"),
            Self::Num30 => write!(f, "30"),
        }
    }
}
