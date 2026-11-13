pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ConstantSchemaOverride {
    /// The constant value to use
    #[serde(default)]
    pub constant_value: Option<ConstantSchemaOverrideConstantValue>,
}

impl ConstantSchemaOverride {
    pub fn builder() -> ConstantSchemaOverrideBuilder {
        <ConstantSchemaOverrideBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConstantSchemaOverrideBuilder {
    constant_value: Option<ConstantSchemaOverrideConstantValue>,
}

impl ConstantSchemaOverrideBuilder {
    pub fn constant_value(mut self, value: ConstantSchemaOverrideConstantValue) -> Self {
        self.constant_value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ConstantSchemaOverride`].
    pub fn build(self) -> Result<ConstantSchemaOverride, BuildError> {
        Ok(ConstantSchemaOverride {
            constant_value: self.constant_value,
        })
    }
}
