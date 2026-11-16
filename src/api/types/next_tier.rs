pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum NextTier {
    Free,
    Starter,
    Go,
    Creator,
    Pro,
    GrowingBusiness,
    Scale2024_08_10,
    GrantTier1_2025_07_23,
    GrantTier2_2025_07_23,
    Grant,
    Trial,
    Enterprise,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for NextTier {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Free => serializer.serialize_str("free"),
            Self::Starter => serializer.serialize_str("starter"),
            Self::Go => serializer.serialize_str("go"),
            Self::Creator => serializer.serialize_str("creator"),
            Self::Pro => serializer.serialize_str("pro"),
            Self::GrowingBusiness => serializer.serialize_str("growing_business"),
            Self::Scale2024_08_10 => serializer.serialize_str("scale_2024_08_10"),
            Self::GrantTier1_2025_07_23 => serializer.serialize_str("grant_tier_1_2025_07_23"),
            Self::GrantTier2_2025_07_23 => serializer.serialize_str("grant_tier_2_2025_07_23"),
            Self::Grant => serializer.serialize_str("grant"),
            Self::Trial => serializer.serialize_str("trial"),
            Self::Enterprise => serializer.serialize_str("enterprise"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for NextTier {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "free" => Ok(Self::Free),
            "starter" => Ok(Self::Starter),
            "go" => Ok(Self::Go),
            "creator" => Ok(Self::Creator),
            "pro" => Ok(Self::Pro),
            "growing_business" => Ok(Self::GrowingBusiness),
            "scale_2024_08_10" => Ok(Self::Scale2024_08_10),
            "grant_tier_1_2025_07_23" => Ok(Self::GrantTier1_2025_07_23),
            "grant_tier_2_2025_07_23" => Ok(Self::GrantTier2_2025_07_23),
            "grant" => Ok(Self::Grant),
            "trial" => Ok(Self::Trial),
            "enterprise" => Ok(Self::Enterprise),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for NextTier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Free => write!(f, "free"),
            Self::Starter => write!(f, "starter"),
            Self::Go => write!(f, "go"),
            Self::Creator => write!(f, "creator"),
            Self::Pro => write!(f, "pro"),
            Self::GrowingBusiness => write!(f, "growing_business"),
            Self::Scale2024_08_10 => write!(f, "scale_2024_08_10"),
            Self::GrantTier1_2025_07_23 => write!(f, "grant_tier_1_2025_07_23"),
            Self::GrantTier2_2025_07_23 => write!(f, "grant_tier_2_2025_07_23"),
            Self::Grant => write!(f, "grant"),
            Self::Trial => write!(f, "trial"),
            Self::Enterprise => write!(f, "enterprise"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
