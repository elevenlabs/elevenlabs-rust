pub use crate::prelude::*;

/// Request for Create (body + query parameters)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CompositionPlanCreateRequest {
    /// When enable_logging is set to false zero retention mode will be used for the request. Zero retention mode may only be used by enterprise customers.
    #[serde(skip)]
    pub enable_logging: Option<bool>,
    pub body: BodyGenerateCompositionPlanV1MusicPlanPost,
}

impl CompositionPlanCreateRequest {
    pub fn builder() -> CompositionPlanCreateRequestBuilder {
        <CompositionPlanCreateRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CompositionPlanCreateRequestBuilder {
    enable_logging: Option<bool>,
    body: Option<BodyGenerateCompositionPlanV1MusicPlanPost>,
}

impl CompositionPlanCreateRequestBuilder {
    pub fn enable_logging(mut self, value: bool) -> Self {
        self.enable_logging = Some(value);
        self
    }

    pub fn body(mut self, value: BodyGenerateCompositionPlanV1MusicPlanPost) -> Self {
        self.body = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CompositionPlanCreateRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`body`](CompositionPlanCreateRequestBuilder::body)
    pub fn build(self) -> Result<CompositionPlanCreateRequest, BuildError> {
        Ok(CompositionPlanCreateRequest {
            enable_logging: self.enable_logging,
            body: self.body.ok_or_else(|| BuildError::missing_field("body"))?,
        })
    }
}
