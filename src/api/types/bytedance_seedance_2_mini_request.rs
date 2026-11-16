pub use crate::prelude::*;

/// Request body for the ByteDance Seedance 2.0 Mini video model.
///
/// ByteDance models are disabled by default and require explicit approval
/// before use. Contact support to request access.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BytedanceSeedance2MiniRequest {
    /// Include to send the generation's result to the workspace's configured flows webhooks once it completes or fails. The webhook payload matches the terminal response of the corresponding GET endpoint.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub webhook: Option<WebhookTarget>,
    /// A text description of the video to generate.
    #[serde(default)]
    pub prompt: String,
    /// The aspect ratio of the output video. With `auto`, the model picks an aspect ratio based on the inputs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aspect_ratio: Option<BytedanceSeedance2FastRequestAspectRatio>,
    /// The duration of the output video in seconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::int::option")]
    pub duration_secs: Option<i64>,
    /// A seed for reproducible generation: the same seed and inputs give similar output across generations. Omit for random.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::int::option")]
    pub seed: Option<i64>,
    /// Whether to generate audio with the video.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generate_audio: Option<bool>,
    /// The image to use as the video's first frame.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_frame: Option<ImageReference>,
    /// The image to use as the video's last frame.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_frame: Option<ImageReference>,
    /// Up to 9 reference images to draw subjects from. Cannot be combined with `start_frame`/`end_frame`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub images: Option<Vec<ImageReference>>,
    /// Up to 3 reference videos to draw subjects or motion from. Cannot be combined with `start_frame`/`end_frame`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub videos: Option<Vec<VideoReference>>,
    /// Up to 3 reference audios, e.g. for lipsync. Requires at least one of `images` or `videos`, and cannot be combined with `start_frame`/`end_frame`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audios: Option<Vec<AudioReference>>,
    /// The resolution of the output video.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolution: Option<BytedanceSeedance2FastRequestResolution>,
}

impl BytedanceSeedance2MiniRequest {
    pub fn builder() -> BytedanceSeedance2MiniRequestBuilder {
        <BytedanceSeedance2MiniRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BytedanceSeedance2MiniRequestBuilder {
    webhook: Option<WebhookTarget>,
    prompt: Option<String>,
    aspect_ratio: Option<BytedanceSeedance2FastRequestAspectRatio>,
    duration_secs: Option<i64>,
    seed: Option<i64>,
    generate_audio: Option<bool>,
    start_frame: Option<ImageReference>,
    end_frame: Option<ImageReference>,
    images: Option<Vec<ImageReference>>,
    videos: Option<Vec<VideoReference>>,
    audios: Option<Vec<AudioReference>>,
    resolution: Option<BytedanceSeedance2FastRequestResolution>,
}

impl BytedanceSeedance2MiniRequestBuilder {
    pub fn webhook(mut self, value: WebhookTarget) -> Self {
        self.webhook = Some(value);
        self
    }

    pub fn prompt(mut self, value: impl Into<String>) -> Self {
        self.prompt = Some(value.into());
        self
    }

    pub fn aspect_ratio(mut self, value: BytedanceSeedance2FastRequestAspectRatio) -> Self {
        self.aspect_ratio = Some(value);
        self
    }

    pub fn duration_secs(mut self, value: i64) -> Self {
        self.duration_secs = Some(value);
        self
    }

    pub fn seed(mut self, value: i64) -> Self {
        self.seed = Some(value);
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

    pub fn images(mut self, value: Vec<ImageReference>) -> Self {
        self.images = Some(value);
        self
    }

    pub fn videos(mut self, value: Vec<VideoReference>) -> Self {
        self.videos = Some(value);
        self
    }

    pub fn audios(mut self, value: Vec<AudioReference>) -> Self {
        self.audios = Some(value);
        self
    }

    pub fn resolution(mut self, value: BytedanceSeedance2FastRequestResolution) -> Self {
        self.resolution = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BytedanceSeedance2MiniRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`prompt`](BytedanceSeedance2MiniRequestBuilder::prompt)
    pub fn build(self) -> Result<BytedanceSeedance2MiniRequest, BuildError> {
        Ok(BytedanceSeedance2MiniRequest {
            webhook: self.webhook,
            prompt: self
                .prompt
                .ok_or_else(|| BuildError::missing_field("prompt"))?,
            aspect_ratio: self.aspect_ratio,
            duration_secs: self.duration_secs,
            seed: self.seed,
            generate_audio: self.generate_audio,
            start_frame: self.start_frame,
            end_frame: self.end_frame,
            images: self.images,
            videos: self.videos,
            audios: self.audios,
            resolution: self.resolution,
        })
    }
}
