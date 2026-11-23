use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DefaultUrls {
    pub base: String,
    pub production: String,
    pub production_us: String,
    pub production_eu: String,
    pub production_india: String,
    pub production_singapore: String,
}
impl Default for DefaultUrls {
    fn default() -> Self {
        Self {
            base: "https://api.elevenlabs.io".to_string(),
            production: "wss://api.elevenlabs.io".to_string(),
            production_us: "wss://api.us.elevenlabs.io".to_string(),
            production_eu: "wss://api.eu.residency.elevenlabs.io".to_string(),
            production_india: "wss://api.in.residency.elevenlabs.io".to_string(),
            production_singapore: "wss://api.sg.residency.elevenlabs.io".to_string(),
        }
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductionUrls {
    pub base: String,
    pub production: String,
    pub production_us: String,
    pub production_eu: String,
    pub production_india: String,
    pub production_singapore: String,
}
impl Default for ProductionUrls {
    fn default() -> Self {
        Self {
            base: "https://api.elevenlabs.io".to_string(),
            production: "wss://api.elevenlabs.io".to_string(),
            production_us: "wss://api.us.elevenlabs.io".to_string(),
            production_eu: "wss://api.eu.residency.elevenlabs.io".to_string(),
            production_india: "wss://api.in.residency.elevenlabs.io".to_string(),
            production_singapore: "wss://api.sg.residency.elevenlabs.io".to_string(),
        }
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductionUsUrls {
    pub base: String,
    pub production: String,
    pub production_us: String,
    pub production_eu: String,
    pub production_india: String,
    pub production_singapore: String,
}
impl Default for ProductionUsUrls {
    fn default() -> Self {
        Self {
            base: "https://api.us.elevenlabs.io".to_string(),
            production: "wss://api.elevenlabs.io".to_string(),
            production_us: "wss://api.us.elevenlabs.io".to_string(),
            production_eu: "wss://api.eu.residency.elevenlabs.io".to_string(),
            production_india: "wss://api.in.residency.elevenlabs.io".to_string(),
            production_singapore: "wss://api.sg.residency.elevenlabs.io".to_string(),
        }
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductionEuUrls {
    pub base: String,
    pub production: String,
    pub production_us: String,
    pub production_eu: String,
    pub production_india: String,
    pub production_singapore: String,
}
impl Default for ProductionEuUrls {
    fn default() -> Self {
        Self {
            base: "https://api.eu.residency.elevenlabs.io".to_string(),
            production: "wss://api.elevenlabs.io".to_string(),
            production_us: "wss://api.us.elevenlabs.io".to_string(),
            production_eu: "wss://api.eu.residency.elevenlabs.io".to_string(),
            production_india: "wss://api.in.residency.elevenlabs.io".to_string(),
            production_singapore: "wss://api.sg.residency.elevenlabs.io".to_string(),
        }
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductionIndiaUrls {
    pub base: String,
    pub production: String,
    pub production_us: String,
    pub production_eu: String,
    pub production_india: String,
    pub production_singapore: String,
}
impl Default for ProductionIndiaUrls {
    fn default() -> Self {
        Self {
            base: "https://api.in.residency.elevenlabs.io".to_string(),
            production: "wss://api.elevenlabs.io".to_string(),
            production_us: "wss://api.us.elevenlabs.io".to_string(),
            production_eu: "wss://api.eu.residency.elevenlabs.io".to_string(),
            production_india: "wss://api.in.residency.elevenlabs.io".to_string(),
            production_singapore: "wss://api.sg.residency.elevenlabs.io".to_string(),
        }
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductionSingaporeUrls {
    pub base: String,
    pub production: String,
    pub production_us: String,
    pub production_eu: String,
    pub production_india: String,
    pub production_singapore: String,
}
impl Default for ProductionSingaporeUrls {
    fn default() -> Self {
        Self {
            base: "https://api.sg.residency.elevenlabs.io".to_string(),
            production: "wss://api.elevenlabs.io".to_string(),
            production_us: "wss://api.us.elevenlabs.io".to_string(),
            production_eu: "wss://api.eu.residency.elevenlabs.io".to_string(),
            production_india: "wss://api.in.residency.elevenlabs.io".to_string(),
            production_singapore: "wss://api.sg.residency.elevenlabs.io".to_string(),
        }
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Environment {
    Default(DefaultUrls),
    Production(ProductionUrls),
    ProductionUs(ProductionUsUrls),
    ProductionEu(ProductionEuUrls),
    ProductionIndia(ProductionIndiaUrls),
    ProductionSingapore(ProductionSingaporeUrls),
}
impl Environment {
    pub fn production() -> Self {
        Self::Production(ProductionUrls::default())
    }

    pub fn production_us() -> Self {
        Self::ProductionUs(ProductionUsUrls::default())
    }

    pub fn production_eu() -> Self {
        Self::ProductionEu(ProductionEuUrls::default())
    }

    pub fn production_india() -> Self {
        Self::ProductionIndia(ProductionIndiaUrls::default())
    }

    pub fn production_singapore() -> Self {
        Self::ProductionSingapore(ProductionSingaporeUrls::default())
    }

    pub fn url(&self) -> &str {
        match self {
            Self::Default(urls) => &urls.base,
            Self::Production(urls) => &urls.base,
            Self::ProductionUs(urls) => &urls.base,
            Self::ProductionEu(urls) => &urls.base,
            Self::ProductionIndia(urls) => &urls.base,
            Self::ProductionSingapore(urls) => &urls.base,
        }
    }

    pub fn base_url(&self) -> &str {
        match self {
            Self::Default(urls) => &urls.base,
            Self::Production(urls) => &urls.base,
            Self::ProductionUs(urls) => &urls.base,
            Self::ProductionEu(urls) => &urls.base,
            Self::ProductionIndia(urls) => &urls.base,
            Self::ProductionSingapore(urls) => &urls.base,
        }
    }

    pub fn production_url(&self) -> &str {
        match self {
            Self::Default(urls) => &urls.production,
            Self::Production(urls) => &urls.production,
            Self::ProductionUs(urls) => &urls.production,
            Self::ProductionEu(urls) => &urls.production,
            Self::ProductionIndia(urls) => &urls.production,
            Self::ProductionSingapore(urls) => &urls.production,
        }
    }

    pub fn production_us_url(&self) -> &str {
        match self {
            Self::Default(urls) => &urls.production_us,
            Self::Production(urls) => &urls.production_us,
            Self::ProductionUs(urls) => &urls.production_us,
            Self::ProductionEu(urls) => &urls.production_us,
            Self::ProductionIndia(urls) => &urls.production_us,
            Self::ProductionSingapore(urls) => &urls.production_us,
        }
    }

    pub fn production_eu_url(&self) -> &str {
        match self {
            Self::Default(urls) => &urls.production_eu,
            Self::Production(urls) => &urls.production_eu,
            Self::ProductionUs(urls) => &urls.production_eu,
            Self::ProductionEu(urls) => &urls.production_eu,
            Self::ProductionIndia(urls) => &urls.production_eu,
            Self::ProductionSingapore(urls) => &urls.production_eu,
        }
    }

    pub fn production_india_url(&self) -> &str {
        match self {
            Self::Default(urls) => &urls.production_india,
            Self::Production(urls) => &urls.production_india,
            Self::ProductionUs(urls) => &urls.production_india,
            Self::ProductionEu(urls) => &urls.production_india,
            Self::ProductionIndia(urls) => &urls.production_india,
            Self::ProductionSingapore(urls) => &urls.production_india,
        }
    }

    pub fn production_singapore_url(&self) -> &str {
        match self {
            Self::Default(urls) => &urls.production_singapore,
            Self::Production(urls) => &urls.production_singapore,
            Self::ProductionUs(urls) => &urls.production_singapore,
            Self::ProductionEu(urls) => &urls.production_singapore,
            Self::ProductionIndia(urls) => &urls.production_singapore,
            Self::ProductionSingapore(urls) => &urls.production_singapore,
        }
    }
}
impl Default for Environment {
    fn default() -> Self {
        Self::Default(DefaultUrls::default())
    }
}
