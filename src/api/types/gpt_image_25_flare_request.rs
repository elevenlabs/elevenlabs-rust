pub use crate::prelude::*;

/// Request body for the OpenAI GPT Image 2.5 Flare model.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GptImage25FlareRequest {
    /// Include to send the generation's result to the workspace's configured flows webhooks once it completes or fails. The webhook payload matches the terminal response of the corresponding GET endpoint.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub webhook: Option<WebhookTarget>,
    /// A text description of the image to generate.
    #[serde(default)]
    pub prompt: String,
    /// Up to 10 reference images to edit or draw from.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub images: Option<Vec<ImageReference>>,
    /// An image whose fully transparent areas mark where the first reference image may be edited; requires `images`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mask: Option<ImageReference>,
    /// The quality of the output image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quality: Option<GptImage25FlareRequestQuality>,
    /// The aspect ratio of the output image. With `auto`, the model picks an aspect ratio based on the inputs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aspect_ratio: Option<GptImage25FlareRequestAspectRatio>,
    /// The resolution of the output image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolution: Option<Gemini3ProImageRequestResolution>,
}

impl GptImage25FlareRequest {
    pub fn builder() -> GptImage25FlareRequestBuilder {
        <GptImage25FlareRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GptImage25FlareRequestBuilder {
    webhook: Option<WebhookTarget>,
    prompt: Option<String>,
    images: Option<Vec<ImageReference>>,
    mask: Option<ImageReference>,
    quality: Option<GptImage25FlareRequestQuality>,
    aspect_ratio: Option<GptImage25FlareRequestAspectRatio>,
    resolution: Option<Gemini3ProImageRequestResolution>,
}

impl GptImage25FlareRequestBuilder {
    pub fn webhook(mut self, value: WebhookTarget) -> Self {
        self.webhook = Some(value);
        self
    }

    pub fn prompt(mut self, value: impl Into<String>) -> Self {
        self.prompt = Some(value.into());
        self
    }

    pub fn images(mut self, value: Vec<ImageReference>) -> Self {
        self.images = Some(value);
        self
    }

    pub fn mask(mut self, value: ImageReference) -> Self {
        self.mask = Some(value);
        self
    }

    pub fn quality(mut self, value: GptImage25FlareRequestQuality) -> Self {
        self.quality = Some(value);
        self
    }

    pub fn aspect_ratio(mut self, value: GptImage25FlareRequestAspectRatio) -> Self {
        self.aspect_ratio = Some(value);
        self
    }

    pub fn resolution(mut self, value: Gemini3ProImageRequestResolution) -> Self {
        self.resolution = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GptImage25FlareRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`prompt`](GptImage25FlareRequestBuilder::prompt)
    pub fn build(self) -> Result<GptImage25FlareRequest, BuildError> {
        Ok(GptImage25FlareRequest {
            webhook: self.webhook,
            prompt: self
                .prompt
                .ok_or_else(|| BuildError::missing_field("prompt"))?,
            images: self.images,
            mask: self.mask,
            quality: self.quality,
            aspect_ratio: self.aspect_ratio,
            resolution: self.resolution,
        })
    }
}
