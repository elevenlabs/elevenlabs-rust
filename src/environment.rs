use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum Environment {
    #[serde(rename = "default")]
    #[default]
    Default,
    #[serde(rename = "PRODUCTION")]
    Production,
    #[serde(rename = "PRODUCTION_US")]
    ProductionUs,
    #[serde(rename = "PRODUCTION_EU")]
    ProductionEu,
    #[serde(rename = "PRODUCTION_INDIA")]
    ProductionIndia,
    #[serde(rename = "PRODUCTION_SINGAPORE")]
    ProductionSingapore,
}

impl Environment {
    pub fn url(&self) -> &'static str {
        match self {
            Environment::Default => "https://api.elevenlabs.io/",
            Environment::Production => "https://api.elevenlabs.io/",
            Environment::ProductionUs => "https://api.us.elevenlabs.io/",
            Environment::ProductionEu => "https://api.eu.residency.elevenlabs.io",
            Environment::ProductionIndia => "https://api.in.residency.elevenlabs.io",
            Environment::ProductionSingapore => "https://api.sg.residency.elevenlabs.io",
        }
    }
}
