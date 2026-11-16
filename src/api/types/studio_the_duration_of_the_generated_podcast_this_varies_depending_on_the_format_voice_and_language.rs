pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TheDurationOfTheGeneratedPodcastThisVariesDependingOnTheFormatVoiceAndLanguage {
    Short,
    Default,
    Long,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for TheDurationOfTheGeneratedPodcastThisVariesDependingOnTheFormatVoiceAndLanguage {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Short => serializer.serialize_str("short"),
            Self::Default => serializer.serialize_str("default"),
            Self::Long => serializer.serialize_str("long"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de>
    for TheDurationOfTheGeneratedPodcastThisVariesDependingOnTheFormatVoiceAndLanguage
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "short" => Ok(Self::Short),
            "default" => Ok(Self::Default),
            "long" => Ok(Self::Long),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display
    for TheDurationOfTheGeneratedPodcastThisVariesDependingOnTheFormatVoiceAndLanguage
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Short => write!(f, "short"),
            Self::Default => write!(f, "default"),
            Self::Long => write!(f, "long"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
