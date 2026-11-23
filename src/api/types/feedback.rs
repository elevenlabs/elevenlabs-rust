pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Feedback {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<FeedbackType>,
    /// ID of the agent response event being rated.
    #[serde(with = "crate::core::number_serializers::int")]
    pub event_id: i64,
    /// Feedback score for the referenced agent response. Null clears any previously submitted feedback for the event.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score: Option<Score>,
}

impl Feedback {
    pub fn builder() -> FeedbackBuilder {
        <FeedbackBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct FeedbackBuilder {
    r#type: Option<FeedbackType>,
    event_id: Option<i64>,
    score: Option<Score>,
}

impl FeedbackBuilder {
    pub fn r#type(mut self, value: FeedbackType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn event_id(mut self, value: i64) -> Self {
        self.event_id = Some(value);
        self
    }

    pub fn score(mut self, value: Score) -> Self {
        self.score = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`Feedback`].
    /// This method will fail if any of the following fields are not set:
    /// - [`event_id`](FeedbackBuilder::event_id)
    pub fn build(self) -> Result<Feedback, BuildError> {
        Ok(Feedback {
            r#type: self.r#type,
            event_id: self
                .event_id
                .ok_or_else(|| BuildError::missing_field("event_id"))?,
            score: self.score,
        })
    }
}
