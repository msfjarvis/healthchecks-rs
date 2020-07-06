use crate::model::Check;
use crate::model::ChecksResult;
use crate::util::default_user_agent;
use anyhow::anyhow;
use ureq::get;

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
pub fn create_config(api_key: String, user_agent: Option<String>) -> ApiConfig {
    ApiConfig {
        api_key,
        user_agent: user_agent.unwrap_or(default_user_agent().to_owned()),
    }
}

impl ApiConfig {
    /// Get a list of [Check](../model/struct.Check.html)s.
    pub fn get_checks(&self) -> anyhow::Result<Vec<Check>> {
        let resp = get(&format!("{}/{}", HEALTHCHECK_API_URL, "checks"))
            .set("X-Api-Key", &self.api_key)
            .set("User-Agent", &self.user_agent)
            .call();
        if resp.ok() {
            Ok(resp.into_json_deserialize::<ChecksResult>()?.checks)
        } else {
            Err(anyhow!("error {}: {}", resp.status(), resp.into_string()?))
        }
    }
}
