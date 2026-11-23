pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ErrorEvent {
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::int")]
    pub code: i64,
    #[serde(default)]
    pub error_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

impl ErrorEvent {
    pub fn builder() -> ErrorEventBuilder {
        <ErrorEventBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ErrorEventBuilder {
    code: Option<i64>,
    error_name: Option<String>,
    message: Option<String>,
}

impl ErrorEventBuilder {
    pub fn code(mut self, value: i64) -> Self {
        self.code = Some(value);
        self
    }

    pub fn error_name(mut self, value: impl Into<String>) -> Self {
        self.error_name = Some(value.into());
        self
    }

    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ErrorEvent`].
    /// This method will fail if any of the following fields are not set:
    /// - [`code`](ErrorEventBuilder::code)
    /// - [`error_name`](ErrorEventBuilder::error_name)
    pub fn build(self) -> Result<ErrorEvent, BuildError> {
        Ok(ErrorEvent {
            code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
            error_name: self
                .error_name
                .ok_or_else(|| BuildError::missing_field("error_name"))?,
            message: self.message,
        })
    }
}
