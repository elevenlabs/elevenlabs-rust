pub use crate::prelude::*;

/// Query parameters for List similar voices
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct FindSimilarVoicesQueryRequest {
    /// Voice category used for filtering
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<FindSimilarVoicesCategory>,
    /// Gender used for filtering
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gender: Option<String>,
    /// Age used for filtering
    #[serde(skip_serializing_if = "Option::is_none")]
    pub age: Option<String>,
    /// Use-case used for filtering
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_cases: Option<Vec<String>>,
    /// Filter voices with a minimum notice period of the given number of days.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_notice_period_days: Option<i64>,
    /// Include/exclude voices with custom rates
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_custom_rates: Option<bool>,
    /// Include/exclude voices that are live moderated
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_live_moderated: Option<bool>,
    /// Filter voices by public owner ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_id: Option<String>,
}

impl FindSimilarVoicesQueryRequest {
    pub fn builder() -> FindSimilarVoicesQueryRequestBuilder {
        <FindSimilarVoicesQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct FindSimilarVoicesQueryRequestBuilder {
    category: Option<FindSimilarVoicesCategory>,
    gender: Option<String>,
    age: Option<String>,
    use_cases: Option<Vec<String>>,
    min_notice_period_days: Option<i64>,
    include_custom_rates: Option<bool>,
    include_live_moderated: Option<bool>,
    owner_id: Option<String>,
}

impl FindSimilarVoicesQueryRequestBuilder {
    pub fn category(mut self, value: FindSimilarVoicesCategory) -> Self {
        self.category = Some(value);
        self
    }

    pub fn gender(mut self, value: impl Into<String>) -> Self {
        self.gender = Some(value.into());
        self
    }

    pub fn age(mut self, value: impl Into<String>) -> Self {
        self.age = Some(value.into());
        self
    }

    pub fn use_cases(mut self, value: Vec<String>) -> Self {
        self.use_cases = Some(value);
        self
    }

    pub fn min_notice_period_days(mut self, value: i64) -> Self {
        self.min_notice_period_days = Some(value);
        self
    }

    pub fn include_custom_rates(mut self, value: bool) -> Self {
        self.include_custom_rates = Some(value);
        self
    }

    pub fn include_live_moderated(mut self, value: bool) -> Self {
        self.include_live_moderated = Some(value);
        self
    }

    pub fn owner_id(mut self, value: impl Into<String>) -> Self {
        self.owner_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`FindSimilarVoicesQueryRequest`].
    pub fn build(self) -> Result<FindSimilarVoicesQueryRequest, BuildError> {
        Ok(FindSimilarVoicesQueryRequest {
            category: self.category,
            gender: self.gender,
            age: self.age,
            use_cases: self.use_cases,
            min_notice_period_days: self.min_notice_period_days,
            include_custom_rates: self.include_custom_rates,
            include_live_moderated: self.include_live_moderated,
            owner_id: self.owner_id,
        })
    }
}
