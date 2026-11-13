pub use crate::prelude::*;

/// Request body for the ByteDance Seedream 5.0 Pro image model.
///
/// ByteDance models are disabled by default and require explicit approval
/// before use. Contact support to request access.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BytedanceSeedream5ProRequest {
    /// Include to send the generation's result to the workspace's configured flows webhooks once it completes or fails. The webhook payload matches the terminal response of the corresponding GET endpoint.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub webhook: Option<WebhookTarget>,
    /// A text description of the image to generate.
    #[serde(default)]
    pub prompt: String,
    /// Up to 10 reference images to edit or draw from.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub images: Option<Vec<ImageReference>>,
    /// The aspect ratio of the output image. With `auto`, the model picks an aspect ratio based on the inputs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aspect_ratio: Option<BytedanceSeedream5LiteRequestAspectRatio>,
    /// A seed for reproducible generation: the same seed and inputs give similar output across generations. Omit for random.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::int::option")]
    pub seed: Option<i64>,
    /// The resolution of the output image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolution: Option<BytedanceSeedream5ProRequestResolution>,
}

impl BytedanceSeedream5ProRequest {
    pub fn builder() -> BytedanceSeedream5ProRequestBuilder {
        <BytedanceSeedream5ProRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BytedanceSeedream5ProRequestBuilder {
    webhook: Option<WebhookTarget>,
    prompt: Option<String>,
    images: Option<Vec<ImageReference>>,
    aspect_ratio: Option<BytedanceSeedream5LiteRequestAspectRatio>,
    seed: Option<i64>,
    resolution: Option<BytedanceSeedream5ProRequestResolution>,
}

impl BytedanceSeedream5ProRequestBuilder {
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

    pub fn aspect_ratio(mut self, value: BytedanceSeedream5LiteRequestAspectRatio) -> Self {
        self.aspect_ratio = Some(value);
        self
    }

    pub fn seed(mut self, value: i64) -> Self {
        self.seed = Some(value);
        self
    }

    pub fn resolution(mut self, value: BytedanceSeedream5ProRequestResolution) -> Self {
        self.resolution = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BytedanceSeedream5ProRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`prompt`](BytedanceSeedream5ProRequestBuilder::prompt)
    pub fn build(self) -> Result<BytedanceSeedream5ProRequest, BuildError> {
        Ok(BytedanceSeedream5ProRequest {
            webhook: self.webhook,
            prompt: self
                .prompt
                .ok_or_else(|| BuildError::missing_field("prompt"))?,
            images: self.images,
            aspect_ratio: self.aspect_ratio,
            seed: self.seed,
            resolution: self.resolution,
        })
    }
}
