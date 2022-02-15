use crate::{
    errors::{HealthchecksApiError, HealthchecksConfigError},
    model::{Channel, Check, Flip, NewCheck, Ping, UpdatedCheck},
    util::default_user_agent,
};
use std::result::Result;
use ureq::{delete, get, post, Error, Request};

const HEALTHCHECK_API_URL: &str = "https://healthchecks.io/api/v1/";

/// Typealias to prevent some repetitiveness in function definitions
pub type ApiResult<T> = Result<T, HealthchecksApiError>;

/// Client type for communication with the healthchecks.io management API.
#[derive(Clone)]
pub struct ManageClient {
    pub(crate) api_key: String,
    pub(crate) user_agent: String,
    pub(crate) api_url: String,
}

/// Create an instance of [`ManageClient`] from a given API key and an
/// optional custom user agent. Basic validation is performed but an
/// invalid API key will go through this method and only fail when
/// actually interacting with the API.
pub fn get_client(
    api_key: String,
    user_agent: Option<String>,
) -> Result<ManageClient, HealthchecksConfigError> {
    get_client_with_url(api_key, user_agent, HEALTHCHECK_API_URL.to_owned())
}

/// Create an instance of [`ManageClient`] with a custom endpoint for the API.
/// This is identical to [`get_client`](crate::manage::get_client) which uses it internally,
/// but doesn't default to the hosted version at [healthchecks.io](https://healthchecks.io).
pub fn get_client_with_url(
    api_key: String,
    user_agent: Option<String>,
    api_url: String,
) -> Result<ManageClient, HealthchecksConfigError> {
    if api_key.is_empty() {
        Err(HealthchecksConfigError::EmptyApiKey)
    } else if matches!(user_agent, Some(ref ua) if ua.is_empty()) {
        Err(HealthchecksConfigError::EmptyUserAgent)
    } else if api_url.is_empty() {
        Err(HealthchecksConfigError::EmptyApiUrl)
    } else {
        let user_agent = user_agent.unwrap_or_else(|| default_user_agent().to_owned());

        Ok(ManageClient {
            api_key,
            user_agent,
            api_url,
        })
    }
}

/// When creating a new check, it's possible that the check already existed.
/// This enum conveys that information.
///
/// See [`ManageClient::upsert_check`]
pub enum UpsertResult {
    Created,
    Updated,
}

impl ManageClient {
    fn ureq_get(&self, path: String) -> Request {
        get(&path)
            .set("X-Api-Key", &self.api_key)
            .set("User-Agent", &self.user_agent)
    }

    fn ureq_post(&self, path: String) -> Request {
        post(&path)
            .set("X-Api-Key", &self.api_key)
            .set("User-Agent", &self.user_agent)
    }

    fn ureq_delete(&self, path: String) -> Request {
        delete(&path)
            .set("X-Api-Key", &self.api_key)
            .set("User-Agent", &self.user_agent)
    }

    /// Get a list of [`Check`]s.
    pub fn get_checks(&self) -> ApiResult<Vec<Check>> {
        #[derive(serde::Deserialize)]
        struct ChecksResult {
            pub checks: Vec<Check>,
        }
        let r = self.ureq_get(format!("{}/{}", self.api_url, "checks"));
        match r.call() {
            Ok(response) => Ok(response.into_json::<ChecksResult>()?.checks),
            Err(Error::Status(401, _)) => Err(HealthchecksApiError::InvalidApiKey),
            Err(Error::Status(_, response)) => Err(HealthchecksApiError::UnexpectedError(
                response.into_string()?,
            )),
            Err(Error::Transport(err)) => Err(HealthchecksApiError::TransportError(Box::new(err))),
        }
    }

    /// Get a [`Check`] with the given UUID or unique key.
    pub fn get_check(&self, check_id: &str) -> ApiResult<Check> {
        let r = self.ureq_get(format!("{}/{}/{}", self.api_url, "checks", check_id));
        match r.call() {
            Ok(response) => Ok(response.into_json::<Check>()?),
            Err(Error::Status(401, _)) => Err(HealthchecksApiError::InvalidApiKey),
            Err(Error::Status(403, _)) => Err(HealthchecksApiError::AccessDenied),
            Err(Error::Status(404, _)) => {
                Err(HealthchecksApiError::NoCheckFound(check_id.to_string()))
            }
            Err(Error::Status(_, response)) => Err(HealthchecksApiError::UnexpectedError(
                response.into_string()?,
            )),
            Err(Error::Transport(err)) => Err(HealthchecksApiError::TransportError(Box::new(err))),
        }
    }

    /// Returns a list of [`Channel`]s belonging to the project.
    pub fn get_channels(&self) -> ApiResult<Vec<Channel>> {
        #[derive(serde::Deserialize)]
        struct ChannelsResult {
            pub channels: Vec<Channel>,
        }
        let r = self.ureq_get(format!("{}/{}", self.api_url, "channels"));
        match r.call() {
            Ok(response) => Ok(response.into_json::<ChannelsResult>()?.channels),
            Err(Error::Status(401, _)) => Err(HealthchecksApiError::PossibleReadOnlyKey),
            Err(Error::Status(_, response)) => Err(HealthchecksApiError::UnexpectedError(
                response.into_string()?,
            )),
            Err(Error::Transport(err)) => Err(HealthchecksApiError::TransportError(Box::new(err))),
        }
    }

    /// Pauses the [`Check`] with the given UUID or unique key.
    pub fn pause(&self, check_id: &str) -> ApiResult<Check> {
        let r = self.ureq_post(format!("{}/checks/{}/pause", self.api_url, check_id));
        match r.call() {
            Ok(response) => Ok(response.into_json::<Check>()?),
            Err(Error::Status(401, _)) => Err(HealthchecksApiError::PossibleReadOnlyKey),
            Err(Error::Status(403, _)) => Err(HealthchecksApiError::AccessDenied),
            Err(Error::Status(404, _)) => {
                Err(HealthchecksApiError::NoCheckFound(check_id.to_string()))
            }
            Err(Error::Status(_, response)) => Err(HealthchecksApiError::UnexpectedError(
                response.into_string()?,
            )),
            Err(Error::Transport(err)) => Err(HealthchecksApiError::TransportError(Box::new(err))),
        }
    }

    /// Get a list of check's logged pings with the given UUID or unique key.
    pub fn list_logged_pings(&self, check_id: &str) -> ApiResult<Vec<Ping>> {
        #[derive(serde::Deserialize)]
        struct PingsResult {
            pub pings: Vec<Ping>,
        }
        let r = self.ureq_post(format!("{}/checks/{}/pings", self.api_url, check_id));
        match r.send_string("") {
            Ok(response) => Ok(response.into_json::<PingsResult>()?.pings),
            Err(Error::Status(401, _)) => Err(HealthchecksApiError::InvalidApiKey),
            Err(Error::Status(403, _)) => Err(HealthchecksApiError::AccessDenied),
            Err(Error::Status(404, _)) => {
                Err(HealthchecksApiError::NoCheckFound(check_id.to_string()))
            }
            Err(Error::Status(_, response)) => Err(HealthchecksApiError::UnexpectedError(
                response.into_string()?,
            )),
            Err(Error::Transport(err)) => Err(HealthchecksApiError::TransportError(Box::new(err))),
        }
    }

    /// Get a list of check's status changes with the given UUID or unique key.
    pub fn list_status_changes(&self, check_id: &str) -> ApiResult<Vec<Flip>> {
        let r = self.ureq_post(format!("{}/checks/{}/flips", self.api_url, check_id));
        match r.call() {
            Ok(response) => Ok(response.into_json::<Vec<Flip>>()?),
            Err(Error::Status(401, _)) => Err(HealthchecksApiError::InvalidApiKey),
            Err(Error::Status(403, _)) => Err(HealthchecksApiError::AccessDenied),
            Err(Error::Status(404, _)) => {
                Err(HealthchecksApiError::NoCheckFound(check_id.to_string()))
            }
            Err(Error::Status(_, response)) => Err(HealthchecksApiError::UnexpectedError(
                response.into_string()?,
            )),
            Err(Error::Transport(err)) => Err(HealthchecksApiError::TransportError(Box::new(err))),
        }
    }

    /// Deletes the [`Check`] with the given UUID or unique key.
    pub fn delete(&self, check_id: &str) -> ApiResult<Check> {
        let r = self.ureq_delete(format!("{}/{}/{}", self.api_url, "checks", check_id));
        match r.call() {
            Ok(response) => Ok(response.into_json::<Check>()?),
            Err(Error::Status(401, _)) => Err(HealthchecksApiError::InvalidApiKey),
            Err(Error::Status(403, _)) => Err(HealthchecksApiError::AccessDenied),
            Err(Error::Status(404, _)) => {
                Err(HealthchecksApiError::NoCheckFound(check_id.to_string()))
            }
            Err(Error::Status(_, response)) => Err(HealthchecksApiError::UnexpectedError(
                response.into_string()?,
            )),
            Err(Error::Transport(err)) => Err(HealthchecksApiError::TransportError(Box::new(err))),
        }
    }

    /// Creates a new check with the given [`NewCheck`] configuration.
    pub fn create_check(&self, check: NewCheck) -> ApiResult<Check> {
        self.upsert_check(check).and_then(|(result, check)| {
            if let UpsertResult::Created = result {
                Ok(check)
            } else {
                Err(HealthchecksApiError::ExistingCheckMatched)
            }
        })
    }

    /// Creates a new check with the given [`NewCheck`] configuration.
    ///
    /// The [`unique`] field can be used to update existing checks, provided the checks can be found.
    /// Otherwise, it will be created.
    ///
    /// [`unique`]: NewCheck::unique
    pub fn upsert_check(&self, check: NewCheck) -> ApiResult<(UpsertResult, Check)> {
        let check_json = serde_json::to_value(check)?;
        let r = self.ureq_post(format!("{}/{}/", self.api_url, "checks"));
        match r
            .set("Content-Type", "application/json")
            .send_json(check_json)
        {
            Ok(response) => match response.status() {
                201 => Ok((UpsertResult::Created, response.into_json::<Check>()?)),
                200 => Ok((UpsertResult::Updated, response.into_json::<Check>()?)),
                _ => Err(HealthchecksApiError::UnexpectedError(format!(
                    "Invalid result code: {}",
                    response.status()
                ))),
            },
            Err(Error::Status(400, _)) => Err(HealthchecksApiError::NotWellFormed),
            Err(Error::Status(401, _)) => Err(HealthchecksApiError::InvalidApiKey),
            Err(Error::Status(403, _)) => Err(HealthchecksApiError::CheckLimitReached),
            Err(Error::Status(_, response)) => Err(HealthchecksApiError::UnexpectedError(
                response.into_string()?,
            )),
            Err(Error::Transport(err)) => Err(HealthchecksApiError::TransportError(Box::new(err))),
        }
    }

    /// Update the check with the given `check_id` with the data from `check`.
    pub fn update_check(&self, check: UpdatedCheck, check_id: &str) -> ApiResult<Check> {
        let check_json = serde_json::to_value(check)?;
        let r = self.ureq_post(format!("{}/{}/{}", self.api_url, "checks", check_id));
        match r
            .set("Content-Type", "application/json")
            .send_json(check_json)
        {
            Ok(response) => Ok(response.into_json::<Check>()?),
            Err(Error::Status(400, _)) => Err(HealthchecksApiError::NotWellFormed),
            Err(Error::Status(401, _)) => Err(HealthchecksApiError::InvalidApiKey),
            Err(Error::Status(403, _)) => Err(HealthchecksApiError::AccessDenied),
            Err(Error::Status(404, _)) => {
                Err(HealthchecksApiError::NoCheckFound(check_id.to_string()))
            }
            Err(Error::Status(_, response)) => Err(HealthchecksApiError::UnexpectedError(
                response.into_string()?,
            )),
            Err(Error::Transport(err)) => Err(HealthchecksApiError::TransportError(Box::new(err))),
        }
    }
}
