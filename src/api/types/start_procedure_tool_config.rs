pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct StartProcedureToolConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub procedures: Option<std::collections::HashMap<String, StartProcedureToolConfigProcedures>>,
}

impl StartProcedureToolConfig {
    pub fn builder() -> StartProcedureToolConfigBuilder {
        <StartProcedureToolConfigBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct StartProcedureToolConfigBuilder {
    procedures: Option<std::collections::HashMap<String, StartProcedureToolConfigProcedures>>,
}

impl StartProcedureToolConfigBuilder {
    pub fn procedures(
        mut self,
        value: std::collections::HashMap<String, StartProcedureToolConfigProcedures>,
    ) -> Self {
        self.procedures = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`StartProcedureToolConfig`].
    pub fn build(self) -> Result<StartProcedureToolConfig, BuildError> {
        Ok(StartProcedureToolConfig {
            procedures: self.procedures,
        })
    }
}
