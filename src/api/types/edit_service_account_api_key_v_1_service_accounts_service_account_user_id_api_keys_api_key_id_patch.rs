pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct EditServiceAccountApiKeyV1ServiceAccountsServiceAccountUserIdApiKeysApiKeyIdPatch {
    /// Whether to enable or disable the API key.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_enabled: Option<IsEnabled>,
    /// The name of the XI API key to use (used for identification purposes only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The permissions of the XI API.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions:
        Option<AccountApiKeyV1ServiceAccountsServiceAccountUserIdApiKeysApiKeyIdPatchPermissions>,
    /// The character limit of the XI API key. If provided this will limit the usage of this api key to n characters per month where n is the chosen value. Requests that incur charges will fail after reaching this monthly limit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub character_limit: Option<CharacterLimit>,
    /// List of IP addresses or CIDR ranges allowed to use this API key. Each entry may be a CIDR range (e.g. '10.0.0.0/24') or a bare IP address (normalized to /32 or /128). On create, omit or pass null to allow all IPs. On update, omit to leave the allowlist unchanged, or pass "clear" to remove it.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_ips: Option<AllowedIps>,
    /// Whether the holder of this key may disable it via the self-disable endpoint. On create, omit or pass null to use the workspace's default (enabled for non-Enterprise plans, disabled for Enterprise plans). On update, omit to leave it unchanged, or pass "clear" to reset it to the workspace default. Only honored for workspaces with self-disable access enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub third_party_disable_allowed: Option<ThirdPartyDisableAllowed>,
    /// Maximum concurrent text-to-speech/speech-to-speech requests for this API key. Only available for enterprise customers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tts_concurrency_limit: Option<TtsConcurrencyLimit>,
    /// Maximum concurrent dubbing requests for this API key. Only available for enterprise customers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dubbing_concurrency_limit: Option<DubbingConcurrencyLimit>,
    /// Maximum concurrent music generation requests for this API key. Only available for enterprise customers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub music_concurrency_limit: Option<MusicConcurrencyLimit>,
    /// Unix timestamp (in seconds) after which this API key stops working. When set, it must be at least 15 minutes and at most 90 days in the future. On create, omit or pass null for a key that never expires. On update, omit to leave it unchanged, or pass "clear" to remove the expiry (never expires). A workspace may disallow never-expiring keys, or allow them only on service account API keys; creating one or clearing an expiry then fails.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at_unix: Option<ExpiresAtUnix>,
}

impl EditServiceAccountApiKeyV1ServiceAccountsServiceAccountUserIdApiKeysApiKeyIdPatch {
    pub fn builder(
    ) -> EditServiceAccountApiKeyV1ServiceAccountsServiceAccountUserIdApiKeysApiKeyIdPatchBuilder
    {
        <EditServiceAccountApiKeyV1ServiceAccountsServiceAccountUserIdApiKeysApiKeyIdPatchBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EditServiceAccountApiKeyV1ServiceAccountsServiceAccountUserIdApiKeysApiKeyIdPatchBuilder
{
    is_enabled: Option<IsEnabled>,
    name: Option<String>,
    permissions:
        Option<AccountApiKeyV1ServiceAccountsServiceAccountUserIdApiKeysApiKeyIdPatchPermissions>,
    character_limit: Option<CharacterLimit>,
    allowed_ips: Option<AllowedIps>,
    third_party_disable_allowed: Option<ThirdPartyDisableAllowed>,
    tts_concurrency_limit: Option<TtsConcurrencyLimit>,
    dubbing_concurrency_limit: Option<DubbingConcurrencyLimit>,
    music_concurrency_limit: Option<MusicConcurrencyLimit>,
    expires_at_unix: Option<ExpiresAtUnix>,
}

impl EditServiceAccountApiKeyV1ServiceAccountsServiceAccountUserIdApiKeysApiKeyIdPatchBuilder {
    pub fn is_enabled(mut self, value: IsEnabled) -> Self {
        self.is_enabled = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn permissions(
        mut self,
        value: AccountApiKeyV1ServiceAccountsServiceAccountUserIdApiKeysApiKeyIdPatchPermissions,
    ) -> Self {
        self.permissions = Some(value);
        self
    }

    pub fn character_limit(mut self, value: CharacterLimit) -> Self {
        self.character_limit = Some(value);
        self
    }

    pub fn allowed_ips(mut self, value: AllowedIps) -> Self {
        self.allowed_ips = Some(value);
        self
    }

    pub fn third_party_disable_allowed(mut self, value: ThirdPartyDisableAllowed) -> Self {
        self.third_party_disable_allowed = Some(value);
        self
    }

    pub fn tts_concurrency_limit(mut self, value: TtsConcurrencyLimit) -> Self {
        self.tts_concurrency_limit = Some(value);
        self
    }

    pub fn dubbing_concurrency_limit(mut self, value: DubbingConcurrencyLimit) -> Self {
        self.dubbing_concurrency_limit = Some(value);
        self
    }

    pub fn music_concurrency_limit(mut self, value: MusicConcurrencyLimit) -> Self {
        self.music_concurrency_limit = Some(value);
        self
    }

    pub fn expires_at_unix(mut self, value: ExpiresAtUnix) -> Self {
        self.expires_at_unix = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`EditServiceAccountApiKeyV1ServiceAccountsServiceAccountUserIdApiKeysApiKeyIdPatch`].
    pub fn build(
        self,
    ) -> Result<
        EditServiceAccountApiKeyV1ServiceAccountsServiceAccountUserIdApiKeysApiKeyIdPatch,
        BuildError,
    > {
        Ok(
            EditServiceAccountApiKeyV1ServiceAccountsServiceAccountUserIdApiKeysApiKeyIdPatch {
                is_enabled: self.is_enabled,
                name: self.name,
                permissions: self.permissions,
                character_limit: self.character_limit,
                allowed_ips: self.allowed_ips,
                third_party_disable_allowed: self.third_party_disable_allowed,
                tts_concurrency_limit: self.tts_concurrency_limit,
                dubbing_concurrency_limit: self.dubbing_concurrency_limit,
                music_concurrency_limit: self.music_concurrency_limit,
                expires_at_unix: self.expires_at_unix,
            },
        )
    }
}
