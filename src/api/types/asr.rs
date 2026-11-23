pub use crate::prelude::*;

/// Configuration for conversational transcription
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct Asr {
    /// Keywords to boost prediction probability for
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keywords: Option<Vec<String>>,
}

impl Asr {
    pub fn builder() -> AsrBuilder {
        <AsrBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AsrBuilder {
    keywords: Option<Vec<String>>,
}

impl AsrBuilder {
    pub fn keywords(mut self, value: Vec<String>) -> Self {
        self.keywords = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`Asr`].
    pub fn build(self) -> Result<Asr, BuildError> {
        Ok(Asr {
            keywords: self.keywords,
        })
    }
}
