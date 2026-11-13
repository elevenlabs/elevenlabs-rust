pub use crate::prelude::*;

/// Query parameters for Get Voice Accents
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct AccentsGetQueryRequest {
    /// If provided, only accents for this language code are returned.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    /// If provided, returns the accents available for this model. Defaults to the most complete accent list when omitted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_id: Option<String>,
}

impl AccentsGetQueryRequest {
    pub fn builder() -> AccentsGetQueryRequestBuilder {
        <AccentsGetQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AccentsGetQueryRequestBuilder {
    language: Option<String>,
    model_id: Option<String>,
}

impl AccentsGetQueryRequestBuilder {
    pub fn language(mut self, value: impl Into<String>) -> Self {
        self.language = Some(value.into());
        self
    }

    pub fn model_id(mut self, value: impl Into<String>) -> Self {
        self.model_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AccentsGetQueryRequest`].
    pub fn build(self) -> Result<AccentsGetQueryRequest, BuildError> {
        Ok(AccentsGetQueryRequest {
            language: self.language,
            model_id: self.model_id,
        })
    }
}
