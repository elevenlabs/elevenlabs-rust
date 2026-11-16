pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Error {
    pub message: String,
    pub error: String,
    #[serde(with = "crate::core::number_serializers::int")]
    pub code: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub param: Option<String>,
}

impl Error {
    pub fn builder() -> ErrorBuilder {
        <ErrorBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ErrorBuilder {
    message: Option<String>,
    error: Option<String>,
    code: Option<i64>,
    param: Option<String>,
}

impl ErrorBuilder {
    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    pub fn error(mut self, value: impl Into<String>) -> Self {
        self.error = Some(value.into());
        self
    }

    pub fn code(mut self, value: i64) -> Self {
        self.code = Some(value);
        self
    }

    pub fn param(mut self, value: impl Into<String>) -> Self {
        self.param = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`Error`].
    /// This method will fail if any of the following fields are not set:
    /// - [`message`](ErrorBuilder::message)
    /// - [`error`](ErrorBuilder::error)
    /// - [`code`](ErrorBuilder::code)
    pub fn build(self) -> Result<Error, BuildError> {
        Ok(Error {
            message: self
                .message
                .ok_or_else(|| BuildError::missing_field("message"))?,
            error: self
                .error
                .ok_or_else(|| BuildError::missing_field("error"))?,
            code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
            param: self.param,
        })
    }
}
