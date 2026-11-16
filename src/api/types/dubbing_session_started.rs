pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct DubbingSessionStarted {
    #[serde(default)]
    pub session_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_session_id: Option<String>,
}

impl DubbingSessionStarted {
    pub fn builder() -> DubbingSessionStartedBuilder {
        <DubbingSessionStartedBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DubbingSessionStartedBuilder {
    session_id: Option<String>,
    client_session_id: Option<String>,
}

impl DubbingSessionStartedBuilder {
    pub fn session_id(mut self, value: impl Into<String>) -> Self {
        self.session_id = Some(value.into());
        self
    }

    pub fn client_session_id(mut self, value: impl Into<String>) -> Self {
        self.client_session_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DubbingSessionStarted`].
    /// This method will fail if any of the following fields are not set:
    /// - [`session_id`](DubbingSessionStartedBuilder::session_id)
    pub fn build(self) -> Result<DubbingSessionStarted, BuildError> {
        Ok(DubbingSessionStarted {
            session_id: self
                .session_id
                .ok_or_else(|| BuildError::missing_field("session_id"))?,
            client_session_id: self.client_session_id,
        })
    }
}
