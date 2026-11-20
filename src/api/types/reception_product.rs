pub use crate::prelude::*;

/// Represents a product, menu item, or catalog item.
///
/// Used for restaurants (menu), e-commerce (products), salons (retail items), etc.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ReceptionProduct {
    /// Whether a catalog item's price and duration are known up front.
    ///
    /// - `fixed`: the catalog price and duration apply (the default).
    /// - `quote`: neither is known until the owner has seen the job. The
    ///   receptionist collects intake answers and files a quote request instead
    ///   of booking a slot or taking an order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pricing_mode: Option<PricingMode>,
    /// Questions the agent asks before booking or ordering this item. Required for a useful `pricing_mode='quote'` item -- without them a quote request arrives with nothing to price -- but also valid on fixed-price items that need extra detail (current hair color before a dye job, allergies before a treatment).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intake_questions: Option<Vec<ReceptionIntakeQuestion>>,
    /// Group memberships for this item. Each entry carries the group label and the item's 0-based rank within that group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<ReceptionGroupMembership>>,
    /// Display position among items with no group; `null` sorts last. Change it with the reorder endpoint.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::int::option")]
    pub ungrouped_rank: Option<i64>,
    #[serde(default)]
    pub name: StrippedStr,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<StrippedStr>,
    /// Price of the product; null for no listed price.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::float::option")]
    pub price: Option<f64>,
    /// False hides the product without deleting it.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_active: Option<bool>,
    /// False keeps the product listed (callers can ask about it) but not orderable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orderable: Option<bool>,
    /// Fulfillment options customers may choose when ordering this product.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_fulfillment_types: Option<Vec<OrderFulfillmentType>>,
    /// Locations where this product is sold; empty list means all locations.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_ids: Option<Vec<String>>,
    /// On-hand quantity by location. None means unlimited. A missing location key means zero.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stock: Option<std::collections::HashMap<String, i64>>,
    /// The product's ID.
    #[serde(default)]
    pub id: String,
    /// Unix timestamp (seconds since epoch)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at_unix: Option<UnixTimestamp>,
    /// Unix timestamp (seconds since epoch)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at_unix: Option<UnixTimestamp>,
}

impl ReceptionProduct {
    pub fn builder() -> ReceptionProductBuilder {
        <ReceptionProductBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReceptionProductBuilder {
    pricing_mode: Option<PricingMode>,
    intake_questions: Option<Vec<ReceptionIntakeQuestion>>,
    groups: Option<Vec<ReceptionGroupMembership>>,
    ungrouped_rank: Option<i64>,
    name: Option<StrippedStr>,
    description: Option<StrippedStr>,
    price: Option<f64>,
    is_active: Option<bool>,
    orderable: Option<bool>,
    allowed_fulfillment_types: Option<Vec<OrderFulfillmentType>>,
    location_ids: Option<Vec<String>>,
    stock: Option<std::collections::HashMap<String, i64>>,
    id: Option<String>,
    created_at_unix: Option<UnixTimestamp>,
    updated_at_unix: Option<UnixTimestamp>,
}

impl ReceptionProductBuilder {
    pub fn pricing_mode(mut self, value: PricingMode) -> Self {
        self.pricing_mode = Some(value);
        self
    }

    pub fn intake_questions(mut self, value: Vec<ReceptionIntakeQuestion>) -> Self {
        self.intake_questions = Some(value);
        self
    }

    pub fn groups(mut self, value: Vec<ReceptionGroupMembership>) -> Self {
        self.groups = Some(value);
        self
    }

    pub fn ungrouped_rank(mut self, value: i64) -> Self {
        self.ungrouped_rank = Some(value);
        self
    }

    pub fn name(mut self, value: StrippedStr) -> Self {
        self.name = Some(value);
        self
    }

    pub fn description(mut self, value: StrippedStr) -> Self {
        self.description = Some(value);
        self
    }

    pub fn price(mut self, value: f64) -> Self {
        self.price = Some(value);
        self
    }

    pub fn is_active(mut self, value: bool) -> Self {
        self.is_active = Some(value);
        self
    }

    pub fn orderable(mut self, value: bool) -> Self {
        self.orderable = Some(value);
        self
    }

    pub fn allowed_fulfillment_types(mut self, value: Vec<OrderFulfillmentType>) -> Self {
        self.allowed_fulfillment_types = Some(value);
        self
    }

    pub fn location_ids(mut self, value: Vec<String>) -> Self {
        self.location_ids = Some(value);
        self
    }

    pub fn stock(mut self, value: std::collections::HashMap<String, i64>) -> Self {
        self.stock = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn created_at_unix(mut self, value: UnixTimestamp) -> Self {
        self.created_at_unix = Some(value);
        self
    }

    pub fn updated_at_unix(mut self, value: UnixTimestamp) -> Self {
        self.updated_at_unix = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ReceptionProduct`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](ReceptionProductBuilder::name)
    /// - [`id`](ReceptionProductBuilder::id)
    pub fn build(self) -> Result<ReceptionProduct, BuildError> {
        Ok(ReceptionProduct {
            pricing_mode: self.pricing_mode,
            intake_questions: self.intake_questions,
            groups: self.groups,
            ungrouped_rank: self.ungrouped_rank,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            description: self.description,
            price: self.price,
            is_active: self.is_active,
            orderable: self.orderable,
            allowed_fulfillment_types: self.allowed_fulfillment_types,
            location_ids: self.location_ids,
            stock: self.stock,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            created_at_unix: self.created_at_unix,
            updated_at_unix: self.updated_at_unix,
        })
    }
}
