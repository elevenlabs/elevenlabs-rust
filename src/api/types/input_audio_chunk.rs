pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct InputAudioChunk {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_type: Option<String>,
    #[serde(default)]
    pub audio_base_64: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commit: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_text: Option<String>,
    /// A library of terms to recognize more accurately, supported by Scribe v3 Live models. Only set this on the first audio chunk, not as a query parameter. At most 10,000 terms are allowed, each at most 50 characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keyterm_library: Option<Vec<String>>,
}

impl InputAudioChunk {
    pub fn builder() -> InputAudioChunkBuilder {
        <InputAudioChunkBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InputAudioChunkBuilder {
    message_type: Option<String>,
    audio_base_64: Option<String>,
    commit: Option<bool>,
    previous_text: Option<String>,
    keyterm_library: Option<Vec<String>>,
}

impl InputAudioChunkBuilder {
    pub fn message_type(mut self, value: impl Into<String>) -> Self {
        self.message_type = Some(value.into());
        self
    }

    pub fn audio_base_64(mut self, value: impl Into<String>) -> Self {
        self.audio_base_64 = Some(value.into());
        self
    }

    pub fn commit(mut self, value: bool) -> Self {
        self.commit = Some(value);
        self
    }

    pub fn previous_text(mut self, value: impl Into<String>) -> Self {
        self.previous_text = Some(value.into());
        self
    }

    pub fn keyterm_library(mut self, value: Vec<String>) -> Self {
        self.keyterm_library = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`InputAudioChunk`].
    /// This method will fail if any of the following fields are not set:
    /// - [`audio_base_64`](InputAudioChunkBuilder::audio_base_64)
    pub fn build(self) -> Result<InputAudioChunk, BuildError> {
        Ok(InputAudioChunk {
            message_type: self.message_type,
            audio_base_64: self
                .audio_base_64
                .ok_or_else(|| BuildError::missing_field("audio_base_64"))?,
            commit: self.commit,
            previous_text: self.previous_text,
            keyterm_library: self.keyterm_library,
        })
    }
}
