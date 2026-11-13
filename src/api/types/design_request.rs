pub use crate::prelude::*;

/// Request for Design (body + query parameters)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DesignRequest {
    /// Output format of the generated audio. Formatted as codec_sample_rate_bitrate. So an mp3 with 22.05kHz sample rate at 32kbs is represented as mp3_22050_32. MP3 with 192kbps bitrate requires you to be subscribed to Creator tier or above. PCM with 44.1kHz sample rate requires you to be subscribed to Pro tier or above. Note that the μ-law format (sometimes written mu-law, often approximated as u-law) is commonly used for Twilio audio inputs.
    #[serde(skip)]
    pub output_format: Option<AllowedOutputFormats>,
    pub body: VoiceDesignRequestModel,
}

impl DesignRequest {
    pub fn builder() -> DesignRequestBuilder {
        <DesignRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DesignRequestBuilder {
    output_format: Option<AllowedOutputFormats>,
    body: Option<VoiceDesignRequestModel>,
}

impl DesignRequestBuilder {
    pub fn output_format(mut self, value: AllowedOutputFormats) -> Self {
        self.output_format = Some(value);
        self
    }

    pub fn body(mut self, value: VoiceDesignRequestModel) -> Self {
        self.body = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DesignRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`body`](DesignRequestBuilder::body)
    pub fn build(self) -> Result<DesignRequest, BuildError> {
        Ok(DesignRequest {
            output_format: self.output_format,
            body: self.body.ok_or_else(|| BuildError::missing_field("body"))?,
        })
    }
}
