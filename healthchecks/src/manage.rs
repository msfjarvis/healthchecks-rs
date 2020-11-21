use crate::{
    errors::{HealthchecksApiError, HealthchecksConfigError},
    model::{Channel, Check, Flip, NewCheck, Ping, UpdatedCheck},
    util::default_user_agent,
};
use std::result::Result;
use ureq::{delete, get, post, Request};

const HEALTHCHECK_API_URL: &str = "https://healthchecks.io/api/v1/";

/// Typealias to prevent some repetitiveness in function definitions
type ApiResult<T> = Result<T, HealthchecksApiError>;

/// Struct that encapsulates the API key used to communicate with the healthchecks.io
/// management API. Instances of this struct expose methods to query the API.
pub struct ApiConfig {
    pub(crate) api_key: String,
    pub(crate) user_agent: String,
}

/// Create an instance of [`ApiConfig`] from a given API key. No validation
/// is performed.
pub fn get_config(
    api_key: String,
    user_agent: Option<String>,
) -> Result<ApiConfig, HealthchecksConfigError> {
    if api_key.is_empty() {
        Err(HealthchecksConfigError::EmptyApiKey)
    } else if let Some(ua) = user_agent {
        if ua.is_empty() {
            Err(HealthchecksConfigError::EmptyUserAgent)
        } else {
            Ok(ApiConfig {
                api_key,
                user_agent: ua,
            })
        }
    } else {
        Ok(ApiConfig {
            api_key,
            user_agent: default_user_agent().to_owned(),
        })
    }
}

impl ApiConfig {
    #[inline]
    fn set_headers<'a>(&self, req: &'a mut Request) -> &'a mut Request {
        req.set("X-Api-Key", &self.api_key)
            .set("User-Agent", &self.user_agent)
    }

    /// Get a list of [`Check`]s.
    pub fn get_checks(&self) -> ApiResult<Vec<Check>> {
        #[derive(serde::Deserialize)]
        struct ChecksResult {
            pub checks: Vec<Check>,
        }
        let mut r = &mut get(&format!("{}/{}", HEALTHCHECK_API_URL, "checks"));
        r = self.set_headers(r);
        let resp = r.call();
        match resp.status() {
            200 => Ok(resp.into_json_deserialize::<ChecksResult>()?.checks),
            401 => Err(HealthchecksApiError::InvalidAPIKey),
            _ => Err(HealthchecksApiError::UnexpectedError(resp.into_string()?)),
        }
    }

    /// Get a [`Check`] with the given UUID or unique key.
    pub fn get_check(&self, check_id: &str) -> ApiResult<Check> {
        let mut r = &mut get(&format!(
            "{}/{}/{}",
            HEALTHCHECK_API_URL, "checks", check_id
        ));
        r = self.set_headers(r);
        let resp = r.call();
        match resp.status() {
            200 => Ok(resp.into_json_deserialize::<Check>()?),
            401 => Err(HealthchecksApiError::InvalidAPIKey),
            403 => Err(HealthchecksApiError::AccessDenied),
            404 => Err(HealthchecksApiError::NoCheckFound(check_id.to_string())),
            _ => Err(HealthchecksApiError::UnexpectedError(resp.into_string()?)),
        }
    }

    /// Returns a list of [`Channel`]s belonging to the project.
    pub fn get_channels(&self) -> ApiResult<Vec<Channel>> {
        #[derive(serde::Deserialize)]
        struct ChannelsResult {
            pub channels: Vec<Channel>,
        }
        let mut r = &mut get(&format!("{}/{}", HEALTHCHECK_API_URL, "channels"));
        r = self.set_headers(r);
        let resp = r.call();
        match resp.status() {
            200 => Ok(resp.into_json_deserialize::<ChannelsResult>()?.channels),
            401 => Err(HealthchecksApiError::PossibleReadOnlyKey),
            _ => Err(HealthchecksApiError::UnexpectedError(resp.into_string()?)),
        }
    }

    /// Pauses the [`Check`] with the given UUID or unique key.
    pub fn pause(&self, check_id: &str) -> ApiResult<Check> {
        let mut r = &mut post(&format!(
            "{}/checks/{}/pause",
            HEALTHCHECK_API_URL, check_id
        ));
        r = self.set_headers(r);
        let resp = r.send_string("");
        match resp.status() {
            200 => Ok(resp.into_json_deserialize::<Check>()?),
            401 => Err(HealthchecksApiError::PossibleReadOnlyKey),
            403 => Err(HealthchecksApiError::AccessDenied),
            404 => Err(HealthchecksApiError::NoCheckFound(check_id.to_string())),
            _ => Err(HealthchecksApiError::UnexpectedError(resp.into_string()?)),
        }
    }

    /// Get a list of check's logged pings with the given UUID or unique key.
    pub fn list_logged_pings(&self, check_id: &str) -> ApiResult<Vec<Ping>> {
        #[derive(serde::Deserialize)]
        struct PingsResult {
            pub pings: Vec<Ping>,
        }
        let mut r = &mut post(&format!(
            "{}/checks/{}/pings",
            HEALTHCHECK_API_URL, check_id
        ));
        r = self.set_headers(r);
        let resp = r.send_string("");
        match resp.status() {
            200 => Ok(resp.into_json_deserialize::<PingsResult>()?.pings),
            401 => Err(HealthchecksApiError::InvalidAPIKey),
            403 => Err(HealthchecksApiError::AccessDenied),
            404 => Err(HealthchecksApiError::NoCheckFound(check_id.to_string())),
            _ => Err(HealthchecksApiError::UnexpectedError(resp.into_string()?)),
        }
    }

    /// Get a list of check's status changes with the given UUID or unique key.
    pub fn list_status_changes(&self, check_id: &str) -> ApiResult<Vec<Flip>> {
        let mut r = &mut post(&format!(
            "{}/checks/{}/flips",
            HEALTHCHECK_API_URL, check_id
        ));
        r = self.set_headers(r);
        let resp = r.call();
        match resp.status() {
            200 => Ok(resp.into_json_deserialize::<Vec<Flip>>()?),
            401 => Err(HealthchecksApiError::InvalidAPIKey),
            403 => Err(HealthchecksApiError::AccessDenied),
            404 => Err(HealthchecksApiError::NoCheckFound(check_id.to_string())),
            _ => Err(HealthchecksApiError::UnexpectedError(resp.into_string()?)),
        }
    }

    /// Deletes the [`Check`] with the given UUID or unique key.
    pub fn delete(&self, check_id: &str) -> ApiResult<Check> {
        let mut r = &mut delete(&format!(
            "{}/{}/{}",
            HEALTHCHECK_API_URL, "checks", check_id
        ));
        r = self.set_headers(r);
        let resp = r.call();
        match resp.status() {
            200 => Ok(resp.into_json_deserialize::<Check>()?),
            401 => Err(HealthchecksApiError::InvalidAPIKey),
            403 => Err(HealthchecksApiError::AccessDenied),
            404 => Err(HealthchecksApiError::NoCheckFound(check_id.to_string())),
            _ => Err(HealthchecksApiError::UnexpectedError(resp.into_string()?)),
        }
    }

    /// Creates a new check with the given [`NewCheck`] configuration.
    pub fn create_check(&self, check: NewCheck) -> ApiResult<Check> {
        let check_json =
            serde_json::to_value(check).expect("Failed to convert check into valid JSON");
        let mut r = &mut post(&format!("{}/{}/", HEALTHCHECK_API_URL, "checks"));
        r = self.set_headers(r);
        let resp = r
            .set("Content-Type", "application/json")
            .send_json(check_json);
        match resp.status() {
            201 => Ok(resp.into_json_deserialize::<Check>()?),
            200 => Err(HealthchecksApiError::ExistingCheckMatched),
            400 => Err(HealthchecksApiError::NotWellFormed),
            401 => Err(HealthchecksApiError::InvalidAPIKey),
            403 => Err(HealthchecksApiError::CheckLimitReached),
            _ => Err(HealthchecksApiError::UnexpectedError(resp.into_string()?)),
        }
    }

    /// Update the check with the given `check_id` with the data from `check`.
    pub fn update_check(&self, check: UpdatedCheck, check_id: &str) -> ApiResult<Check> {
        let check_json = serde_json::to_value(check)?;
        let mut r = &mut post(&format!(
            "{}/{}/{}",
            HEALTHCHECK_API_URL, "checks", check_id
        ));
        r = self.set_headers(r);
        let resp = r
            .set("Content-Type", "application/json")
            .send_json(check_json);
        match resp.status() {
            200 => Ok(resp.into_json_deserialize::<Check>()?),
            400 => Err(HealthchecksApiError::NotWellFormed),
            401 => Err(HealthchecksApiError::InvalidAPIKey),
            403 => Err(HealthchecksApiError::AccessDenied),
            404 => Err(HealthchecksApiError::NoCheckFound(check_id.to_string())),
            _ => Err(HealthchecksApiError::UnexpectedError(resp.into_string()?)),
        }
    }
}
