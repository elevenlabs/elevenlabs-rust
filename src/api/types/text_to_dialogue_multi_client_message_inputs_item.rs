pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TextToDialogueMultiClientMessageInputsItem {
    /// Text to convert to speech.
    #[serde(default)]
    pub text: String,
    /// Voice ID for this segment.
    #[serde(default)]
    pub voice_id: String,
    /// Signal that this input starts a new speaker turn for prosody continuity.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_turn: Option<bool>,
}

impl TextToDialogueMultiClientMessageInputsItem {
    pub fn builder() -> TextToDialogueMultiClientMessageInputsItemBuilder {
        <TextToDialogueMultiClientMessageInputsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TextToDialogueMultiClientMessageInputsItemBuilder {
    text: Option<String>,
    voice_id: Option<String>,
    new_turn: Option<bool>,
}

impl TextToDialogueMultiClientMessageInputsItemBuilder {
    pub fn text(mut self, value: impl Into<String>) -> Self {
        self.text = Some(value.into());
        self
    }

    pub fn voice_id(mut self, value: impl Into<String>) -> Self {
        self.voice_id = Some(value.into());
        self
    }

    pub fn new_turn(mut self, value: bool) -> Self {
        self.new_turn = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TextToDialogueMultiClientMessageInputsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`text`](TextToDialogueMultiClientMessageInputsItemBuilder::text)
    /// - [`voice_id`](TextToDialogueMultiClientMessageInputsItemBuilder::voice_id)
    pub fn build(self) -> Result<TextToDialogueMultiClientMessageInputsItem, BuildError> {
        Ok(TextToDialogueMultiClientMessageInputsItem {
            text: self.text.ok_or_else(|| BuildError::missing_field("text"))?,
            voice_id: self
                .voice_id
                .ok_or_else(|| BuildError::missing_field("voice_id"))?,
            new_turn: self.new_turn,
        })
    }
}
