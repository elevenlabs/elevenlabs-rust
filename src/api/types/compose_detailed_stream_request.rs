pub use crate::prelude::*;

/// Request for ComposeDetailedStream (body + query parameters)
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ComposeDetailedStreamRequest {
    /// Output format of the generated audio. Formatted as codec_sample_rate_bitrate. Use "auto" (the default) to let the API pick the best format for the selected model: mp3_44100_128 for v1 models and mp3_48000_192 for v2 models.
    #[serde(skip)]
    pub output_format: Option<ComposeDetailedOutputFormat>,
    /// When enable_logging is set to false zero retention mode will be used for the request. Zero retention mode may only be used by enterprise customers.
    #[serde(skip)]
    pub enable_logging: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<BodyStreamComposedMusicWithADetailedResponseV1MusicDetailedStreamPost>,
}

impl ComposeDetailedStreamRequest {
    pub fn builder() -> ComposeDetailedStreamRequestBuilder {
        <ComposeDetailedStreamRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ComposeDetailedStreamRequestBuilder {
    output_format: Option<ComposeDetailedOutputFormat>,
    enable_logging: Option<bool>,
    body: Option<BodyStreamComposedMusicWithADetailedResponseV1MusicDetailedStreamPost>,
}

impl ComposeDetailedStreamRequestBuilder {
    pub fn output_format(mut self, value: ComposeDetailedOutputFormat) -> Self {
        self.output_format = Some(value);
        self
    }

    pub fn enable_logging(mut self, value: bool) -> Self {
        self.enable_logging = Some(value);
        self
    }

    pub fn body(
        mut self,
        value: BodyStreamComposedMusicWithADetailedResponseV1MusicDetailedStreamPost,
    ) -> Self {
        self.body = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ComposeDetailedStreamRequest`].
    pub fn build(self) -> Result<ComposeDetailedStreamRequest, BuildError> {
        Ok(ComposeDetailedStreamRequest {
            output_format: self.output_format,
            enable_logging: self.enable_logging,
            body: self.body,
        })
    }
}
