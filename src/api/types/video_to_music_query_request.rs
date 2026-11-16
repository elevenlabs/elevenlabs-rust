pub use crate::prelude::*;

/// Query parameters for Video To Music
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct VideoToMusicQueryRequest {
    /// Output format of the generated audio. Formatted as codec_sample_rate_bitrate. So an mp3 with 22.05kHz sample rate at 32kbs is represented as mp3_22050_32. MP3 with 192kbps bitrate requires you to be subscribed to Creator tier or above. PCM with 44.1kHz sample rate requires you to be subscribed to Pro tier or above. Note that the μ-law format (sometimes written mu-law, often approximated as u-law) is commonly used for Twilio audio inputs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_format: Option<AllowedOutputFormats>,
    /// When enable_logging is set to false zero retention mode will be used for the request. Zero retention mode may only be used by enterprise customers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_logging: Option<bool>,
}

impl VideoToMusicQueryRequest {
    pub fn builder() -> VideoToMusicQueryRequestBuilder {
        <VideoToMusicQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct VideoToMusicQueryRequestBuilder {
    output_format: Option<AllowedOutputFormats>,
    enable_logging: Option<bool>,
}

impl VideoToMusicQueryRequestBuilder {
    pub fn output_format(mut self, value: AllowedOutputFormats) -> Self {
        self.output_format = Some(value);
        self
    }

    pub fn enable_logging(mut self, value: bool) -> Self {
        self.enable_logging = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`VideoToMusicQueryRequest`].
    pub fn build(self) -> Result<VideoToMusicQueryRequest, BuildError> {
        Ok(VideoToMusicQueryRequest {
            output_format: self.output_format,
            enable_logging: self.enable_logging,
        })
    }
}
