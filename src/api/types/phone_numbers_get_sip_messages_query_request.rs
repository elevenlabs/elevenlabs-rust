pub use crate::prelude::*;

/// Query parameters for Get Sip Messages For A Phone Number
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PhoneNumbersGetSipMessagesQueryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    /// Used for fetching next page. Cursor is returned in the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

impl PhoneNumbersGetSipMessagesQueryRequest {
    pub fn builder() -> PhoneNumbersGetSipMessagesQueryRequestBuilder {
        <PhoneNumbersGetSipMessagesQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PhoneNumbersGetSipMessagesQueryRequestBuilder {
    page_size: Option<i64>,
    cursor: Option<String>,
}

impl PhoneNumbersGetSipMessagesQueryRequestBuilder {
    pub fn page_size(mut self, value: i64) -> Self {
        self.page_size = Some(value);
        self
    }

    pub fn cursor(mut self, value: impl Into<String>) -> Self {
        self.cursor = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PhoneNumbersGetSipMessagesQueryRequest`].
    pub fn build(self) -> Result<PhoneNumbersGetSipMessagesQueryRequest, BuildError> {
        Ok(PhoneNumbersGetSipMessagesQueryRequest {
            page_size: self.page_size,
            cursor: self.cursor,
        })
    }
}
