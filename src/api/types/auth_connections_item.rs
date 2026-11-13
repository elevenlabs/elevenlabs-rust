pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "auth_type")]
#[non_exhaustive]
pub enum AuthConnectionsItem {
    #[serde(rename = "api_integration_oauth2_auth_code")]
    ApiIntegrationOauth2AuthCode {
        #[serde(flatten)]
        data: ApiIntegrationOAuth2AuthCodeResponse,
    },

    #[serde(rename = "api_integration_oauth2_custom_app")]
    ApiIntegrationOauth2CustomApp {
        #[serde(flatten)]
        data: ApiIntegrationOAuth2CustomAppResponse,
    },

    #[serde(rename = "basic_auth")]
    BasicAuth {
        #[serde(flatten)]
        data: BasicAuthResponse,
    },

    #[serde(rename = "bearer_auth")]
    BearerAuth {
        #[serde(flatten)]
        data: BearerAuthResponse,
    },

    #[serde(rename = "custom_header_auth")]
    CustomHeaderAuth {
        #[serde(flatten)]
        data: CustomHeaderAuthResponse,
    },

    #[serde(rename = "mtls")]
    Mtls {
        #[serde(flatten)]
        data: MtlsAuthResponse,
    },

    #[serde(rename = "oauth2_client_credentials")]
    Oauth2ClientCredentials {
        #[serde(flatten)]
        data: OAuth2ClientCredsResponse,
    },

    #[serde(rename = "oauth2_jwt")]
    Oauth2Jwt {
        #[serde(flatten)]
        data: OAuth2JwtResponse,
    },

    #[serde(rename = "private_key_jwt")]
    PrivateKeyJwt {
        #[serde(flatten)]
        data: PrivateKeyJwtResponse,
    },

    #[serde(rename = "refresh_token_auth")]
    RefreshTokenAuth {
        #[serde(flatten)]
        data: RefreshTokenAuthResponse,
    },

    #[serde(rename = "slack_bot_auth")]
    SlackBotAuth {
        #[serde(flatten)]
        data: SlackBotAuthResponse,
    },

    #[serde(rename = "url_secret")]
    UrlSecret {
        #[serde(flatten)]
        data: UrlSecretAuthResponse,
    },

    #[serde(rename = "whatsapp_auth")]
    WhatsappAuth {
        #[serde(flatten)]
        data: WhatsAppAuthResponse,
    },

    /// Catch-all variant for unrecognized discriminant values.
    /// If the server sends a discriminant not recognized by the current SDK
    /// version, the raw payload is captured here so callers can still inspect it.
    #[serde(untagged)]
    __Unknown(serde_json::Value),
}

impl AuthConnectionsItem {
    pub fn is_api_integration_oauth2_auth_code(&self) -> bool {
        matches!(self, Self::ApiIntegrationOauth2AuthCode { .. })
    }

    pub fn as_api_integration_oauth2_auth_code(
        &self,
    ) -> Option<&ApiIntegrationOAuth2AuthCodeResponse> {
        match self {
            Self::ApiIntegrationOauth2AuthCode { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_api_integration_oauth2_auth_code(
        self,
    ) -> Option<ApiIntegrationOAuth2AuthCodeResponse> {
        match self {
            Self::ApiIntegrationOauth2AuthCode { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_api_integration_oauth2_custom_app(&self) -> bool {
        matches!(self, Self::ApiIntegrationOauth2CustomApp { .. })
    }

    pub fn as_api_integration_oauth2_custom_app(
        &self,
    ) -> Option<&ApiIntegrationOAuth2CustomAppResponse> {
        match self {
            Self::ApiIntegrationOauth2CustomApp { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_api_integration_oauth2_custom_app(
        self,
    ) -> Option<ApiIntegrationOAuth2CustomAppResponse> {
        match self {
            Self::ApiIntegrationOauth2CustomApp { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_basic_auth(&self) -> bool {
        matches!(self, Self::BasicAuth { .. })
    }

    pub fn as_basic_auth(&self) -> Option<&BasicAuthResponse> {
        match self {
            Self::BasicAuth { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_basic_auth(self) -> Option<BasicAuthResponse> {
        match self {
            Self::BasicAuth { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_bearer_auth(&self) -> bool {
        matches!(self, Self::BearerAuth { .. })
    }

    pub fn as_bearer_auth(&self) -> Option<&BearerAuthResponse> {
        match self {
            Self::BearerAuth { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_bearer_auth(self) -> Option<BearerAuthResponse> {
        match self {
            Self::BearerAuth { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_custom_header_auth(&self) -> bool {
        matches!(self, Self::CustomHeaderAuth { .. })
    }

    pub fn as_custom_header_auth(&self) -> Option<&CustomHeaderAuthResponse> {
        match self {
            Self::CustomHeaderAuth { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_custom_header_auth(self) -> Option<CustomHeaderAuthResponse> {
        match self {
            Self::CustomHeaderAuth { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_mtls(&self) -> bool {
        matches!(self, Self::Mtls { .. })
    }

    pub fn as_mtls(&self) -> Option<&MtlsAuthResponse> {
        match self {
            Self::Mtls { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_mtls(self) -> Option<MtlsAuthResponse> {
        match self {
            Self::Mtls { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_oauth2_client_credentials(&self) -> bool {
        matches!(self, Self::Oauth2ClientCredentials { .. })
    }

    pub fn as_oauth2_client_credentials(&self) -> Option<&OAuth2ClientCredsResponse> {
        match self {
            Self::Oauth2ClientCredentials { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_oauth2_client_credentials(self) -> Option<OAuth2ClientCredsResponse> {
        match self {
            Self::Oauth2ClientCredentials { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_oauth2_jwt(&self) -> bool {
        matches!(self, Self::Oauth2Jwt { .. })
    }

    pub fn as_oauth2_jwt(&self) -> Option<&OAuth2JwtResponse> {
        match self {
            Self::Oauth2Jwt { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_oauth2_jwt(self) -> Option<OAuth2JwtResponse> {
        match self {
            Self::Oauth2Jwt { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_private_key_jwt(&self) -> bool {
        matches!(self, Self::PrivateKeyJwt { .. })
    }

    pub fn as_private_key_jwt(&self) -> Option<&PrivateKeyJwtResponse> {
        match self {
            Self::PrivateKeyJwt { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_private_key_jwt(self) -> Option<PrivateKeyJwtResponse> {
        match self {
            Self::PrivateKeyJwt { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_refresh_token_auth(&self) -> bool {
        matches!(self, Self::RefreshTokenAuth { .. })
    }

    pub fn as_refresh_token_auth(&self) -> Option<&RefreshTokenAuthResponse> {
        match self {
            Self::RefreshTokenAuth { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_refresh_token_auth(self) -> Option<RefreshTokenAuthResponse> {
        match self {
            Self::RefreshTokenAuth { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_slack_bot_auth(&self) -> bool {
        matches!(self, Self::SlackBotAuth { .. })
    }

    pub fn as_slack_bot_auth(&self) -> Option<&SlackBotAuthResponse> {
        match self {
            Self::SlackBotAuth { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_slack_bot_auth(self) -> Option<SlackBotAuthResponse> {
        match self {
            Self::SlackBotAuth { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_url_secret(&self) -> bool {
        matches!(self, Self::UrlSecret { .. })
    }

    pub fn as_url_secret(&self) -> Option<&UrlSecretAuthResponse> {
        match self {
            Self::UrlSecret { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_url_secret(self) -> Option<UrlSecretAuthResponse> {
        match self {
            Self::UrlSecret { data } => Some(data),
            _ => None,
        }
    }

    pub fn is_whatsapp_auth(&self) -> bool {
        matches!(self, Self::WhatsappAuth { .. })
    }

    pub fn as_whatsapp_auth(&self) -> Option<&WhatsAppAuthResponse> {
        match self {
            Self::WhatsappAuth { data } => Some(data),
            _ => None,
        }
    }

    pub fn into_whatsapp_auth(self) -> Option<WhatsAppAuthResponse> {
        match self {
            Self::WhatsappAuth { data } => Some(data),
            _ => None,
        }
    }
}
