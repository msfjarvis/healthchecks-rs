use crate::model::Channel;
use crate::model::ChannelsResult;
use crate::model::Check;
use crate::model::ChecksResult;
use crate::util::default_user_agent;
use anyhow::anyhow;
use ureq::get;
use ureq::Request;

const HEALTHCHECK_API_URL: &str = "https://healthchecks.io/api/v1/";

/// Struct that encapsulates the API key used to communicate with the healthchecks.io
/// management API. Instances of this struct expose methods to query the API.
pub struct ApiConfig {
    pub(crate) api_key: String,
    pub(crate) user_agent: String,
}

/// Create an instance of [ApiConfig](struct.ApiConfig.html) from a given API key. There's no
/// validation being performed on the provided key.
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
    fn set_headers<'a>(&self, req: &'a mut Request) -> &'a mut Request {
        req.set("X-Api-Key", &self.api_key)
            .set("User-Agent", &self.user_agent)
    }

    /// Get a list of [Check](../model/struct.Check.html)s.
    pub fn get_checks(&self) -> anyhow::Result<Vec<Check>> {
        let mut r = &mut get(&format!("{}/{}", HEALTHCHECK_API_URL, "checks"));
        r = self.set_headers(r);
        let resp = r.call();
        match resp.status() {
            200 => Ok(resp.into_json_deserialize::<ChecksResult>()?.checks),
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
            200 => Ok(resp.into_json_deserialize::<Check>()?),
            401 => Err(anyhow!("Invalid API key")),
            403 => Err(anyhow!("Access denied")),
            404 => Err(anyhow!(
                "Failed to find a check with the uuid: {}",
                check_id
            )),
            _ => Err(anyhow!("Unexpected error: {}", resp.error())),
        }
    }

    /// Returns a list of integrations belonging to the project.
    pub fn get_channels(&self) -> anyhow::Result<Vec<Channel>> {
        let mut r = &mut get(&format!("{}/{}", HEALTHCHECK_API_URL, "channels"));
        r = self.set_headers(r);
        let resp = r.call();
        match resp.status() {
            200 => Ok(resp.into_json_deserialize::<ChannelsResult>()?.channels),
            401 => Err(anyhow!(
                "Invalid API key: make sure you're not using a read-only key"
            )),
            _ => Err(anyhow!("Unexpected error: {}", resp.error())),
        }
    }
}
