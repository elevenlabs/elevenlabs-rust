pub use crate::prelude::*;

/// Query parameters for List Orders
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct OrdersListQueryRequest {
    /// Maximum number of orders to return per page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    /// Number of orders to skip for pagination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
    /// Filter orders by one or more statuses.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<Vec<OrderState>>,
    /// Filter orders created on or after this date.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    /// Filter orders created on or before this date.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
}

impl OrdersListQueryRequest {
    pub fn builder() -> OrdersListQueryRequestBuilder {
        <OrdersListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct OrdersListQueryRequestBuilder {
    page_size: Option<i64>,
    offset: Option<i64>,
    status: Option<Vec<OrderState>>,
    start_date: Option<String>,
    end_date: Option<String>,
}

impl OrdersListQueryRequestBuilder {
    pub fn page_size(mut self, value: i64) -> Self {
        self.page_size = Some(value);
        self
    }

    pub fn offset(mut self, value: i64) -> Self {
        self.offset = Some(value);
        self
    }

    pub fn status(mut self, value: Vec<OrderState>) -> Self {
        self.status = Some(value);
        self
    }

    pub fn start_date(mut self, value: impl Into<String>) -> Self {
        self.start_date = Some(value.into());
        self
    }

    pub fn end_date(mut self, value: impl Into<String>) -> Self {
        self.end_date = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`OrdersListQueryRequest`].
    pub fn build(self) -> Result<OrdersListQueryRequest, BuildError> {
        Ok(OrdersListQueryRequest {
            page_size: self.page_size,
            offset: self.offset,
            status: self.status,
            start_date: self.start_date,
            end_date: self.end_date,
        })
    }
}
