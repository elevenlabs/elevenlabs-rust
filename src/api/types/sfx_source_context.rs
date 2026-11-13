pub use crate::prelude::*;

/// Context for sound effect clips.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct SfxSourceContext {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sound_generation_history_item_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generation_config: Option<serde_json::Value>,
}

impl SfxSourceContext {
    pub fn builder() -> SfxSourceContextBuilder {
        <SfxSourceContextBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SfxSourceContextBuilder {
    sound_generation_history_item_id: Option<String>,
    text: Option<String>,
    generation_config: Option<serde_json::Value>,
}

impl SfxSourceContextBuilder {
    pub fn sound_generation_history_item_id(mut self, value: impl Into<String>) -> Self {
        self.sound_generation_history_item_id = Some(value.into());
        self
    }

    pub fn text(mut self, value: impl Into<String>) -> Self {
        self.text = Some(value.into());
        self
    }

    pub fn generation_config(mut self, value: serde_json::Value) -> Self {
        self.generation_config = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SfxSourceContext`].
    pub fn build(self) -> Result<SfxSourceContext, BuildError> {
        Ok(SfxSourceContext {
            sound_generation_history_item_id: self.sound_generation_history_item_id,
            text: self.text,
            generation_config: self.generation_config,
        })
    }
}
