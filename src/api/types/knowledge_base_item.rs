pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct KnowledgeBaseItem {
    /// The type of the knowledge base
    pub r#type: KnowledgeBaseItemType,
    /// The name of the knowledge base
    #[serde(default)]
    pub name: String,
    /// The ID of the knowledge base
    #[serde(default)]
    pub id: String,
    /// The usage mode of the knowledge base
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage_mode: Option<UsageMode>,
}

impl KnowledgeBaseItem {
    pub fn builder() -> KnowledgeBaseItemBuilder {
        <KnowledgeBaseItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct KnowledgeBaseItemBuilder {
    r#type: Option<KnowledgeBaseItemType>,
    name: Option<String>,
    id: Option<String>,
    usage_mode: Option<UsageMode>,
}

impl KnowledgeBaseItemBuilder {
    pub fn r#type(mut self, value: KnowledgeBaseItemType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn usage_mode(mut self, value: UsageMode) -> Self {
        self.usage_mode = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`KnowledgeBaseItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`r#type`](KnowledgeBaseItemBuilder::type)
    /// - [`name`](KnowledgeBaseItemBuilder::name)
    /// - [`id`](KnowledgeBaseItemBuilder::id)
    pub fn build(self) -> Result<KnowledgeBaseItem, BuildError> {
        Ok(KnowledgeBaseItem {
            r#type: self
                .r#type
                .ok_or_else(|| BuildError::missing_field("r#type"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            usage_mode: self.usage_mode,
        })
    }
}
