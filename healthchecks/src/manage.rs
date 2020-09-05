use crate::{
    model::{Channel, Check},
    util::default_user_agent,
};
use anyhow::{anyhow, Context};
use nanoserde::DeJson;
use ureq::{delete, get, post, Request};

const HEALTHCHECK_API_URL: &str = "https://healthchecks.io/api/v1/";

/// Struct that encapsulates the API key used to communicate with the healthchecks.io
/// management API. Instances of this struct expose methods to query the API.
pub struct ApiConfig {
    pub(crate) api_key: String,
    pub(crate) user_agent: String,
}

/// Create an instance of [ApiConfig](struct.ApiConfig.html) from a given API key. No validation
/// is performed.
#[inline]
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

    /// Get a list of [Check](../model/struct.Check.html)s.
    pub fn get_checks(&self) -> anyhow::Result<Vec<Check>> {
        #[derive(DeJson)]
        struct ChecksResult {
            pub checks: Vec<Check>,
        }
        let mut r = &mut get(&format!("{}/{}", HEALTHCHECK_API_URL, "checks"));
        r = self.set_headers(r);
        let resp = r.call();
        match resp.status() {
            200 => {
                let res: ChecksResult = DeJson::deserialize_json(&resp.into_string()?)
                    .context("Failed to parse API response")?;
                Ok(res.checks)
            }
            401 => Err(anyhow!("Invalid API key")),
            _ => Err(anyhow!("Unexpected error: {}", resp.error())),
        }
    }

    /// Get a [Check](../model/struct.Check.html) with the given UUID or unique key.
    pub fn get_check(&self, check_id: &str) -> anyhow::Result<Check> {
        let mut r = &mut get(&format!(
            "{}/{}/{}",
            HEALTHCHECK_API_URL, "checks", check_id
        ));
        r = self.set_headers(r);
        let resp = r.call();
        match resp.status() {
            200 => Ok(DeJson::deserialize_json(&resp.into_string()?)
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

    /// Returns a list of [Channel](../model/struct.Channel.html)s belonging to the project.
    pub fn get_channels(&self) -> anyhow::Result<Vec<Channel>> {
        #[derive(DeJson)]
        struct ChannelsResult {
            pub channels: Vec<Channel>,
        }
        let mut r = &mut get(&format!("{}/{}", HEALTHCHECK_API_URL, "channels"));
        r = self.set_headers(r);
        let resp = r.call();
        match resp.status() {
            200 => {
                let res: ChannelsResult = DeJson::deserialize_json(&resp.into_string()?)
                    .context("Failed to parse API response")?;
                Ok(res.channels)
            }
            401 => Err(anyhow!(
                "Invalid API key: make sure you're not using a read-only key"
            )),
            _ => Err(anyhow!("Unexpected error: {}", resp.error())),
        }
    }

    /// Pauses the [Check](../model/struct.Check.html) with the given UUID or unique key.
    pub fn pause(&self, check_id: &str) -> anyhow::Result<Check> {
        let mut r = &mut post(&format!(
            "{}/checks/{}/pause",
            HEALTHCHECK_API_URL, check_id
        ));
        r = self.set_headers(r);
        let resp = r.send_string("");
        match resp.status() {
            200 => Ok(DeJson::deserialize_json(&resp.into_string()?)
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

    /// Deletes the [Check](../model/struct.Check.html) with the given UUID or unique key.
    pub fn delete(&self, check_id: &str) -> anyhow::Result<Check> {
        let mut r = &mut delete(&format!(
            "{}/{}/{}",
            HEALTHCHECK_API_URL, "checks", check_id
        ));
        r = self.set_headers(r);
        let resp = r.call();
        match resp.status() {
            200 => Ok(DeJson::deserialize_json(&resp.into_string()?)
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
}
