pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateSoundEffectRequest {
    /// The text that will get converted into a sound effect.
    #[serde(default)]
    pub text: String,
    /// Whether to create a sound effect that loops smoothly.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#loop: Option<bool>,
    /// The duration of the sound which will be generated in seconds. Must be at least 0.5 and at most 30. If set to None we will guess the optimal duration using the prompt. Defaults to None.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::float::option")]
    pub duration_seconds: Option<f64>,
    /// A higher prompt influence makes your generation follow the prompt more closely while also making generations less variable. Must be a value between 0 and 1. Defaults to 0.3.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::float::option")]
    pub prompt_influence: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_id: Option<SfxModelId>,
}

impl CreateSoundEffectRequest {
    pub fn builder() -> CreateSoundEffectRequestBuilder {
        <CreateSoundEffectRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateSoundEffectRequestBuilder {
    text: Option<String>,
    r#loop: Option<bool>,
    duration_seconds: Option<f64>,
    prompt_influence: Option<f64>,
    model_id: Option<SfxModelId>,
}

impl CreateSoundEffectRequestBuilder {
    pub fn text(mut self, value: impl Into<String>) -> Self {
        self.text = Some(value.into());
        self
    }

    pub fn r#loop(mut self, value: bool) -> Self {
        self.r#loop = Some(value);
        self
    }

    pub fn duration_seconds(mut self, value: f64) -> Self {
        self.duration_seconds = Some(value);
        self
    }

    pub fn prompt_influence(mut self, value: f64) -> Self {
        self.prompt_influence = Some(value);
        self
    }

    pub fn model_id(mut self, value: SfxModelId) -> Self {
        self.model_id = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateSoundEffectRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`text`](CreateSoundEffectRequestBuilder::text)
    pub fn build(self) -> Result<CreateSoundEffectRequest, BuildError> {
        Ok(CreateSoundEffectRequest {
            text: self.text.ok_or_else(|| BuildError::missing_field("text"))?,
            r#loop: self.r#loop,
            duration_seconds: self.duration_seconds,
            prompt_influence: self.prompt_influence,
            model_id: self.model_id,
        })
    }
}
