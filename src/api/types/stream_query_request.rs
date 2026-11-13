pub use crate::prelude::*;

/// Query parameters for Voice changer stream
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct StreamQueryRequest {
    /// When enable_logging is set to false zero retention mode will be used for the request. This will mean history features are unavailable for this request, including request stitching. Zero retention mode may only be used by enterprise customers.
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub optimize_streaming_latency: Option<i64>,
    /// Output format of the generated audio. Formatted as codec_sample_rate_bitrate. So an mp3 with 22.05kHz sample rate at 32kbs is represented as mp3_22050_32. MP3 with 192kbps bitrate requires you to be subscribed to Creator tier or above. PCM with 44.1kHz sample rate requires you to be subscribed to Pro tier or above. Note that the μ-law format (sometimes written mu-law, often approximated as u-law) is commonly used for Twilio audio inputs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_format: Option<StreamOutputFormatQueryParameter1>,
}

impl StreamQueryRequest {
    pub fn builder() -> StreamQueryRequestBuilder {
        <StreamQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct StreamQueryRequestBuilder {
    enable_logging: Option<bool>,
    optimize_streaming_latency: Option<i64>,
    output_format: Option<StreamOutputFormatQueryParameter1>,
}

impl StreamQueryRequestBuilder {
    pub fn enable_logging(mut self, value: bool) -> Self {
        self.enable_logging = Some(value);
        self
    }

    pub fn optimize_streaming_latency(mut self, value: i64) -> Self {
        self.optimize_streaming_latency = Some(value);
        self
    }

    pub fn output_format(mut self, value: StreamOutputFormatQueryParameter1) -> Self {
        self.output_format = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`StreamQueryRequest`].
    pub fn build(self) -> Result<StreamQueryRequest, BuildError> {
        Ok(StreamQueryRequest {
            enable_logging: self.enable_logging,
            optimize_streaming_latency: self.optimize_streaming_latency,
            output_format: self.output_format,
        })
    }
}
