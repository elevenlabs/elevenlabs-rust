pub use crate::prelude::*;

/// Request body for the ByteDance Seedance 2.5 video model.
///
/// Diverges from the Seedance 2.0 public shape: durations up to 30s, larger
/// reference caps, audio-only input allowed, and no ``seed`` (Ark tolerates it
/// but does not honour it).
///
/// ByteDance models are disabled by default and require explicit approval
/// before use. Contact support to request access.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BytedanceSeedance25Request {
    /// Include to send the generation's result to the workspace's configured flows webhooks once it completes or fails. The webhook payload matches the terminal response of the corresponding GET endpoint.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub webhook: Option<WebhookTarget>,
    /// A text description of the video to generate.
    #[serde(default)]
    pub prompt: String,
    /// The aspect ratio of the output video. With `auto`, the model picks an aspect ratio based on the inputs. First-frame / first-and-last-frame tasks always use `auto`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aspect_ratio: Option<BytedanceSeedance25RequestAspectRatio>,
    /// The resolution of the output video.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolution: Option<BytedanceSeedance25RequestResolution>,
    /// The duration of the output video in seconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::int::option")]
    pub duration_secs: Option<i64>,
    /// Whether to generate audio with the video.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generate_audio: Option<bool>,
    /// The image to use as the video's first frame.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_frame: Option<ImageReference>,
    /// The image to use as the video's last frame.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_frame: Option<ImageReference>,
    /// Up to 30 reference images to draw subjects from. Cannot be combined with `start_frame`/`end_frame`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub images: Option<Vec<ImageReference>>,
    /// Up to 10 reference videos to draw subjects or motion from. Cannot be combined with `start_frame`/`end_frame`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub videos: Option<Vec<VideoReference>>,
    /// Up to 10 reference audios, e.g. for lipsync. Cannot be combined with `start_frame`/`end_frame`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audios: Option<Vec<AudioReference>>,
}

impl BytedanceSeedance25Request {
    pub fn builder() -> BytedanceSeedance25RequestBuilder {
        <BytedanceSeedance25RequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BytedanceSeedance25RequestBuilder {
    webhook: Option<WebhookTarget>,
    prompt: Option<String>,
    aspect_ratio: Option<BytedanceSeedance25RequestAspectRatio>,
    resolution: Option<BytedanceSeedance25RequestResolution>,
    duration_secs: Option<i64>,
    generate_audio: Option<bool>,
    start_frame: Option<ImageReference>,
    end_frame: Option<ImageReference>,
    images: Option<Vec<ImageReference>>,
    videos: Option<Vec<VideoReference>>,
    audios: Option<Vec<AudioReference>>,
}

impl BytedanceSeedance25RequestBuilder {
    pub fn webhook(mut self, value: WebhookTarget) -> Self {
        self.webhook = Some(value);
        self
    }

    pub fn prompt(mut self, value: impl Into<String>) -> Self {
        self.prompt = Some(value.into());
        self
    }

    pub fn aspect_ratio(mut self, value: BytedanceSeedance25RequestAspectRatio) -> Self {
        self.aspect_ratio = Some(value);
        self
    }

    pub fn resolution(mut self, value: BytedanceSeedance25RequestResolution) -> Self {
        self.resolution = Some(value);
        self
    }

    pub fn duration_secs(mut self, value: i64) -> Self {
        self.duration_secs = Some(value);
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

    /// Consumes the builder and constructs a [`BytedanceSeedance25Request`].
    /// This method will fail if any of the following fields are not set:
    /// - [`prompt`](BytedanceSeedance25RequestBuilder::prompt)
    pub fn build(self) -> Result<BytedanceSeedance25Request, BuildError> {
        Ok(BytedanceSeedance25Request {
            webhook: self.webhook,
            prompt: self
                .prompt
                .ok_or_else(|| BuildError::missing_field("prompt"))?,
            aspect_ratio: self.aspect_ratio,
            resolution: self.resolution,
            duration_secs: self.duration_secs,
            generate_audio: self.generate_audio,
            start_frame: self.start_frame,
            end_frame: self.end_frame,
            images: self.images,
            videos: self.videos,
            audios: self.audios,
        })
    }
}
