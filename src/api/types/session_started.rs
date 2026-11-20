pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SessionStarted {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_type: Option<DubbingSessionStartedMessageType>,
    pub session_id: String,
    pub config: SessionStartedConfig,
}

impl SessionStarted {
    pub fn builder() -> SessionStartedBuilder {
        <SessionStartedBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SessionStartedBuilder {
    message_type: Option<DubbingSessionStartedMessageType>,
    session_id: Option<String>,
    config: Option<SessionStartedConfig>,
}

impl SessionStartedBuilder {
    pub fn message_type(mut self, value: DubbingSessionStartedMessageType) -> Self {
        self.message_type = Some(value);
        self
    }

    pub fn session_id(mut self, value: impl Into<String>) -> Self {
        self.session_id = Some(value.into());
        self
    }

    pub fn config(mut self, value: SessionStartedConfig) -> Self {
        self.config = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SessionStarted`].
    /// This method will fail if any of the following fields are not set:
    /// - [`session_id`](SessionStartedBuilder::session_id)
    /// - [`config`](SessionStartedBuilder::config)
    pub fn build(self) -> Result<SessionStarted, BuildError> {
        Ok(SessionStarted {
            message_type: self.message_type,
            session_id: self
                .session_id
                .ok_or_else(|| BuildError::missing_field("session_id"))?,
            config: self
                .config
                .ok_or_else(|| BuildError::missing_field("config"))?,
        })
    }
}
