pub use crate::prelude::*;

/// Request body for the OpenAI GPT Image 1 model.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GptImage1Request {
    /// Include to send the generation's result to the workspace's configured flows webhooks once it completes or fails. The webhook payload matches the terminal response of the corresponding GET endpoint.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub webhook: Option<WebhookTarget>,
    /// A text description of the image to generate.
    #[serde(default)]
    pub prompt: String,
    /// Up to 5 reference images to edit or draw from.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub images: Option<Vec<ImageReference>>,
    /// An image whose fully transparent areas mark where the first reference image may be edited; requires `images`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mask: Option<ImageReference>,
    /// The quality of the output image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quality: Option<GptImage1RequestQuality>,
    /// The background of the output image. With `auto`, the model picks the background that suits the image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background: Option<GptImage1RequestBackground>,
    /// The aspect ratio of the output image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aspect_ratio: Option<GptImage1RequestAspectRatio>,
}

impl GptImage1Request {
    pub fn builder() -> GptImage1RequestBuilder {
        <GptImage1RequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GptImage1RequestBuilder {
    webhook: Option<WebhookTarget>,
    prompt: Option<String>,
    images: Option<Vec<ImageReference>>,
    mask: Option<ImageReference>,
    quality: Option<GptImage1RequestQuality>,
    background: Option<GptImage1RequestBackground>,
    aspect_ratio: Option<GptImage1RequestAspectRatio>,
}

impl GptImage1RequestBuilder {
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

    pub fn quality(mut self, value: GptImage1RequestQuality) -> Self {
        self.quality = Some(value);
        self
    }

    pub fn background(mut self, value: GptImage1RequestBackground) -> Self {
        self.background = Some(value);
        self
    }

    pub fn aspect_ratio(mut self, value: GptImage1RequestAspectRatio) -> Self {
        self.aspect_ratio = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GptImage1Request`].
    /// This method will fail if any of the following fields are not set:
    /// - [`prompt`](GptImage1RequestBuilder::prompt)
    pub fn build(self) -> Result<GptImage1Request, BuildError> {
        Ok(GptImage1Request {
            webhook: self.webhook,
            prompt: self
                .prompt
                .ok_or_else(|| BuildError::missing_field("prompt"))?,
            images: self.images,
            mask: self.mask,
            quality: self.quality,
            background: self.background,
            aspect_ratio: self.aspect_ratio,
        })
    }
}
