pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct BodyPostAgentHoldAudioV1ConvaiAgentsAgentIdHoldAudioPost {
    /// An MP3 or WAV file played on loop to callers waiting in the agent's concurrency wait queue. Maximum size 40 MB, maximum duration 180 seconds.
    #[serde(default)]
    pub hold_audio_file: Vec<u8>,
    /// The name sent with the file; when unset the part is named after its field.
    #[serde(skip)]
    pub hold_audio_file_file_name: Option<String>,
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
    hold_audio_file_file_name: Option<String>,
}

impl BodyPostAgentHoldAudioV1ConvaiAgentsAgentIdHoldAudioPostBuilder {
    pub fn hold_audio_file(mut self, value: Vec<u8>) -> Self {
        self.hold_audio_file = Some(value);
        self
    }

    pub fn hold_audio_file_file_name(mut self, value: impl Into<String>) -> Self {
        self.hold_audio_file_file_name = Some(value.into());
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
            hold_audio_file_file_name: self.hold_audio_file_file_name,
        })
    }
}

impl BodyPostAgentHoldAudioV1ConvaiAgentsAgentIdHoldAudioPost {
    pub fn to_multipart(self) -> reqwest::multipart::Form {
        let mut form = reqwest::multipart::Form::new();
        form = form.part("hold_audio_file", {
            let file_name = self
                .hold_audio_file_file_name
                .unwrap_or_else(|| "hold_audio_file".to_string());
            let mime = mime_guess::from_path(&file_name).first_or_octet_stream();
            reqwest::multipart::Part::bytes(self.hold_audio_file)
                .file_name(file_name)
                .mime_str(mime.as_ref())
                .expect("a guessed MIME type is valid")
        });
        form
    }
}
