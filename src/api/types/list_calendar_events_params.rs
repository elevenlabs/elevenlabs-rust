pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListCalendarEventsParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_cancelled: Option<bool>,
}

impl ListCalendarEventsParams {
    pub fn builder() -> ListCalendarEventsParamsBuilder {
        <ListCalendarEventsParamsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListCalendarEventsParamsBuilder {
    include_cancelled: Option<bool>,
}

impl ListCalendarEventsParamsBuilder {
    pub fn include_cancelled(mut self, value: bool) -> Self {
        self.include_cancelled = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListCalendarEventsParams`].
    pub fn build(self) -> Result<ListCalendarEventsParams, BuildError> {
        Ok(ListCalendarEventsParams {
            include_cancelled: self.include_cancelled,
        })
    }
}
