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

/// Struct that encapsulates the API key used to communicate with the healthchecks.io
/// management API. Instances of this struct expose methods to query the API.
pub struct ManageClient {
    pub(crate) api_key: String,
    pub(crate) user_agent: String,
}

/// Create an instance of [`ManageClient`] from a given API key. No validation
/// is performed.
pub fn get_client(
    api_key: String,
    user_agent: Option<String>,
) -> Result<ManageClient, HealthchecksConfigError> {
    if api_key.is_empty() {
        Err(HealthchecksConfigError::EmptyApiKey)
    } else if let Some(ua) = user_agent {
        if ua.is_empty() {
            Err(HealthchecksConfigError::EmptyUserAgent)
        } else {
            Ok(ManageClient {
                api_key,
                user_agent: ua,
            })
        }
    } else {
        Ok(ManageClient {
            api_key,
            user_agent: default_user_agent().to_owned(),
        })
    }
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
        let r = self.ureq_get(format!("{}/{}", HEALTHCHECK_API_URL, "checks"));
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
        let r = self.ureq_get(format!("{}/{}/{}", HEALTHCHECK_API_URL, "checks", check_id));
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
        let r = self.ureq_get(format!("{}/{}", HEALTHCHECK_API_URL, "channels"));
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
        let r = self.ureq_post(format!("{}/checks/{}/pause", HEALTHCHECK_API_URL, check_id));
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
        let r = self.ureq_post(format!("{}/checks/{}/pings", HEALTHCHECK_API_URL, check_id));
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
        let r = self.ureq_post(format!("{}/checks/{}/flips", HEALTHCHECK_API_URL, check_id));
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
        let r = self.ureq_delete(format!("{}/{}/{}", HEALTHCHECK_API_URL, "checks", check_id));
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
        let check_json = serde_json::to_value(check)?;
        let r = self.ureq_post(format!("{}/{}/", HEALTHCHECK_API_URL, "checks"));
        match r
            .set("Content-Type", "application/json")
            .send_json(check_json)
        {
            Ok(response) => match response.status() {
                201 => Ok(response.into_json::<Check>()?),
                200 => Err(HealthchecksApiError::ExistingCheckMatched),
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
        let r = self.ureq_post(format!("{}/{}/{}", HEALTHCHECK_API_URL, "checks", check_id));
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
