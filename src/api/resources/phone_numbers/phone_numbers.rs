#[allow(unused_imports)]
use crate::api::types::*;
use crate::config::ClientConfig;
use crate::core::config_manager::ConfigManager;
use crate::core::HttpClient;
use crate::core::RequestOptions;
use std::sync::Arc;

pub struct PhoneNumbersClient {
    pub http_client: Arc<HttpClient>,
    config: Arc<ConfigManager>,
}

impl PhoneNumbersClient {
    /// Builds a standalone client from a configuration of its own.
    ///
    /// A client reached through the root client shares that client's
    /// configuration, so a later `set_access_token` reaches it; one built here
    /// owns its configuration outright and is unaffected by the root client.
    pub fn new(config: ClientConfig) -> Result<Self, crate::error::ApiError> {
        let http_client = Arc::new(HttpClient::new(config.clone())?);
        let config_manager = Arc::new(ConfigManager::new(config));
        Ok(Self::with_shared(http_client, config_manager))
    }

    pub(crate) fn with_shared(http_client: Arc<HttpClient>, config: Arc<ConfigManager>) -> Self {
        Self {
            http_client,
            config,
        }
    }

    /// List Phone Numbers
    ///
    /// Retrieve all Phone Numbers
    ///
    /// # Arguments
    ///
    /// * `request` - Query parameters for this request
    /// * `options` - Per-call overrides
    ///
    /// # Returns
    ///
    /// Successful Response
    pub async fn list(
        &self,
        request: &PhoneNumbersListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<Vec<PhoneNumbersListOkResponse>, crate::error::ApiError> {
        let provider = request.provider.clone();
        let agent_id = request.agent_id.clone();
        let branch_id = request.branch_id.clone();
        let url = "/v1/convai/phone-numbers";

        let mut request = self.http_client.get(url);
        if let Some(val) = &provider {
            request = request.query(&[("provider", &val.to_string())]);
        }
        if let Some(val) = &agent_id {
            request = request.query(&[("agent_id", &val.to_string())]);
        }
        if let Some(val) = &branch_id {
            request = request.query(&[("branch_id", &val.to_string())]);
        }
        if let Some(opts) = &options {
            for (key, value) in &opts.additional_headers {
                request = request.header(key.as_str(), value.as_str());
            }
            for (key, value) in &opts.additional_query_params {
                request = request.query(&[(key.as_str(), value.as_str())]);
            }
            if let Some(timeout_seconds) = opts.timeout_seconds {
                request = request.timeout(std::time::Duration::from_secs(timeout_seconds));
            }
        }
        let config = self.config.get_phone_numbers_config();
        let config = match &options {
            Some(opts) if opts.token.is_some() || opts.api_key.is_some() => {
                let mut overridden = config.clone();
                if let Some(api_key) = &opts.api_key {
                    overridden.api_key = Some(api_key.clone());
                }
                overridden
            }
            _ => config,
        };
        for (key, value) in &config.custom_headers {
            request = request.header(key.as_str(), value.as_str());
        }
        request = crate::core::auth::apply_auth(&config, request, true);

        let response = self
            .http_client
            .send_with_retry(request, options.as_ref())
            .await?;

        if !response.status().is_success() {
            let status = response.status().as_u16();
            let body = response.text().await.unwrap_or_default();
            return Err(crate::error::ApiError::from_response(
                status,
                Some(body.as_str()),
            ));
        }

        let status = response.status().as_u16();
        let bytes = response.bytes().await?;
        let result: Vec<PhoneNumbersListOkResponse> = if bytes.is_empty() {
            Default::default()
        } else {
            serde_json::from_slice(&bytes).map_err(|err| crate::error::ApiError::DecodeError {
                status,
                message: format!("{err}"),
            })?
        };
        Ok(result)
    }

    /// Import Phone Number
    ///
    /// Import Phone Number from provider configuration (Twilio, Exotel, or SIP trunk)
    ///
    /// # Arguments
    ///
    /// * `body`
    /// * `options` - Per-call overrides
    ///
    /// # Returns
    ///
    /// Successful Response
    pub async fn create(
        &self,
        body: &PhoneNumbersCreateRequest,
        options: Option<RequestOptions>,
    ) -> Result<CreatePhoneNumberResponseModel, crate::error::ApiError> {
        let url = "/v1/convai/phone-numbers";

        let mut request = self.http_client.post(url);
        if let Some(opts) = &options {
            for (key, value) in &opts.additional_headers {
                request = request.header(key.as_str(), value.as_str());
            }
            for (key, value) in &opts.additional_query_params {
                request = request.query(&[(key.as_str(), value.as_str())]);
            }
            if let Some(timeout_seconds) = opts.timeout_seconds {
                request = request.timeout(std::time::Duration::from_secs(timeout_seconds));
            }
        }
        request = match options
            .as_ref()
            .and_then(|opts| opts.merge_into_body(&body))
        {
            Some(merged) => request.json(&merged),
            None => request.json(&body),
        };
        let config = self.config.get_phone_numbers_config();
        let config = match &options {
            Some(opts) if opts.token.is_some() || opts.api_key.is_some() => {
                let mut overridden = config.clone();
                if let Some(api_key) = &opts.api_key {
                    overridden.api_key = Some(api_key.clone());
                }
                overridden
            }
            _ => config,
        };
        for (key, value) in &config.custom_headers {
            request = request.header(key.as_str(), value.as_str());
        }
        request = crate::core::auth::apply_auth(&config, request, true);

        let response = self
            .http_client
            .send_with_retry(request, options.as_ref())
            .await?;

        if !response.status().is_success() {
            let status = response.status().as_u16();
            let body = response.text().await.unwrap_or_default();
            return Err(crate::error::ApiError::from_response(
                status,
                Some(body.as_str()),
            ));
        }

        let status = response.status().as_u16();
        let result: CreatePhoneNumberResponseModel =
            response
                .json()
                .await
                .map_err(|err| crate::error::ApiError::DecodeError {
                    status,
                    message: format!("{err}"),
                })?;
        Ok(result)
    }

    /// Get Phone Number
    ///
    /// Retrieve Phone Number details by ID
    ///
    /// # Arguments
    ///
    /// * `phone_number_id` - The phone number ID. This is returned when a phone number is imported.
    /// * `options` - Per-call overrides
    ///
    /// # Returns
    ///
    /// Successful Response
    pub async fn get(
        &self,
        phone_number_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<ResponseGetPhoneNumberV1ConvaiPhoneNumbersPhoneNumberIdGet, crate::error::ApiError>
    {
        let url = format!("/v1/convai/phone-numbers/{}", phone_number_id);

        let mut request = self.http_client.get(url);
        if let Some(opts) = &options {
            for (key, value) in &opts.additional_headers {
                request = request.header(key.as_str(), value.as_str());
            }
            for (key, value) in &opts.additional_query_params {
                request = request.query(&[(key.as_str(), value.as_str())]);
            }
            if let Some(timeout_seconds) = opts.timeout_seconds {
                request = request.timeout(std::time::Duration::from_secs(timeout_seconds));
            }
        }
        let config = self.config.get_phone_numbers_config();
        let config = match &options {
            Some(opts) if opts.token.is_some() || opts.api_key.is_some() => {
                let mut overridden = config.clone();
                if let Some(api_key) = &opts.api_key {
                    overridden.api_key = Some(api_key.clone());
                }
                overridden
            }
            _ => config,
        };
        for (key, value) in &config.custom_headers {
            request = request.header(key.as_str(), value.as_str());
        }
        request = crate::core::auth::apply_auth(&config, request, true);

        let response = self
            .http_client
            .send_with_retry(request, options.as_ref())
            .await?;

        if !response.status().is_success() {
            let status = response.status().as_u16();
            let body = response.text().await.unwrap_or_default();
            return Err(crate::error::ApiError::from_response(
                status,
                Some(body.as_str()),
            ));
        }

        let status = response.status().as_u16();
        let result: ResponseGetPhoneNumberV1ConvaiPhoneNumbersPhoneNumberIdGet = response
            .json()
            .await
            .map_err(|err| crate::error::ApiError::DecodeError {
                status,
                message: format!("{err}"),
            })?;
        Ok(result)
    }

    /// Update Phone Number
    ///
    /// Update assigned agent of a phone number
    ///
    /// # Arguments
    ///
    /// * `phone_number_id` - The phone number ID. This is returned when a phone number is imported.
    /// * `body`
    /// * `options` - Per-call overrides
    ///
    /// # Returns
    ///
    /// Successful Response
    pub async fn update(
        &self,
        phone_number_id: &str,
        body: &UpdatePhoneNumberRequest,
        options: Option<RequestOptions>,
    ) -> Result<
        ResponseUpdatePhoneNumberV1ConvaiPhoneNumbersPhoneNumberIdPatch,
        crate::error::ApiError,
    > {
        let url = format!("/v1/convai/phone-numbers/{}", phone_number_id);

        let mut request = self.http_client.patch(url);
        if let Some(opts) = &options {
            for (key, value) in &opts.additional_headers {
                request = request.header(key.as_str(), value.as_str());
            }
            for (key, value) in &opts.additional_query_params {
                request = request.query(&[(key.as_str(), value.as_str())]);
            }
            if let Some(timeout_seconds) = opts.timeout_seconds {
                request = request.timeout(std::time::Duration::from_secs(timeout_seconds));
            }
        }
        request = match options
            .as_ref()
            .and_then(|opts| opts.merge_into_body(&body))
        {
            Some(merged) => request.json(&merged),
            None => request.json(&body),
        };
        let config = self.config.get_phone_numbers_config();
        let config = match &options {
            Some(opts) if opts.token.is_some() || opts.api_key.is_some() => {
                let mut overridden = config.clone();
                if let Some(api_key) = &opts.api_key {
                    overridden.api_key = Some(api_key.clone());
                }
                overridden
            }
            _ => config,
        };
        for (key, value) in &config.custom_headers {
            request = request.header(key.as_str(), value.as_str());
        }
        request = crate::core::auth::apply_auth(&config, request, true);

        let response = self
            .http_client
            .send_with_retry(request, options.as_ref())
            .await?;

        if !response.status().is_success() {
            let status = response.status().as_u16();
            let body = response.text().await.unwrap_or_default();
            return Err(crate::error::ApiError::from_response(
                status,
                Some(body.as_str()),
            ));
        }

        let status = response.status().as_u16();
        let result: ResponseUpdatePhoneNumberV1ConvaiPhoneNumbersPhoneNumberIdPatch = response
            .json()
            .await
            .map_err(|err| crate::error::ApiError::DecodeError {
                status,
                message: format!("{err}"),
            })?;
        Ok(result)
    }

    /// Delete Phone Number
    ///
    /// Delete Phone Number by ID
    ///
    /// # Arguments
    ///
    /// * `phone_number_id` - The phone number ID. This is returned when a phone number is imported.
    /// * `options` - Per-call overrides
    ///
    /// # Returns
    ///
    /// Successful Response
    pub async fn delete(
        &self,
        phone_number_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<serde_json::Value, crate::error::ApiError> {
        let url = format!("/v1/convai/phone-numbers/{}", phone_number_id);

        let mut request = self.http_client.delete(url);
        if let Some(opts) = &options {
            for (key, value) in &opts.additional_headers {
                request = request.header(key.as_str(), value.as_str());
            }
            for (key, value) in &opts.additional_query_params {
                request = request.query(&[(key.as_str(), value.as_str())]);
            }
            if let Some(timeout_seconds) = opts.timeout_seconds {
                request = request.timeout(std::time::Duration::from_secs(timeout_seconds));
            }
        }
        let config = self.config.get_phone_numbers_config();
        let config = match &options {
            Some(opts) if opts.token.is_some() || opts.api_key.is_some() => {
                let mut overridden = config.clone();
                if let Some(api_key) = &opts.api_key {
                    overridden.api_key = Some(api_key.clone());
                }
                overridden
            }
            _ => config,
        };
        for (key, value) in &config.custom_headers {
            request = request.header(key.as_str(), value.as_str());
        }
        request = crate::core::auth::apply_auth(&config, request, true);

        let response = self
            .http_client
            .send_with_retry(request, options.as_ref())
            .await?;

        if !response.status().is_success() {
            let status = response.status().as_u16();
            let body = response.text().await.unwrap_or_default();
            return Err(crate::error::ApiError::from_response(
                status,
                Some(body.as_str()),
            ));
        }

        let status = response.status().as_u16();
        let bytes = response.bytes().await?;
        let result: serde_json::Value = if bytes.is_empty() {
            Default::default()
        } else {
            serde_json::from_slice(&bytes).map_err(|err| crate::error::ApiError::DecodeError {
                status,
                message: format!("{err}"),
            })?
        };
        Ok(result)
    }

    /// List Phone Numbers Page
    ///
    /// Retrieve a page of Phone Numbers
    ///
    /// # Arguments
    ///
    /// * `request` - Query parameters for this request
    /// * `options` - Per-call overrides
    ///
    /// # Returns
    ///
    /// Successful Response
    pub async fn list_v_2(
        &self,
        request: &ListV2QueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<GetPhoneNumbersPageResponseModel, crate::error::ApiError> {
        let page_size = request.page_size.clone();
        let search = request.search.clone();
        let label = request.label.clone();
        let phone_number = request.phone_number.clone();
        let provider = request.provider.clone();
        let supports_outbound = request.supports_outbound.clone();
        let agent_id = request.agent_id.clone();
        let branch_id = request.branch_id.clone();
        let sort_by = request.sort_by.clone();
        let sort_direction = request.sort_direction.clone();
        let cursor = request.cursor.clone();
        let url = "/v1/convai/v2/phone-numbers";

        let mut request = self.http_client.get(url);
        if let Some(val) = &page_size {
            request = request.query(&[("page_size", &val.to_string())]);
        }
        if let Some(val) = &search {
            request = request.query(&[("search", &val.to_string())]);
        }
        if let Some(val) = &label {
            request = request.query(&[("label", &val.to_string())]);
        }
        if let Some(val) = &phone_number {
            request = request.query(&[("phone_number", &val.to_string())]);
        }
        if let Some(val) = &provider {
            request = request.query(&[("provider", &val.to_string())]);
        }
        if let Some(val) = &supports_outbound {
            request = request.query(&[("supports_outbound", &val.to_string())]);
        }
        if let Some(val) = &agent_id {
            request = request.query(&[("agent_id", &val.to_string())]);
        }
        if let Some(val) = &branch_id {
            request = request.query(&[("branch_id", &val.to_string())]);
        }
        if let Some(val) = &sort_by {
            request = request.query(&[("sort_by", &val.to_string())]);
        }
        if let Some(val) = &sort_direction {
            request = request.query(&[("sort_direction", &val.to_string())]);
        }
        if let Some(val) = &cursor {
            request = request.query(&[("cursor", &val.to_string())]);
        }
        if let Some(opts) = &options {
            for (key, value) in &opts.additional_headers {
                request = request.header(key.as_str(), value.as_str());
            }
            for (key, value) in &opts.additional_query_params {
                request = request.query(&[(key.as_str(), value.as_str())]);
            }
            if let Some(timeout_seconds) = opts.timeout_seconds {
                request = request.timeout(std::time::Duration::from_secs(timeout_seconds));
            }
        }
        let config = self.config.get_phone_numbers_config();
        let config = match &options {
            Some(opts) if opts.token.is_some() || opts.api_key.is_some() => {
                let mut overridden = config.clone();
                if let Some(api_key) = &opts.api_key {
                    overridden.api_key = Some(api_key.clone());
                }
                overridden
            }
            _ => config,
        };
        for (key, value) in &config.custom_headers {
            request = request.header(key.as_str(), value.as_str());
        }
        request = crate::core::auth::apply_auth(&config, request, true);

        let response = self
            .http_client
            .send_with_retry(request, options.as_ref())
            .await?;

        if !response.status().is_success() {
            let status = response.status().as_u16();
            let body = response.text().await.unwrap_or_default();
            return Err(crate::error::ApiError::from_response(
                status,
                Some(body.as_str()),
            ));
        }

        let status = response.status().as_u16();
        let result: GetPhoneNumbersPageResponseModel =
            response
                .json()
                .await
                .map_err(|err| crate::error::ApiError::DecodeError {
                    status,
                    message: format!("{err}"),
                })?;
        Ok(result)
    }

    /// Get Sip Messages For A Phone Number
    ///
    /// Get SIP messages for a phone number
    ///
    /// # Arguments
    ///
    /// * `phone_number_id` - The phone number ID. This is returned when a phone number is imported.
    /// * `request` - Query parameters for this request
    /// * `options` - Per-call overrides
    ///
    /// # Returns
    ///
    /// Successful Response
    pub async fn get_sip_messages(
        &self,
        phone_number_id: &str,
        request: &PhoneNumbersGetSipMessagesQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<GetSipLogMessagesResponse, crate::error::ApiError> {
        let page_size = request.page_size.clone();
        let cursor = request.cursor.clone();
        let url = format!("/v1/convai/phone-numbers/{}/sip-messages", phone_number_id);

        let mut request = self.http_client.get(url);
        if let Some(val) = &page_size {
            request = request.query(&[("page_size", &val.to_string())]);
        }
        if let Some(val) = &cursor {
            request = request.query(&[("cursor", &val.to_string())]);
        }
        if let Some(opts) = &options {
            for (key, value) in &opts.additional_headers {
                request = request.header(key.as_str(), value.as_str());
            }
            for (key, value) in &opts.additional_query_params {
                request = request.query(&[(key.as_str(), value.as_str())]);
            }
            if let Some(timeout_seconds) = opts.timeout_seconds {
                request = request.timeout(std::time::Duration::from_secs(timeout_seconds));
            }
        }
        let config = self.config.get_phone_numbers_config();
        let config = match &options {
            Some(opts) if opts.token.is_some() || opts.api_key.is_some() => {
                let mut overridden = config.clone();
                if let Some(api_key) = &opts.api_key {
                    overridden.api_key = Some(api_key.clone());
                }
                overridden
            }
            _ => config,
        };
        for (key, value) in &config.custom_headers {
            request = request.header(key.as_str(), value.as_str());
        }
        request = crate::core::auth::apply_auth(&config, request, true);

        let response = self
            .http_client
            .send_with_retry(request, options.as_ref())
            .await?;

        if !response.status().is_success() {
            let status = response.status().as_u16();
            let body = response.text().await.unwrap_or_default();
            return Err(crate::error::ApiError::from_response(
                status,
                Some(body.as_str()),
            ));
        }

        let status = response.status().as_u16();
        let result: GetSipLogMessagesResponse =
            response
                .json()
                .await
                .map_err(|err| crate::error::ApiError::DecodeError {
                    status,
                    message: format!("{err}"),
                })?;
        Ok(result)
    }

    /// List Phone Numbers
    ///
    /// List the workspace's phone numbers and the receptionist each one rings. A number with no `customer_facing_agent_id` is held by the workspace but not attached to a receptionist yet.
    ///
    /// # Arguments
    ///
    /// * `xi_workspace_id` - The customer workspace a reseller's API key acts on.
    /// * `options` - Per-call overrides
    ///
    /// # Returns
    ///
    /// Successful Response
    pub async fn list_2(
        &self,
        xi_workspace_id: String,
        options: Option<RequestOptions>,
    ) -> Result<Vec<ReceptionPhoneAssignment>, crate::error::ApiError> {
        let url = "/v1/reception/phone-numbers/assignments";

        let mut request = self.http_client.get(url);
        request = request.header("xi-workspace-id", &xi_workspace_id.to_string());
        if let Some(opts) = &options {
            for (key, value) in &opts.additional_headers {
                request = request.header(key.as_str(), value.as_str());
            }
            for (key, value) in &opts.additional_query_params {
                request = request.query(&[(key.as_str(), value.as_str())]);
            }
            if let Some(timeout_seconds) = opts.timeout_seconds {
                request = request.timeout(std::time::Duration::from_secs(timeout_seconds));
            }
        }
        let config = self.config.get_phone_numbers_config();
        let config = match &options {
            Some(opts) if opts.token.is_some() || opts.api_key.is_some() => {
                let mut overridden = config.clone();
                if let Some(api_key) = &opts.api_key {
                    overridden.api_key = Some(api_key.clone());
                }
                overridden
            }
            _ => config,
        };
        for (key, value) in &config.custom_headers {
            request = request.header(key.as_str(), value.as_str());
        }
        request = crate::core::auth::apply_auth(&config, request, true);

        let response = self
            .http_client
            .send_with_retry(request, options.as_ref())
            .await?;

        if !response.status().is_success() {
            let status = response.status().as_u16();
            let body = response.text().await.unwrap_or_default();
            return Err(crate::error::ApiError::from_response(
                status,
                Some(body.as_str()),
            ));
        }

        let status = response.status().as_u16();
        let bytes = response.bytes().await?;
        let result: Vec<ReceptionPhoneAssignment> = if bytes.is_empty() {
            Default::default()
        } else {
            serde_json::from_slice(&bytes).map_err(|err| crate::error::ApiError::DecodeError {
                status,
                message: format!("{err}"),
            })?
        };
        Ok(result)
    }

    /// Assign Phone Number
    ///
    /// Give a receptionist a phone number. A number the workspace already holds but has not attached is used first. Otherwise, in the US and Canada, a number with one of the requested `area_codes` is taken from the pool or bought, falling back to any number in the business's country. In countries that require paperwork, the number is issued right away but takes public calls only once the workspace's own registration is approved (see `accepts_public_calls`). Counts toward the plan's phone number limit, and free plans can take only a few new numbers a day. Returns 404 when no number is available and 403 `telephony_managed_by_reseller` when the reseller supplies the workspace's numbers from its own Twilio account or SIP trunk and none is free.
    ///
    /// # Arguments
    ///
    /// * `body`
    /// * `xi_workspace_id` - The customer workspace a reseller's API key acts on.
    /// * `options` - Per-call overrides
    ///
    /// # Returns
    ///
    /// Successful Response
    pub async fn create_2(
        &self,
        body: &CreatePhoneAssignmentRequest,
        xi_workspace_id: String,
        options: Option<RequestOptions>,
    ) -> Result<ReceptionPhoneAssignment, crate::error::ApiError> {
        let url = "/v1/reception/phone-numbers/assignments";

        let mut request = self.http_client.post(url);
        request = request.header("xi-workspace-id", &xi_workspace_id.to_string());
        if let Some(opts) = &options {
            for (key, value) in &opts.additional_headers {
                request = request.header(key.as_str(), value.as_str());
            }
            for (key, value) in &opts.additional_query_params {
                request = request.query(&[(key.as_str(), value.as_str())]);
            }
            if let Some(timeout_seconds) = opts.timeout_seconds {
                request = request.timeout(std::time::Duration::from_secs(timeout_seconds));
            }
        }
        request = match options
            .as_ref()
            .and_then(|opts| opts.merge_into_body(&body))
        {
            Some(merged) => request.json(&merged),
            None => request.json(&body),
        };
        let config = self.config.get_phone_numbers_config();
        let config = match &options {
            Some(opts) if opts.token.is_some() || opts.api_key.is_some() => {
                let mut overridden = config.clone();
                if let Some(api_key) = &opts.api_key {
                    overridden.api_key = Some(api_key.clone());
                }
                overridden
            }
            _ => config,
        };
        for (key, value) in &config.custom_headers {
            request = request.header(key.as_str(), value.as_str());
        }
        request = crate::core::auth::apply_auth(&config, request, true);

        let response = self
            .http_client
            .send_with_retry(request, options.as_ref())
            .await?;

        if !response.status().is_success() {
            let status = response.status().as_u16();
            let body = response.text().await.unwrap_or_default();
            return Err(crate::error::ApiError::from_response(
                status,
                Some(body.as_str()),
            ));
        }

        let status = response.status().as_u16();
        let result: ReceptionPhoneAssignment =
            response
                .json()
                .await
                .map_err(|err| crate::error::ApiError::DecodeError {
                    status,
                    message: format!("{err}"),
                })?;
        Ok(result)
    }

    /// Move Phone Number
    ///
    /// Move a phone number to a different receptionist in the same workspace, which answers its calls from then on. Returns 409 `phone_number_not_attached` for a number no receptionist answers yet (attach it with `POST /v1/reception/phone-numbers/assignments`).
    ///
    /// # Arguments
    ///
    /// * `phone_assignment_id` - ID of the phone number assignment. Returned when you assign a number; list them with `GET /v1/reception/phone-numbers/assignments`.
    /// * `body`
    /// * `xi_workspace_id` - The customer workspace a reseller's API key acts on.
    /// * `options` - Per-call overrides
    ///
    /// # Returns
    ///
    /// Successful Response
    pub async fn update_2(
        &self,
        phone_assignment_id: &str,
        body: &ReassignPhoneRequest,
        xi_workspace_id: String,
        options: Option<RequestOptions>,
    ) -> Result<ReceptionPhoneAssignment, crate::error::ApiError> {
        let url = format!(
            "/v1/reception/phone-numbers/assignments/{}",
            phone_assignment_id
        );

        let mut request = self.http_client.patch(url);
        request = request.header("xi-workspace-id", &xi_workspace_id.to_string());
        if let Some(opts) = &options {
            for (key, value) in &opts.additional_headers {
                request = request.header(key.as_str(), value.as_str());
            }
            for (key, value) in &opts.additional_query_params {
                request = request.query(&[(key.as_str(), value.as_str())]);
            }
            if let Some(timeout_seconds) = opts.timeout_seconds {
                request = request.timeout(std::time::Duration::from_secs(timeout_seconds));
            }
        }
        request = match options
            .as_ref()
            .and_then(|opts| opts.merge_into_body(&body))
        {
            Some(merged) => request.json(&merged),
            None => request.json(&body),
        };
        let config = self.config.get_phone_numbers_config();
        let config = match &options {
            Some(opts) if opts.token.is_some() || opts.api_key.is_some() => {
                let mut overridden = config.clone();
                if let Some(api_key) = &opts.api_key {
                    overridden.api_key = Some(api_key.clone());
                }
                overridden
            }
            _ => config,
        };
        for (key, value) in &config.custom_headers {
            request = request.header(key.as_str(), value.as_str());
        }
        request = crate::core::auth::apply_auth(&config, request, true);

        let response = self
            .http_client
            .send_with_retry(request, options.as_ref())
            .await?;

        if !response.status().is_success() {
            let status = response.status().as_u16();
            let body = response.text().await.unwrap_or_default();
            return Err(crate::error::ApiError::from_response(
                status,
                Some(body.as_str()),
            ));
        }

        let status = response.status().as_u16();
        let result: ReceptionPhoneAssignment =
            response
                .json()
                .await
                .map_err(|err| crate::error::ApiError::DecodeError {
                    status,
                    message: format!("{err}"),
                })?;
        Ok(result)
    }

    /// Release Phone Number
    ///
    /// Remove a phone number from the workspace. A pool number goes back to the pool it came from. A number the business brought from its own Twilio account, or one issued in a country that requires paperwork, is released at Twilio and cannot be recovered. Returns 403 `telephony_managed_by_reseller` when the reseller supplies the workspace's numbers from its own Twilio account or SIP trunk.
    ///
    /// # Arguments
    ///
    /// * `phone_assignment_id` - ID of the phone number assignment. Returned when you assign a number; list them with `GET /v1/reception/phone-numbers/assignments`.
    /// * `xi_workspace_id` - The customer workspace a reseller's API key acts on.
    /// * `options` - Per-call overrides
    ///
    /// # Returns
    ///
    /// Successful Response
    pub async fn delete_2(
        &self,
        phone_assignment_id: &str,
        xi_workspace_id: String,
        options: Option<RequestOptions>,
    ) -> Result<(), crate::error::ApiError> {
        let url = format!(
            "/v1/reception/phone-numbers/assignments/{}",
            phone_assignment_id
        );

        let mut request = self.http_client.delete(url);
        request = request.header("xi-workspace-id", &xi_workspace_id.to_string());
        if let Some(opts) = &options {
            for (key, value) in &opts.additional_headers {
                request = request.header(key.as_str(), value.as_str());
            }
            for (key, value) in &opts.additional_query_params {
                request = request.query(&[(key.as_str(), value.as_str())]);
            }
            if let Some(timeout_seconds) = opts.timeout_seconds {
                request = request.timeout(std::time::Duration::from_secs(timeout_seconds));
            }
        }
        let config = self.config.get_phone_numbers_config();
        let config = match &options {
            Some(opts) if opts.token.is_some() || opts.api_key.is_some() => {
                let mut overridden = config.clone();
                if let Some(api_key) = &opts.api_key {
                    overridden.api_key = Some(api_key.clone());
                }
                overridden
            }
            _ => config,
        };
        for (key, value) in &config.custom_headers {
            request = request.header(key.as_str(), value.as_str());
        }
        request = crate::core::auth::apply_auth(&config, request, true);

        let response = self
            .http_client
            .send_with_retry(request, options.as_ref())
            .await?;

        if !response.status().is_success() {
            let status = response.status().as_u16();
            let body = response.text().await.unwrap_or_default();
            return Err(crate::error::ApiError::from_response(
                status,
                Some(body.as_str()),
            ));
        }

        let _ = response;
        Ok(())
    }
}
