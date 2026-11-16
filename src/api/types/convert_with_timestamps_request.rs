pub use crate::prelude::*;

/// Request for ConvertWithTimestamps (body + query parameters)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ConvertWithTimestampsRequest {
    /// When enable_logging is set to false zero retention mode will be used for the request. This will mean history features are unavailable for this request, including request stitching. Zero retention mode may only be used by enterprise customers.
    #[serde(skip)]
    pub enable_logging: Option<bool>,
    /// You can turn on latency optimizations at some cost of quality. The best possible final latency varies by model. Possible values:
    /// 0 - default mode (no latency optimizations)
    /// 1 - normal latency optimizations (about 50% of possible latency improvement of option 3)
    /// 2 - strong latency optimizations (about 75% of possible latency improvement of option 3)
    /// 3 - max latency optimizations
    /// 4 - max latency optimizations, but also with text normalizer turned off for even more latency savings (best latency, but can mispronounce eg numbers and dates).
    ///
    /// Defaults to None.
    ///
    #[serde(skip)]
    pub optimize_streaming_latency: Option<i64>,
    /// Output format of the generated audio. Formatted as codec_sample_rate_bitrate. So an mp3 with 22.05kHz sample rate at 32kbs is represented as mp3_22050_32. MP3 with 192kbps bitrate requires you to be subscribed to Creator tier or above. PCM and WAV formats with 44.1kHz sample rate requires you to be subscribed to Pro tier or above. Note that the μ-law format (sometimes written mu-law, often approximated as u-law) is commonly used for Twilio audio inputs.
    #[serde(skip)]
    pub output_format: Option<ConvertOutputFormatQueryParameter1>,
    pub body: BodyTextToSpeechFullWithTimestamps,
}

impl ConvertWithTimestampsRequest {
    pub fn builder() -> ConvertWithTimestampsRequestBuilder {
        <ConvertWithTimestampsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConvertWithTimestampsRequestBuilder {
    enable_logging: Option<bool>,
    optimize_streaming_latency: Option<i64>,
    output_format: Option<ConvertOutputFormatQueryParameter1>,
    body: Option<BodyTextToSpeechFullWithTimestamps>,
}

impl ConvertWithTimestampsRequestBuilder {
    pub fn enable_logging(mut self, value: bool) -> Self {
        self.enable_logging = Some(value);
        self
    }

    pub fn optimize_streaming_latency(mut self, value: i64) -> Self {
        self.optimize_streaming_latency = Some(value);
        self
    }

    pub fn output_format(mut self, value: ConvertOutputFormatQueryParameter1) -> Self {
        self.output_format = Some(value);
        self
    }

    pub fn body(mut self, value: BodyTextToSpeechFullWithTimestamps) -> Self {
        self.body = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ConvertWithTimestampsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`body`](ConvertWithTimestampsRequestBuilder::body)
    pub fn build(self) -> Result<ConvertWithTimestampsRequest, BuildError> {
        Ok(ConvertWithTimestampsRequest {
            enable_logging: self.enable_logging,
            optimize_streaming_latency: self.optimize_streaming_latency,
            output_format: self.output_format,
            body: self.body.ok_or_else(|| BuildError::missing_field("body"))?,
        })
    }
}
