pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FileContentPart {
    pub source: FileContentPartSource,
}

impl FileContentPart {
    pub fn builder() -> FileContentPartBuilder {
        <FileContentPartBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct FileContentPartBuilder {
    source: Option<FileContentPartSource>,
}

impl FileContentPartBuilder {
    pub fn source(mut self, value: FileContentPartSource) -> Self {
        self.source = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`FileContentPart`].
    /// This method will fail if any of the following fields are not set:
    /// - [`source`](FileContentPartBuilder::source)
    pub fn build(self) -> Result<FileContentPart, BuildError> {
        Ok(FileContentPart {
            source: self
                .source
                .ok_or_else(|| BuildError::missing_field("source"))?,
        })
    }
}
