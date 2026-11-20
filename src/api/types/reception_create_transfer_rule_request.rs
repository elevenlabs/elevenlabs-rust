pub use crate::prelude::*;

/// Request model for creating a new transfer rule.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ReceptionCreateTransferRuleRequest {
    #[serde(default)]
    pub label: StrippedStr,
    #[serde(default)]
    pub phone_number: PhoneNumber,
    #[serde(default)]
    pub condition: StrippedStr,
    /// DTMF digits automatically played after the transfer destination picks up (e.g. '1' to answer a 'press 1 to accept' screen, or 'www1234' to key an extension after a pause). Valid characters are 0-9, *, #, w (0.5s pause) and W (1s pause). Twilio transfers only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_dial_digits: Option<String>,
}

impl ReceptionCreateTransferRuleRequest {
    pub fn builder() -> ReceptionCreateTransferRuleRequestBuilder {
        <ReceptionCreateTransferRuleRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReceptionCreateTransferRuleRequestBuilder {
    label: Option<StrippedStr>,
    phone_number: Option<PhoneNumber>,
    condition: Option<StrippedStr>,
    post_dial_digits: Option<String>,
}

impl ReceptionCreateTransferRuleRequestBuilder {
    pub fn label(mut self, value: StrippedStr) -> Self {
        self.label = Some(value);
        self
    }

    pub fn phone_number(mut self, value: PhoneNumber) -> Self {
        self.phone_number = Some(value);
        self
    }

    pub fn condition(mut self, value: StrippedStr) -> Self {
        self.condition = Some(value);
        self
    }

    pub fn post_dial_digits(mut self, value: impl Into<String>) -> Self {
        self.post_dial_digits = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ReceptionCreateTransferRuleRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`label`](ReceptionCreateTransferRuleRequestBuilder::label)
    /// - [`phone_number`](ReceptionCreateTransferRuleRequestBuilder::phone_number)
    /// - [`condition`](ReceptionCreateTransferRuleRequestBuilder::condition)
    pub fn build(self) -> Result<ReceptionCreateTransferRuleRequest, BuildError> {
        Ok(ReceptionCreateTransferRuleRequest {
            label: self
                .label
                .ok_or_else(|| BuildError::missing_field("label"))?,
            phone_number: self
                .phone_number
                .ok_or_else(|| BuildError::missing_field("phone_number"))?,
            condition: self
                .condition
                .ok_or_else(|| BuildError::missing_field("condition"))?,
            post_dial_digits: self.post_dial_digits,
        })
    }
}
