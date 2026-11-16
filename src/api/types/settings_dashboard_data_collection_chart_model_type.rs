pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DashboardDataCollectionChartModelType {
    DataCollection,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for DashboardDataCollectionChartModelType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::DataCollection => serializer.serialize_str("data_collection"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for DashboardDataCollectionChartModelType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "data_collection" => Ok(Self::DataCollection),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for DashboardDataCollectionChartModelType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DataCollection => write!(f, "data_collection"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
