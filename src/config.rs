use crate::environment::Environment;
use std::collections::HashMap;
use std::time::Duration;

/// How the SDK exchanges credentials for an access token.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OAuthTokenExchangeConfig {
    /// Request body field carrying the client id (e.g. `"client_id"`).
    pub client_id_property: String,
    /// Request body field carrying the client secret.
    pub client_secret_property: String,
    /// Static request body properties sent verbatim (e.g. `{"grant_type": "client_credentials"}`).
    pub extra_request_properties: HashMap<String, String>,
    /// Response field holding the access token (e.g. `"access_token"`).
    pub access_token_property: String,
    /// Response field holding the token lifetime in seconds (e.g. `"expires_in"`).
    pub expires_in_property: String,
}

/// The resolved OAuth setup: where to exchange credentials and how to read the reply.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OAuthConfig {
    /// The token endpoint path (e.g. `"/token"`).
    pub token_endpoint: String,
    /// Request/response property-name mapping for the token exchange.
    pub exchange: OAuthTokenExchangeConfig,
}

// NOTE: fern also carries `token_provider: Arc<OAuthTokenProvider>` here — the runtime that
// caches and refreshes the token. It is omitted deliberately: emitting the field without a
// working provider behind it would be a type that exists and does nothing, which reads as
// parity without being it.

impl Default for OAuthTokenExchangeConfig {
    fn default() -> Self {
        Self {
            client_id_property: "client_id".to_string(),
            client_secret_property: "client_secret".to_string(),
            extra_request_properties: HashMap::new(),
            access_token_property: "access_token".to_string(),
            expires_in_property: "expires_in".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ClientConfig {
    pub base_url: String,
    pub timeout: Duration,
    pub max_retries: u32,
    pub api_key: Option<String>,
    pub token: Option<String>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub client_id: Option<String>,
    pub client_secret: Option<String>,
    pub custom_headers: HashMap<String, String>,
    pub oauth_token_endpoint: Option<String>,
    pub oauth_token_exchange: Option<OAuthTokenExchangeConfig>,
    pub user_agent: String,
    /// Optional custom `reqwest` client, used as-is for every request.
    /// When set, it owns all transport-level configuration (TLS, proxies, timeout,
    /// user agent); when `None` the SDK builds its own client from `timeout` and
    /// `user_agent`.
    pub reqwest_client: Option<reqwest::Client>,
}

impl Default for ClientConfig {
    fn default() -> Self {
        Self {
            base_url: Environment::default().url().to_string(),
            timeout: Duration::from_secs(240),
            max_retries: 3,
            api_key: std::env::var("ELEVENLABS_API_KEY").ok(),
            token: None,
            username: None,
            password: None,
            client_id: None,
            client_secret: None,
            custom_headers: Default::default(),
            oauth_token_endpoint: None,
            oauth_token_exchange: None,
            user_agent: "postman-codegen/3.33.0 elevenlabs-rust-sdk/1.0.0 (rust)".to_string(),
            reqwest_client: None,
        }
    }
}

/// A closure that applies a per-call configuration override to a [ClientConfig].
pub type RequestOption = Box<dyn Fn(&mut ClientConfig) + Send + Sync>;

#[derive(Debug, Default)]
pub struct ClientConfigBuilder {
    base_url: Option<String>,
    timeout: Option<Duration>,
    max_retries: Option<u32>,
    api_key: Option<String>,
    token: Option<String>,
    username: Option<String>,
    password: Option<String>,
    client_id: Option<String>,
    client_secret: Option<String>,
    custom_headers: Option<HashMap<String, String>>,
    oauth_token_endpoint: Option<String>,
    oauth_token_exchange: Option<OAuthTokenExchangeConfig>,
    user_agent: Option<String>,
    reqwest_client: Option<reqwest::Client>,
}

impl ClientConfig {
    pub fn builder() -> ClientConfigBuilder {
        ClientConfigBuilder::default()
    }
}

impl ClientConfigBuilder {
    pub fn base_url(mut self, base_url: impl Into<String>) -> Self {
        self.base_url = Some(base_url.into());
        self
    }

    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }

    pub fn environment(mut self, environment: Environment) -> Self {
        self.base_url = Some(environment.url().to_string());
        self
    }

    pub fn max_retries(mut self, max_retries: u32) -> Self {
        self.max_retries = Some(max_retries);
        self
    }

    pub fn api_key(mut self, api_key: impl Into<String>) -> Self {
        self.api_key = Some(api_key.into());
        self
    }

    pub fn token(mut self, token: impl Into<String>) -> Self {
        self.token = Some(token.into());
        self
    }

    pub fn username(mut self, username: impl Into<String>) -> Self {
        self.username = Some(username.into());
        self
    }

    pub fn password(mut self, password: impl Into<String>) -> Self {
        self.password = Some(password.into());
        self
    }

    pub fn client_id(mut self, client_id: impl Into<String>) -> Self {
        self.client_id = Some(client_id.into());
        self
    }

    pub fn client_secret(mut self, client_secret: impl Into<String>) -> Self {
        self.client_secret = Some(client_secret.into());
        self
    }

    pub fn custom_headers(mut self, custom_headers: HashMap<String, String>) -> Self {
        self.custom_headers = Some(custom_headers);
        self
    }

    pub fn custom_header(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.custom_headers
            .get_or_insert_with(HashMap::new)
            .insert(key.into(), value.into());
        self
    }

    pub fn oauth_token_endpoint(mut self, oauth_token_endpoint: impl Into<String>) -> Self {
        self.oauth_token_endpoint = Some(oauth_token_endpoint.into());
        self
    }

    pub fn oauth_token_exchange(mut self, oauth_token_exchange: OAuthTokenExchangeConfig) -> Self {
        self.oauth_token_exchange = Some(oauth_token_exchange);
        self
    }

    pub fn oauth_credentials(
        mut self,
        client_id: impl Into<String>,
        client_secret: impl Into<String>,
    ) -> Self {
        self.client_id = Some(client_id.into());
        self.client_secret = Some(client_secret.into());
        self
    }

    pub fn user_agent(mut self, user_agent: impl Into<String>) -> Self {
        self.user_agent = Some(user_agent.into());
        self
    }

    pub fn reqwest_client(mut self, reqwest_client: reqwest::Client) -> Self {
        self.reqwest_client = Some(reqwest_client);
        self
    }

    pub fn build(self) -> Result<ClientConfig, String> {
        Ok(ClientConfig {
            base_url: self
                .base_url
                .unwrap_or_else(|| Environment::default().url().to_string()),
            timeout: self.timeout.unwrap_or(Duration::from_secs(240)),
            max_retries: self.max_retries.unwrap_or(3),
            api_key: self
                .api_key
                .or_else(|| std::env::var("ELEVENLABS_API_KEY").ok()),
            token: self.token,
            username: self.username,
            password: self.password,
            client_id: self.client_id,
            client_secret: self.client_secret,
            custom_headers: self.custom_headers.unwrap_or_default(),
            oauth_token_endpoint: self.oauth_token_endpoint,
            oauth_token_exchange: self.oauth_token_exchange,
            user_agent: self.user_agent.unwrap_or_else(|| {
                "postman-codegen/3.33.0 elevenlabs-rust-sdk/1.0.0 (rust)".to_string()
            }),
            reqwest_client: self.reqwest_client,
        })
    }
}

pub fn with_base_url(base_url: impl Into<String>) -> RequestOption {
    let val = base_url.into();
    Box::new(move |c| {
        c.base_url = val.clone();
    })
}
pub fn with_timeout(timeout: Duration) -> RequestOption {
    Box::new(move |c| {
        c.timeout = timeout;
    })
}
pub fn with_max_retries(max_retries: u32) -> RequestOption {
    Box::new(move |c| {
        c.max_retries = max_retries;
    })
}
pub fn with_api_key(api_key: impl Into<String>) -> RequestOption {
    let val = api_key.into();
    Box::new(move |c| {
        c.api_key = Some(val.clone());
    })
}
