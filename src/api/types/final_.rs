pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Final_ {
    /// Marks the end of the closing flush sequence.
    pub is_final: FinalIsFinal,
}

impl Final_ {
    pub fn builder() -> Final_Builder {
        <Final_Builder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct Final_Builder {
    is_final: Option<FinalIsFinal>,
}

impl Final_Builder {
    pub fn is_final(mut self, value: FinalIsFinal) -> Self {
        self.is_final = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`Final_`].
    /// This method will fail if any of the following fields are not set:
    /// - [`is_final`](Final_Builder::is_final)
    pub fn build(self) -> Result<Final_, BuildError> {
        Ok(Final_ {
            is_final: self
                .is_final
                .ok_or_else(|| BuildError::missing_field("is_final"))?,
        })
    }
}
