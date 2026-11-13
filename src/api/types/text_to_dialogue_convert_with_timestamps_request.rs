pub use crate::prelude::*;

/// Request for ConvertWithTimestamps (body + query parameters)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TextToDialogueConvertWithTimestampsRequest {
    /// Output format of the generated audio. Formatted as codec_sample_rate_bitrate. So an mp3 with 22.05kHz sample rate at 32kbs is represented as mp3_22050_32. MP3 with 192kbps bitrate requires you to be subscribed to Creator tier or above. PCM and WAV formats with 44.1kHz sample rate requires you to be subscribed to Pro tier or above. Note that the μ-law format (sometimes written mu-law, often approximated as u-law) is commonly used for Twilio audio inputs.
    #[serde(skip)]
    pub output_format: Option<ConvertOutputFormatQueryParameter1>,
    /// When enable_logging is set to false zero retention mode will be used for the request. This will mean history features are unavailable for this request, including request stitching. Zero retention mode may only be used by enterprise customers.
    #[serde(skip)]
    pub enable_logging: Option<bool>,
    pub body: BodyTextToDialogueFullWithTimestamps,
}

impl TextToDialogueConvertWithTimestampsRequest {
    pub fn builder() -> TextToDialogueConvertWithTimestampsRequestBuilder {
        <TextToDialogueConvertWithTimestampsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TextToDialogueConvertWithTimestampsRequestBuilder {
    output_format: Option<ConvertOutputFormatQueryParameter1>,
    enable_logging: Option<bool>,
    body: Option<BodyTextToDialogueFullWithTimestamps>,
}

impl TextToDialogueConvertWithTimestampsRequestBuilder {
    pub fn output_format(mut self, value: ConvertOutputFormatQueryParameter1) -> Self {
        self.output_format = Some(value);
        self
    }

    pub fn enable_logging(mut self, value: bool) -> Self {
        self.enable_logging = Some(value);
        self
    }

    pub fn body(mut self, value: BodyTextToDialogueFullWithTimestamps) -> Self {
        self.body = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TextToDialogueConvertWithTimestampsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`body`](TextToDialogueConvertWithTimestampsRequestBuilder::body)
    pub fn build(self) -> Result<TextToDialogueConvertWithTimestampsRequest, BuildError> {
        Ok(TextToDialogueConvertWithTimestampsRequest {
            output_format: self.output_format,
            enable_logging: self.enable_logging,
            body: self.body.ok_or_else(|| BuildError::missing_field("body"))?,
        })
    }
}
