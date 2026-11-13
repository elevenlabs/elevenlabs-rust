pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct BodyGetDependentAgentsForMultipleDocumentsV1ConvaiKnowledgeBaseDependentAgentsPost {
    /// The ids of documents or folders from the knowledge base.
    #[serde(default)]
    pub document_ids: Vec<String>,
}

impl BodyGetDependentAgentsForMultipleDocumentsV1ConvaiKnowledgeBaseDependentAgentsPost {
    pub fn builder(
    ) -> BodyGetDependentAgentsForMultipleDocumentsV1ConvaiKnowledgeBaseDependentAgentsPostBuilder
    {
        <BodyGetDependentAgentsForMultipleDocumentsV1ConvaiKnowledgeBaseDependentAgentsPostBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BodyGetDependentAgentsForMultipleDocumentsV1ConvaiKnowledgeBaseDependentAgentsPostBuilder
{
    document_ids: Option<Vec<String>>,
}

impl BodyGetDependentAgentsForMultipleDocumentsV1ConvaiKnowledgeBaseDependentAgentsPostBuilder {
    pub fn document_ids(mut self, value: Vec<String>) -> Self {
        self.document_ids = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BodyGetDependentAgentsForMultipleDocumentsV1ConvaiKnowledgeBaseDependentAgentsPost`].
    /// This method will fail if any of the following fields are not set:
    /// - [`document_ids`](BodyGetDependentAgentsForMultipleDocumentsV1ConvaiKnowledgeBaseDependentAgentsPostBuilder::document_ids)
    pub fn build(
        self,
    ) -> Result<
        BodyGetDependentAgentsForMultipleDocumentsV1ConvaiKnowledgeBaseDependentAgentsPost,
        BuildError,
    > {
        Ok(
            BodyGetDependentAgentsForMultipleDocumentsV1ConvaiKnowledgeBaseDependentAgentsPost {
                document_ids: self
                    .document_ids
                    .ok_or_else(|| BuildError::missing_field("document_ids"))?,
            },
        )
    }
}
