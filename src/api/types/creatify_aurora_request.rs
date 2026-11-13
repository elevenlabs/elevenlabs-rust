pub use crate::prelude::*;

/// Request body for the Creatify Aurora lipsync video model.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreatifyAuroraRequest {
    /// Include to send the generation's result to the workspace's configured flows webhooks once it completes or fails. The webhook payload matches the terminal response of the corresponding GET endpoint.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub webhook: Option<WebhookTarget>,
    pub image: ImageReference,
    pub audio: AudioReference,
    /// The resolution of the output video.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolution: Option<BytedanceSeedance2FastRequestResolution>,
    /// How strongly the generation adheres to the input image. Omit to use the model's default.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::float::option")]
    pub guidance_scale: Option<f64>,
    /// How strongly the lip movements adhere to the audio. Omit to use the model's default.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::float::option")]
    pub audio_guidance_scale: Option<f64>,
}

impl CreatifyAuroraRequest {
    pub fn builder() -> CreatifyAuroraRequestBuilder {
        <CreatifyAuroraRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreatifyAuroraRequestBuilder {
    webhook: Option<WebhookTarget>,
    image: Option<ImageReference>,
    audio: Option<AudioReference>,
    resolution: Option<BytedanceSeedance2FastRequestResolution>,
    guidance_scale: Option<f64>,
    audio_guidance_scale: Option<f64>,
}

impl CreatifyAuroraRequestBuilder {
    pub fn webhook(mut self, value: WebhookTarget) -> Self {
        self.webhook = Some(value);
        self
    }

    pub fn image(mut self, value: ImageReference) -> Self {
        self.image = Some(value);
        self
    }

    pub fn audio(mut self, value: AudioReference) -> Self {
        self.audio = Some(value);
        self
    }

    pub fn resolution(mut self, value: BytedanceSeedance2FastRequestResolution) -> Self {
        self.resolution = Some(value);
        self
    }

    pub fn guidance_scale(mut self, value: f64) -> Self {
        self.guidance_scale = Some(value);
        self
    }

    pub fn audio_guidance_scale(mut self, value: f64) -> Self {
        self.audio_guidance_scale = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreatifyAuroraRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`image`](CreatifyAuroraRequestBuilder::image)
    /// - [`audio`](CreatifyAuroraRequestBuilder::audio)
    pub fn build(self) -> Result<CreatifyAuroraRequest, BuildError> {
        Ok(CreatifyAuroraRequest {
            webhook: self.webhook,
            image: self
                .image
                .ok_or_else(|| BuildError::missing_field("image"))?,
            audio: self
                .audio
                .ok_or_else(|| BuildError::missing_field("audio"))?,
            resolution: self.resolution,
            guidance_scale: self.guidance_scale,
            audio_guidance_scale: self.audio_guidance_scale,
        })
    }
}
