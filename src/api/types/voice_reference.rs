pub use crate::prelude::*;

/// A voice this workspace can use.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct VoiceReference {
    /// The ID of a voice in this workspace.
    #[serde(default)]
    pub voice_id: String,
}

impl VoiceReference {
    pub fn builder() -> VoiceReferenceBuilder {
        <VoiceReferenceBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct VoiceReferenceBuilder {
    voice_id: Option<String>,
}

impl VoiceReferenceBuilder {
    pub fn voice_id(mut self, value: impl Into<String>) -> Self {
        self.voice_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`VoiceReference`].
    /// This method will fail if any of the following fields are not set:
    /// - [`voice_id`](VoiceReferenceBuilder::voice_id)
    pub fn build(self) -> Result<VoiceReference, BuildError> {
        Ok(VoiceReference {
            voice_id: self
                .voice_id
                .ok_or_else(|| BuildError::missing_field("voice_id"))?,
        })
    }
}
