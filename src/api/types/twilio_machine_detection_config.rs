pub use crate::prelude::*;

/// How to run Twilio's carrier-level answering machine detection (AMD) on a call.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TwilioMachineDetectionConfig {
    /// Which Twilio answering-machine-detection (AMD) mode to request for a call.
    ///
    /// `enable` returns a verdict as soon as Twilio can tell a human from a machine.
    /// `detect_message_end` additionally waits for the greeting to finish, which is what
    /// distinguishes the `machine_end_*` verdicts, at the cost of a later result.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<TwilioMachineDetectionMode>,
    /// Actions to take on the call per AMD verdict. Each action can appear in at most one rule, so list all of its verdicts together. Verdicts without a rule leave the call running.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions: Option<Vec<TwilioMachineDetectionRule>>,
}

impl TwilioMachineDetectionConfig {
    pub fn builder() -> TwilioMachineDetectionConfigBuilder {
        <TwilioMachineDetectionConfigBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TwilioMachineDetectionConfigBuilder {
    mode: Option<TwilioMachineDetectionMode>,
    actions: Option<Vec<TwilioMachineDetectionRule>>,
}

impl TwilioMachineDetectionConfigBuilder {
    pub fn mode(mut self, value: TwilioMachineDetectionMode) -> Self {
        self.mode = Some(value);
        self
    }

    pub fn actions(mut self, value: Vec<TwilioMachineDetectionRule>) -> Self {
        self.actions = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TwilioMachineDetectionConfig`].
    pub fn build(self) -> Result<TwilioMachineDetectionConfig, BuildError> {
        Ok(TwilioMachineDetectionConfig {
            mode: self.mode,
            actions: self.actions,
        })
    }
}
