pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DubbingLanguageResponseVoiceSettings {
    /// How strongly the dubbed speakers clone the source voices, 0 to 10.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::int::option")]
    pub cloning_strength: Option<i64>,
}

impl DubbingLanguageResponseVoiceSettings {
    pub fn builder() -> DubbingLanguageResponseVoiceSettingsBuilder {
        <DubbingLanguageResponseVoiceSettingsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DubbingLanguageResponseVoiceSettingsBuilder {
    cloning_strength: Option<i64>,
}

impl DubbingLanguageResponseVoiceSettingsBuilder {
    pub fn cloning_strength(mut self, value: i64) -> Self {
        self.cloning_strength = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DubbingLanguageResponseVoiceSettings`].
    pub fn build(self) -> Result<DubbingLanguageResponseVoiceSettings, BuildError> {
        Ok(DubbingLanguageResponseVoiceSettings {
            cloning_strength: self.cloning_strength,
        })
    }
}
