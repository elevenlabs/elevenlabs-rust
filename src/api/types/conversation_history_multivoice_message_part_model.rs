pub use crate::prelude::*;

/// Represents a single voice part of a multi-voice message.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ConversationHistoryMultivoiceMessagePartModel {
    #[serde(default)]
    pub text: String,
    #[serde(default)]
    pub voice_label: Option<String>,
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::int::option")]
    pub time_in_call_secs: Option<i64>,
}

impl ConversationHistoryMultivoiceMessagePartModel {
    pub fn builder() -> ConversationHistoryMultivoiceMessagePartModelBuilder {
        <ConversationHistoryMultivoiceMessagePartModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConversationHistoryMultivoiceMessagePartModelBuilder {
    text: Option<String>,
    voice_label: Option<String>,
    time_in_call_secs: Option<i64>,
}

impl ConversationHistoryMultivoiceMessagePartModelBuilder {
    pub fn text(mut self, value: impl Into<String>) -> Self {
        self.text = Some(value.into());
        self
    }

    pub fn voice_label(mut self, value: impl Into<String>) -> Self {
        self.voice_label = Some(value.into());
        self
    }

    pub fn time_in_call_secs(mut self, value: i64) -> Self {
        self.time_in_call_secs = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ConversationHistoryMultivoiceMessagePartModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`text`](ConversationHistoryMultivoiceMessagePartModelBuilder::text)
    pub fn build(self) -> Result<ConversationHistoryMultivoiceMessagePartModel, BuildError> {
        Ok(ConversationHistoryMultivoiceMessagePartModel {
            text: self.text.ok_or_else(|| BuildError::missing_field("text"))?,
            voice_label: self.voice_label,
            time_in_call_secs: self.time_in_call_secs,
        })
    }
}
