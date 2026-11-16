pub use crate::prelude::*;

/// Request body for the Google Gemini 3.1 Flash image model.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Gemini31FlashImageRequest {
    /// Include to send the generation's result to the workspace's configured flows webhooks once it completes or fails. The webhook payload matches the terminal response of the corresponding GET endpoint.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub webhook: Option<WebhookTarget>,
    /// A text description of the image to generate.
    #[serde(default)]
    pub prompt: String,
    /// Up to 14 reference images to edit or draw from.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub images: Option<Vec<ImageReference>>,
    /// The aspect ratio of the output image. With `auto`, the model picks an aspect ratio based on the inputs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aspect_ratio: Option<Gemini31FlashImageRequestAspectRatio>,
    /// The resolution of the output image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolution: Option<Gemini31FlashImageRequestResolution>,
}

impl Gemini31FlashImageRequest {
    pub fn builder() -> Gemini31FlashImageRequestBuilder {
        <Gemini31FlashImageRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct Gemini31FlashImageRequestBuilder {
    webhook: Option<WebhookTarget>,
    prompt: Option<String>,
    images: Option<Vec<ImageReference>>,
    aspect_ratio: Option<Gemini31FlashImageRequestAspectRatio>,
    resolution: Option<Gemini31FlashImageRequestResolution>,
}

impl Gemini31FlashImageRequestBuilder {
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

    pub fn aspect_ratio(mut self, value: Gemini31FlashImageRequestAspectRatio) -> Self {
        self.aspect_ratio = Some(value);
        self
    }

    pub fn resolution(mut self, value: Gemini31FlashImageRequestResolution) -> Self {
        self.resolution = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`Gemini31FlashImageRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`prompt`](Gemini31FlashImageRequestBuilder::prompt)
    pub fn build(self) -> Result<Gemini31FlashImageRequest, BuildError> {
        Ok(Gemini31FlashImageRequest {
            webhook: self.webhook,
            prompt: self
                .prompt
                .ok_or_else(|| BuildError::missing_field("prompt"))?,
            images: self.images,
            aspect_ratio: self.aspect_ratio,
            resolution: self.resolution,
        })
    }
}
