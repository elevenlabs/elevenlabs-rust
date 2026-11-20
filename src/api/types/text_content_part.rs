pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TextContentPart {
    #[serde(default)]
    pub text: String,
}

impl TextContentPart {
    pub fn builder() -> TextContentPartBuilder {
        <TextContentPartBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TextContentPartBuilder {
    text: Option<String>,
}

impl TextContentPartBuilder {
    pub fn text(mut self, value: impl Into<String>) -> Self {
        self.text = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TextContentPart`].
    /// This method will fail if any of the following fields are not set:
    /// - [`text`](TextContentPartBuilder::text)
    pub fn build(self) -> Result<TextContentPart, BuildError> {
        Ok(TextContentPart {
            text: self.text.ok_or_else(|| BuildError::missing_field("text"))?,
        })
    }
}
