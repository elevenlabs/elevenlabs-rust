pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct EndProcedureToolConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub procedures: Option<std::collections::HashMap<String, EndProcedureToolConfigProcedures>>,
}

impl EndProcedureToolConfig {
    pub fn builder() -> EndProcedureToolConfigBuilder {
        <EndProcedureToolConfigBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EndProcedureToolConfigBuilder {
    procedures: Option<std::collections::HashMap<String, EndProcedureToolConfigProcedures>>,
}

impl EndProcedureToolConfigBuilder {
    pub fn procedures(
        mut self,
        value: std::collections::HashMap<String, EndProcedureToolConfigProcedures>,
    ) -> Self {
        self.procedures = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`EndProcedureToolConfig`].
    pub fn build(self) -> Result<EndProcedureToolConfig, BuildError> {
        Ok(EndProcedureToolConfig {
            procedures: self.procedures,
        })
    }
}
