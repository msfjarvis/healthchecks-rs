use crate::{
    model::{Channel, Check, NewCheck, UpdatedCheck},
    util::default_user_agent,
};
use anyhow::{anyhow, Context};
use ureq::{delete, get, post, Request};

const HEALTHCHECK_API_URL: &str = "https://healthchecks.io/api/v1/";

/// Struct that encapsulates the API key used to communicate with the healthchecks.io
/// management API. Instances of this struct expose methods to query the API.
pub struct ApiConfig {
    pub(crate) api_key: String,
    pub(crate) user_agent: String,
}

/// Create an instance of [`ApiConfig`] from a given API key. No validation
/// is performed.
pub fn create_config(api_key: String, user_agent: Option<String>) -> anyhow::Result<ApiConfig> {
    if api_key.is_empty() {
        Err(anyhow!("API key must not be empty"))
    } else if let Some(ua) = user_agent {
        if ua.is_empty() {
            Err(anyhow!("User Agent must not be empty"))
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
    pub fn get_checks(&self) -> anyhow::Result<Vec<Check>> {
        #[derive(serde::Deserialize)]
        struct ChecksResult {
            pub checks: Vec<Check>,
        }
        let mut r = &mut get(&format!("{}/{}", HEALTHCHECK_API_URL, "checks"));
        r = self.set_headers(r);
        let resp = r.call();
        match resp.status() {
            200 => Ok(resp
                .into_json_deserialize::<ChecksResult>()
                .context("Failed to parse API response")?
                .checks),
            401 => Err(anyhow!("Invalid API key")),
            _ => Err(anyhow!("Unexpected error: {}", resp.error())),
        }
    }

    /// Get a [`Check`] with the given UUID or unique key.
    pub fn get_check(&self, check_id: &str) -> anyhow::Result<Check> {
        let mut r = &mut get(&format!(
            "{}/{}/{}",
            HEALTHCHECK_API_URL, "checks", check_id
        ));
        r = self.set_headers(r);
        let resp = r.call();
        match resp.status() {
            200 => Ok(resp
                .into_json_deserialize::<Check>()
                .context("Failed to parse API response")?),
            401 => Err(anyhow!("Invalid API key")),
            403 => Err(anyhow!("Access denied")),
            404 => Err(anyhow!(
                "Failed to find a check with the uuid: {}",
                check_id
            )),
            _ => Err(anyhow!("Unexpected error: {}", resp.error())),
        }
    }

    /// Returns a list of [`Channel`]s belonging to the project.
    pub fn get_channels(&self) -> anyhow::Result<Vec<Channel>> {
        #[derive(serde::Deserialize)]
        struct ChannelsResult {
            pub channels: Vec<Channel>,
        }
        let mut r = &mut get(&format!("{}/{}", HEALTHCHECK_API_URL, "channels"));
        r = self.set_headers(r);
        let resp = r.call();
        match resp.status() {
            200 => Ok(resp
                .into_json_deserialize::<ChannelsResult>()
                .context("Failed to parse API response")?
                .channels),
            401 => Err(anyhow!(
                "Invalid API key: make sure you're not using a read-only key"
            )),
            _ => Err(anyhow!("Unexpected error: {}", resp.error())),
        }
    }

    /// Pauses the [`Check`] with the given UUID or unique key.
    pub fn pause(&self, check_id: &str) -> anyhow::Result<Check> {
        let mut r = &mut post(&format!(
            "{}/checks/{}/pause",
            HEALTHCHECK_API_URL, check_id
        ));
        r = self.set_headers(r);
        let resp = r.send_string("");
        match resp.status() {
            200 => Ok(resp
                .into_json_deserialize::<Check>()
                .context("Failed to parse API response")?),
            401 => Err(anyhow!("Invalid API key")),
            403 => Err(anyhow!("Access denied")),
            404 => Err(anyhow!(
                "Failed to find a check with the uuid: {}",
                check_id
            )),
            _ => Err(anyhow!("Unexpected error: {}", resp.error())),
        }
    }

    /// Deletes the [`Check`] with the given UUID or unique key.
    pub fn delete(&self, check_id: &str) -> anyhow::Result<Check> {
        let mut r = &mut delete(&format!(
            "{}/{}/{}",
            HEALTHCHECK_API_URL, "checks", check_id
        ));
        r = self.set_headers(r);
        let resp = r.call();
        match resp.status() {
            200 => Ok(resp
                .into_json_deserialize::<Check>()
                .context("Failed to parse API response")?),
            401 => Err(anyhow!("Invalid API key")),
            403 => Err(anyhow!("Access denied")),
            404 => Err(anyhow!(
                "Failed to find a check with the uuid: {}",
                check_id
            )),
            _ => Err(anyhow!("Unexpected error: {}", resp.error())),
        }
    }

    /// Creates a new check with the given [`NewCheck`] configuration.
    pub fn create_check(&self, check: NewCheck) -> anyhow::Result<Check> {
        let check_json =
            serde_json::to_value(check).expect("Failed to convert check into valid JSON");
        let mut r = &mut post(&format!("{}/{}/", HEALTHCHECK_API_URL, "checks"));
        r = self.set_headers(r);
        let resp = r
            .set("Content-Type", "application/json")
            .send_json(check_json);
        match resp.status() {
            201 => Ok(resp
                .into_json_deserialize::<Check>()
                .context("Failed to parse API response")?),
            200 => Err(anyhow!(
                "An existing check was matched based on the \"unique\" parameter"
            )),
            400 => Err(anyhow!(
                "The request is not well-formed, violates schema, or uses invalid field values"
            )),
            401 => Err(anyhow!("Invalid API key")),
            403 => Err(anyhow!("The account's check limit has been reached")),
            _ => Err(anyhow!("Unexpected error: {}", resp.error())),
        }
    }

    /// Update the check with the given `check_id` with the data from `check`.
    pub fn update_check(&self, check: UpdatedCheck, check_id: &str) -> anyhow::Result<Check> {
        let check_json =
            serde_json::to_value(check).expect("Failed to convert check into valid JSON");
        let mut r = &mut post(&format!(
            "{}/{}/{}",
            HEALTHCHECK_API_URL, "checks", check_id
        ));
        r = self.set_headers(r);
        let resp = r
            .set("Content-Type", "application/json")
            .send_json(check_json);
        match resp.status() {
            200 => Ok(resp
                .into_json_deserialize::<Check>()
                .context("Failed to parse API response")?),
            400 => Err(anyhow!(
                "The request is not well-formed, violates schema, or uses invalid field values"
            )),
            401 => Err(anyhow!("Invalid API key")),
            403 => Err(anyhow!("Access denied")),
            404 => Err(anyhow!(
                "Failed to find a check with the uuid: {}",
                check_id
            )),
            _ => Err(anyhow!("Unexpected error: {}", resp.error())),
        }
    }
}
