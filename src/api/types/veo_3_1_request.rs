pub use crate::prelude::*;

/// Request body for the Google Veo 3.1 video model.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Veo3_1Request {
    /// Include to send the generation's result to the workspace's configured flows webhooks once it completes or fails. The webhook payload matches the terminal response of the corresponding GET endpoint.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub webhook: Option<WebhookTarget>,
    /// A text description of the video to generate.
    #[serde(default)]
    pub prompt: String,
    /// A text description of what the video should avoid.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub negative_prompt: Option<String>,
    /// A seed for reproducible generation: the same seed and inputs give similar output across generations. Omit for random.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::int::option")]
    pub seed: Option<i64>,
    /// Whether the model may rewrite the prompt to improve results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enhance_prompt: Option<bool>,
    /// The duration of the output video in seconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_secs: Option<Veo3_1FastRequestDurationSecs>,
    /// The aspect ratio of the output video.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aspect_ratio: Option<Veo3_1FastRequestAspectRatio>,
    /// The resolution of the output video.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolution: Option<Veo3_1FastRequestResolution>,
    /// Whether to generate audio with the video.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generate_audio: Option<bool>,
    /// The image to use as the video's first frame.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_frame: Option<ImageReference>,
    /// The image to use as the video's last frame.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_frame: Option<ImageReference>,
    /// Up to 3 reference images to draw subjects or style from. Cannot be combined with `start_frame`/`end_frame`, and requires the 8-second duration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub images: Option<Vec<VeoImageReference>>,
}

impl Veo3_1Request {
    pub fn builder() -> Veo3_1RequestBuilder {
        <Veo3_1RequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct Veo3_1RequestBuilder {
    webhook: Option<WebhookTarget>,
    prompt: Option<String>,
    negative_prompt: Option<String>,
    seed: Option<i64>,
    enhance_prompt: Option<bool>,
    duration_secs: Option<Veo3_1FastRequestDurationSecs>,
    aspect_ratio: Option<Veo3_1FastRequestAspectRatio>,
    resolution: Option<Veo3_1FastRequestResolution>,
    generate_audio: Option<bool>,
    start_frame: Option<ImageReference>,
    end_frame: Option<ImageReference>,
    images: Option<Vec<VeoImageReference>>,
}

impl Veo3_1RequestBuilder {
    pub fn webhook(mut self, value: WebhookTarget) -> Self {
        self.webhook = Some(value);
        self
    }

    pub fn prompt(mut self, value: impl Into<String>) -> Self {
        self.prompt = Some(value.into());
        self
    }

    pub fn negative_prompt(mut self, value: impl Into<String>) -> Self {
        self.negative_prompt = Some(value.into());
        self
    }

    pub fn seed(mut self, value: i64) -> Self {
        self.seed = Some(value);
        self
    }

    pub fn enhance_prompt(mut self, value: bool) -> Self {
        self.enhance_prompt = Some(value);
        self
    }

    pub fn duration_secs(mut self, value: Veo3_1FastRequestDurationSecs) -> Self {
        self.duration_secs = Some(value);
        self
    }

    pub fn aspect_ratio(mut self, value: Veo3_1FastRequestAspectRatio) -> Self {
        self.aspect_ratio = Some(value);
        self
    }

    pub fn resolution(mut self, value: Veo3_1FastRequestResolution) -> Self {
        self.resolution = Some(value);
        self
    }

    pub fn generate_audio(mut self, value: bool) -> Self {
        self.generate_audio = Some(value);
        self
    }

    pub fn start_frame(mut self, value: ImageReference) -> Self {
        self.start_frame = Some(value);
        self
    }

    pub fn end_frame(mut self, value: ImageReference) -> Self {
        self.end_frame = Some(value);
        self
    }

    pub fn images(mut self, value: Vec<VeoImageReference>) -> Self {
        self.images = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`Veo3_1Request`].
    /// This method will fail if any of the following fields are not set:
    /// - [`prompt`](Veo3_1RequestBuilder::prompt)
    pub fn build(self) -> Result<Veo3_1Request, BuildError> {
        Ok(Veo3_1Request {
            webhook: self.webhook,
            prompt: self
                .prompt
                .ok_or_else(|| BuildError::missing_field("prompt"))?,
            negative_prompt: self.negative_prompt,
            seed: self.seed,
            enhance_prompt: self.enhance_prompt,
            duration_secs: self.duration_secs,
            aspect_ratio: self.aspect_ratio,
            resolution: self.resolution,
            generate_audio: self.generate_audio,
            start_frame: self.start_frame,
            end_frame: self.end_frame,
            images: self.images,
        })
    }
}
