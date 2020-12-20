use crate::errors::HealthchecksConfigError;
use crate::util::default_user_agent;
use std::result::Result;
use std::time::Duration;
use uuid::Uuid;

const HEALTHCHECK_PING_URL: &str = "https://hc-ping.com";
// This number is sourced from a blog post on healthchecks.io that attempts
// a statistical analysis of what cURL options improve reliability by the biggest
// factor: https://blog.healthchecks.io/2020/01/fighting-packet-loss-with-curl/
const MAX_RETRIES: i8 = 20;

/// Struct that encapsulates the UUID that uniquely identifies your
/// healthchecks.io endpoint. Instances of this expose methods to
/// report status to healthchecks.io
pub struct HealthcheckConfig {
    pub(crate) uuid: String,
    pub(crate) user_agent: String,
}

/// Create an instance of [`HealthcheckConfig`] from a String UUID
/// and a custom User-Agent header value. This method runs basic UUID validation and returns Err
/// when the UUID is invalid.
pub fn get_config(uuid: &str) -> Result<HealthcheckConfig, HealthchecksConfigError> {
    if Uuid::parse_str(uuid).is_err() {
        Err(HealthchecksConfigError::InvalidUUID(uuid.to_string()))
    } else {
        Ok(HealthcheckConfig {
            uuid: uuid.to_owned(),
            user_agent: default_user_agent().to_owned(),
        })
    }
}

impl HealthcheckConfig {
    /// Set the user agent for the given config
    pub fn set_user_agent(mut self, user_agent: &str) -> HealthcheckConfig {
        self.user_agent = user_agent.to_owned();
        self
    }

    /// Report success to healthchecks.io. Returns a boolean indicating whether the request succeeded.
    pub fn report_success(&self) -> bool {
        let mut retries: i8 = 0;
        let mut request = ureq::get(&format!("{}/{}", HEALTHCHECK_PING_URL, self.uuid));
        while retries < MAX_RETRIES {
            let resp = request
                .set("User-Agent", &self.user_agent)
                .timeout(Duration::from_secs(5))
                .call();
            if resp.ok() {
                return true;
            }
            retries += 1;
        }
        false
    }

    /// Report failure to healthchecks.io. Returns a boolean indicating whether the request succeeded.
    pub fn report_failure(&self) -> bool {
        let mut retries: i8 = 0;
        let mut request = ureq::get(&format!("{}/{}/fail", HEALTHCHECK_PING_URL, self.uuid));
        while retries < MAX_RETRIES {
            let resp = request
                .set("User-Agent", &self.user_agent)
                .timeout(Duration::from_secs(5))
                .call();
            if resp.ok() {
                return true;
            }
            retries += 1;
        }
        false
    }

    /// Report failure to healthchecks.io with an accompanying log snippet to help debug the failure. Returns
    /// a boolean indicating wther the request succeeded.
    pub fn report_failure_with_logs<'a>(&self, data: &'a str) -> bool {
        let mut retries: i8 = 0;
        let mut request = ureq::post(&format!("{}/{}/fail", HEALTHCHECK_PING_URL, self.uuid));
        while retries < MAX_RETRIES {
            let resp = request
                .set("User-Agent", &self.user_agent)
                .timeout(Duration::from_secs(5))
                .send_string(data);
            if resp.ok() {
                return true;
            }
            retries += 1;
        }
        false
    }

    /// Start a timer on healthchecks.io, to measure script run times. Official documentation for it is available [here](https://healthchecks.io/docs/measuring_script_run_time/).
    pub fn start_timer(&self) -> bool {
        let mut retries: i8 = 0;
        let mut request = ureq::get(&format!("{}/{}/start", HEALTHCHECK_PING_URL, self.uuid));
        while retries < MAX_RETRIES {
            let resp = request
                .set("User-Agent", &self.user_agent)
                .timeout(Duration::from_secs(5))
                .call();
            if resp.ok() {
                return true;
            }
            retries += 1;
        }
        false
    }
}
