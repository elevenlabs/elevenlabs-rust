pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct EntitiesItem {
    /// The text that was identified as an entity.
    #[serde(default)]
    pub text: String,
    /// The type of entity detected (e.g., 'credit_card', 'email_address', 'person_name').
    #[serde(default)]
    pub entity_type: String,
    /// Start character position in the transcript text.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::int")]
    pub start_char: i64,
    /// End character position in the transcript text.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::int")]
    pub end_char: i64,
}

impl EntitiesItem {
    pub fn builder() -> EntitiesItemBuilder {
        <EntitiesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EntitiesItemBuilder {
    text: Option<String>,
    entity_type: Option<String>,
    start_char: Option<i64>,
    end_char: Option<i64>,
}

impl EntitiesItemBuilder {
    pub fn text(mut self, value: impl Into<String>) -> Self {
        self.text = Some(value.into());
        self
    }

    pub fn entity_type(mut self, value: impl Into<String>) -> Self {
        self.entity_type = Some(value.into());
        self
    }

    pub fn start_char(mut self, value: i64) -> Self {
        self.start_char = Some(value);
        self
    }

    pub fn end_char(mut self, value: i64) -> Self {
        self.end_char = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`EntitiesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`text`](EntitiesItemBuilder::text)
    /// - [`entity_type`](EntitiesItemBuilder::entity_type)
    /// - [`start_char`](EntitiesItemBuilder::start_char)
    /// - [`end_char`](EntitiesItemBuilder::end_char)
    pub fn build(self) -> Result<EntitiesItem, BuildError> {
        Ok(EntitiesItem {
            text: self.text.ok_or_else(|| BuildError::missing_field("text"))?,
            entity_type: self
                .entity_type
                .ok_or_else(|| BuildError::missing_field("entity_type"))?,
            start_char: self
                .start_char
                .ok_or_else(|| BuildError::missing_field("start_char"))?,
            end_char: self
                .end_char
                .ok_or_else(|| BuildError::missing_field("end_char"))?,
        })
    }
}
