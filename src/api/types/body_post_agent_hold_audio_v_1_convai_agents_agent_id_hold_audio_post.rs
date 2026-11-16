pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct BodyPostAgentHoldAudioV1ConvaiAgentsAgentIdHoldAudioPost {
    /// An MP3 or WAV file played on loop to callers waiting in the agent's concurrency wait queue. Maximum size 40 MB, maximum duration 180 seconds.
    #[serde(default)]
    pub hold_audio_file: Vec<u8>,
}

impl BodyPostAgentHoldAudioV1ConvaiAgentsAgentIdHoldAudioPost {
    pub fn builder() -> BodyPostAgentHoldAudioV1ConvaiAgentsAgentIdHoldAudioPostBuilder {
        <BodyPostAgentHoldAudioV1ConvaiAgentsAgentIdHoldAudioPostBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BodyPostAgentHoldAudioV1ConvaiAgentsAgentIdHoldAudioPostBuilder {
    hold_audio_file: Option<Vec<u8>>,
}

impl BodyPostAgentHoldAudioV1ConvaiAgentsAgentIdHoldAudioPostBuilder {
    pub fn hold_audio_file(mut self, value: Vec<u8>) -> Self {
        self.hold_audio_file = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BodyPostAgentHoldAudioV1ConvaiAgentsAgentIdHoldAudioPost`].
    /// This method will fail if any of the following fields are not set:
    /// - [`hold_audio_file`](BodyPostAgentHoldAudioV1ConvaiAgentsAgentIdHoldAudioPostBuilder::hold_audio_file)
    pub fn build(
        self,
    ) -> Result<BodyPostAgentHoldAudioV1ConvaiAgentsAgentIdHoldAudioPost, BuildError> {
        Ok(BodyPostAgentHoldAudioV1ConvaiAgentsAgentIdHoldAudioPost {
            hold_audio_file: self
                .hold_audio_file
                .ok_or_else(|| BuildError::missing_field("hold_audio_file"))?,
        })
    }
}

impl BodyPostAgentHoldAudioV1ConvaiAgentsAgentIdHoldAudioPost {
    pub fn to_multipart(self) -> reqwest::multipart::Form {
        let mut form = reqwest::multipart::Form::new();
        form = form.part(
            "hold_audio_file",
            reqwest::multipart::Part::bytes(self.hold_audio_file)
                .file_name("hold_audio_file")
                .mime_str("application/octet-stream")
                .expect("application/octet-stream is a valid MIME type"),
        );
        form
    }
}
