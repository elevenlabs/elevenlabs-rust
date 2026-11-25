pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct SubtitlingSessionStarted {
    #[serde(default)]
    pub session_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_session_id: Option<String>,
}

impl SubtitlingSessionStarted {
    pub fn builder() -> SubtitlingSessionStartedBuilder {
        <SubtitlingSessionStartedBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SubtitlingSessionStartedBuilder {
    session_id: Option<String>,
    client_session_id: Option<String>,
}

impl SubtitlingSessionStartedBuilder {
    pub fn session_id(mut self, value: impl Into<String>) -> Self {
        self.session_id = Some(value.into());
        self
    }

    pub fn client_session_id(mut self, value: impl Into<String>) -> Self {
        self.client_session_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SubtitlingSessionStarted`].
    /// This method will fail if any of the following fields are not set:
    /// - [`session_id`](SubtitlingSessionStartedBuilder::session_id)
    pub fn build(self) -> Result<SubtitlingSessionStarted, BuildError> {
        Ok(SubtitlingSessionStarted {
            session_id: self
                .session_id
                .ok_or_else(|| BuildError::missing_field("session_id"))?,
            client_session_id: self.client_session_id,
        })
    }
}
