pub use crate::prelude::*;

/// Request for Remix (body + query parameters)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RemixRequest {
    /// Output format of the generated audio. Formatted as codec_sample_rate_bitrate. So an mp3 with 22.05kHz sample rate at 32kbs is represented as mp3_22050_32. MP3 with 192kbps bitrate requires you to be subscribed to Creator tier or above. PCM with 44.1kHz sample rate requires you to be subscribed to Pro tier or above. Note that the μ-law format (sometimes written mu-law, often approximated as u-law) is commonly used for Twilio audio inputs.
    #[serde(skip)]
    pub output_format: Option<AllowedOutputFormats>,
    pub body: VoiceRemixRequestModel,
}

impl RemixRequest {
    pub fn builder() -> RemixRequestBuilder {
        <RemixRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RemixRequestBuilder {
    output_format: Option<AllowedOutputFormats>,
    body: Option<VoiceRemixRequestModel>,
}

impl RemixRequestBuilder {
    pub fn output_format(mut self, value: AllowedOutputFormats) -> Self {
        self.output_format = Some(value);
        self
    }

    pub fn body(mut self, value: VoiceRemixRequestModel) -> Self {
        self.body = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RemixRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`body`](RemixRequestBuilder::body)
    pub fn build(self) -> Result<RemixRequest, BuildError> {
        Ok(RemixRequest {
            output_format: self.output_format,
            body: self.body.ok_or_else(|| BuildError::missing_field("body"))?,
        })
    }
}
