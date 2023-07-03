use crate::{
    errors::{HealthchecksApiError, HealthchecksConfigError},
    model::{Channel, Check, Flip, NewCheck, Ping, UpdatedCheck},
    DEFAULT_USER_AGENT,
};
use std::result::Result;
use ureq::{delete, get, post, Error, Request};

const HEALTHCHECK_API_URL: &str = if cfg!(v3) {
    "https://healthchecks.io/api/v3"
} else if cfg!(v2) {
    "https://healthchecks.io/api/v2"
} else {
    "https://healthchecks.io/api/v1"
};

/// Typealias to prevent some repetitiveness in function definitions
pub type ApiResult<T> = Result<T, HealthchecksApiError>;

/// Client type for communication with the healthchecks.io management API.
#[allow(clippy::module_name_repetitions)]
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
///
/// # Errors
///
/// - Returns [`HealthchecksConfigError::EmptyApiKey`] if `api_key` is empty.
/// - Returns [`HealthchecksConfigError::EmptyUserAgent`] if `user_agent` is [`Some`] but the underlying
/// [`String`] is empty.
pub fn get_client(
    api_key: String,
    user_agent: Option<String>,
) -> Result<ManageClient, HealthchecksConfigError> {
    get_client_with_url(api_key, user_agent, HEALTHCHECK_API_URL.to_owned())
}

/// Create an instance of [`ManageClient`] with a custom endpoint for the API.
/// This is identical to [`get_client`](crate::manage::get_client) which uses it internally,
/// but doesn't default to the hosted version at [healthchecks.io](https://healthchecks.io).
///
/// # Errors
///
/// - Returns [`HealthchecksConfigError::EmptyApiKey`] if `api_key` is empty.
/// - Returns [`HealthchecksConfigError::EmptyUserAgent`] if `user_agent` is [`Some`] but the underlying
/// [`String`] is empty.
/// - Returns [`HealthchecksConfigError::EmptyApiUrl`] if `api_url` is empty.
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
        let user_agent = user_agent.unwrap_or_else(|| DEFAULT_USER_AGENT.to_string());

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
    fn ureq_get(&self, path: &str) -> Request {
        get(path)
            .set("X-Api-Key", &self.api_key)
            .set("User-Agent", &self.user_agent)
    }

    fn ureq_post(&self, path: &str) -> Request {
        post(path)
            .set("X-Api-Key", &self.api_key)
            .set("User-Agent", &self.user_agent)
    }

    fn ureq_delete(&self, path: &str) -> Request {
        delete(path)
            .set("X-Api-Key", &self.api_key)
            .set("User-Agent", &self.user_agent)
    }

    /// Get a list of [`Check`]s.
    ///
    /// # Errors
    /// - Returns [`HealthchecksApiError::InvalidApiKey`] if the API key is invalid
    /// - Returns [`HealthchecksApiError::TransportError`] if there was a network problem
    /// preventing the API request from completing.
    /// - Returns [`HealthchecksApiError::UnexpectedError`] if the healthchecks server responded unexpectedly.
    pub fn get_checks(&self) -> ApiResult<Vec<Check>> {
        #[derive(serde_derive::Deserialize)]
        struct ChecksResult {
            pub checks: Vec<Check>,
        }
        let r = self.ureq_get(&format!("{}/{}", self.api_url, "checks"));
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
    ///
    /// # Errors
    /// - Returns [`HealthchecksApiError::InvalidApiKey`] if the API key is invalid
    /// - Returns [`HealthchecksApiError::TransportError`] if there was a network problem
    /// preventing the API request from completing.
    /// - Returns [`HealthchecksApiError::UnexpectedError`] if the healthchecks server responded unexpectedly.
    /// - Returns [`HealthchecksApiError::AccessDenied`] if the API key does not have access to the `check_id`.
    /// - Returns [`HealthchecksApiError::NoCheckFound`] if no check was found for the given `check_id`.
    pub fn get_check(&self, check_id: &str) -> ApiResult<Check> {
        let r = self.ureq_get(&format!("{}/{}/{}", self.api_url, "checks", check_id));
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
    ///
    /// # Errors
    /// - Returns [`HealthchecksApiError::InvalidApiKey`] if the API key is invalid
    /// - Returns [`HealthchecksApiError::TransportError`] if there was a network problem
    /// preventing the API request from completing.
    /// - Returns [`HealthchecksApiError::UnexpectedError`] if the healthchecks server responded unexpectedly.
    /// - Returns [`HealthchecksApiError::PossibleReadOnlyKey`] if the API key does not have access and could potentially be a [read-only key](https://healthchecks.io/docs/api/).
    /// - Returns [`HealthchecksApiError::NoCheckFound`] if no check was found for the given `check_id`.
    pub fn get_channels(&self) -> ApiResult<Vec<Channel>> {
        #[derive(serde_derive::Deserialize)]
        struct ChannelsResult {
            pub channels: Vec<Channel>,
        }
        let r = self.ureq_get(&format!("{}/{}", self.api_url, "channels"));
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
    ///
    /// # Errors
    /// - Returns [`HealthchecksApiError::InvalidApiKey`] if the API key is invalid
    /// - Returns [`HealthchecksApiError::TransportError`] if there was a network problem
    /// preventing the API request from completing.
    /// - Returns [`HealthchecksApiError::UnexpectedError`] if the healthchecks server responded unexpectedly.
    /// - Returns [`HealthchecksApiError::PossibleReadOnlyKey`] if the API key does not have access and could potentially be a [read-only key](https://healthchecks.io/docs/api/).
    /// - Returns [`HealthchecksApiError::NoCheckFound`] if no check was found for the given `check_id`.
    pub fn pause(&self, check_id: &str) -> ApiResult<Check> {
        let r = self.ureq_post(&format!("{}/checks/{}/pause", self.api_url, check_id));
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
    ///
    /// # Errors
    /// - Returns [`HealthchecksApiError::InvalidApiKey`] if the API key is invalid
    /// - Returns [`HealthchecksApiError::TransportError`] if there was a network problem
    /// preventing the API request from completing.
    /// - Returns [`HealthchecksApiError::UnexpectedError`] if the healthchecks server responded unexpectedly.
    /// - Returns [`HealthchecksApiError::AccessDenied`] if the API key does not have access to the `check_id`.
    /// - Returns [`HealthchecksApiError::NoCheckFound`] if no check was found for the given `check_id`.
    pub fn list_logged_pings(&self, check_id: &str) -> ApiResult<Vec<Ping>> {
        #[derive(serde_derive::Deserialize)]
        struct PingsResult {
            pub pings: Vec<Ping>,
        }
        let r = self.ureq_post(&format!("{}/checks/{}/pings", self.api_url, check_id));
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
    ///
    /// # Errors
    /// - Returns [`HealthchecksApiError::InvalidApiKey`] if the API key is invalid
    /// - Returns [`HealthchecksApiError::TransportError`] if there was a network problem
    /// preventing the API request from completing.
    /// - Returns [`HealthchecksApiError::UnexpectedError`] if the healthchecks server responded unexpectedly.
    /// - Returns [`HealthchecksApiError::AccessDenied`] if the API key does not have access to the `check_id`.
    /// - Returns [`HealthchecksApiError::NoCheckFound`] if no check was found for the given `check_id`.
    pub fn list_status_changes(&self, check_id: &str) -> ApiResult<Vec<Flip>> {
        let r = self.ureq_post(&format!("{}/checks/{}/flips", self.api_url, check_id));
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
    ///
    /// # Errors
    /// - Returns [`HealthchecksApiError::InvalidApiKey`] if the API key is invalid
    /// - Returns [`HealthchecksApiError::TransportError`] if there was a network problem
    /// preventing the API request from completing.
    /// - Returns [`HealthchecksApiError::UnexpectedError`] if the healthchecks server responded unexpectedly.
    /// - Returns [`HealthchecksApiError::AccessDenied`] if the API key does not have access to the `check_id`.
    /// - Returns [`HealthchecksApiError::NoCheckFound`] if no check was found for the given `check_id`.
    pub fn delete(&self, check_id: &str) -> ApiResult<Check> {
        let r = self.ureq_delete(&format!("{}/{}/{}", self.api_url, "checks", check_id));
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
    ///
    /// # Errors
    /// - Returns [`HealthchecksApiError::InvalidApiKey`] if the API key is invalid
    /// - Returns [`HealthchecksApiError::TransportError`] if there was a network problem
    /// preventing the API request from completing.
    /// - Returns [`HealthchecksApiError::UnexpectedError`] if the healthchecks server responded unexpectedly.
    /// - Returns [`HealthchecksApiError::AccessDenied`] if the API key does not have access to the `check_id`.
    /// - Returns [`HealthchecksApiError::NoCheckFound`] if no check was found for the given `check_id`.
    /// - Returns [`HealthchecksApiError::NotWellFormed`] if the request body was malformed. This should never happen in practice,
    /// please report it on GitHub if you encounter an error of this type.
    /// - Returns [`HealthchecksApiError::ExistingCheckMatched`] if the check already exists.
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
    ///
    /// # Errors
    /// - Returns [`HealthchecksApiError::InvalidApiKey`] if the API key is invalid
    /// - Returns [`HealthchecksApiError::TransportError`] if there was a network problem
    /// preventing the API request from completing.
    /// - Returns [`HealthchecksApiError::UnexpectedError`] if the healthchecks server responded unexpectedly.
    /// - Returns [`HealthchecksApiError::AccessDenied`] if the API key does not have access to the `check_id`.
    /// - Returns [`HealthchecksApiError::NoCheckFound`] if no check was found for the given `check_id`.
    /// - Returns [`HealthchecksApiError::NotWellFormed`] if the request body was malformed. This should never happen in practice,
    /// please report it on GitHub if you encounter an error of this type.
    pub fn upsert_check(&self, check: NewCheck) -> ApiResult<(UpsertResult, Check)> {
        let check_json = serde_json::to_value(check)?;
        let r = self.ureq_post(&format!("{}/{}/", self.api_url, "checks"));
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
    ///
    /// # Errors
    /// - Returns [`HealthchecksApiError::InvalidApiKey`] if the API key is invalid
    /// - Returns [`HealthchecksApiError::TransportError`] if there was a network problem
    /// preventing the API request from completing.
    /// - Returns [`HealthchecksApiError::UnexpectedError`] if the healthchecks server responded unexpectedly.
    /// - Returns [`HealthchecksApiError::AccessDenied`] if the API key does not have access to the `check_id`.
    /// - Returns [`HealthchecksApiError::NoCheckFound`] if no check was found for the given `check_id`.
    /// - Returns [`HealthchecksApiError::NotWellFormed`] if the request body was malformed. This should never happen in practice,
    /// please report it on GitHub if you encounter an error of this type.
    pub fn update_check(&self, check: UpdatedCheck, check_id: &str) -> ApiResult<Check> {
        let check_json = serde_json::to_value(check)?;
        let r = self.ureq_post(&format!("{}/{}/{}", self.api_url, "checks", check_id));
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
