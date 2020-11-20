use crate::errors::HealthchecksConfigError;
use crate::util::default_user_agent;
use std::result::Result;
use ureq::get;
use uuid::Uuid;

const HEALTHCHECK_PING_URL: &str = "https://hc-ping.com";

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
        let res = get(&format!("{}/{}", HEALTHCHECK_PING_URL, self.uuid))
            .set("User-Agent", &self.user_agent)
            .call();
        res.status() == 200
    }

    /// Report failure to healthchecks.io. Returns a boolean indicating whether the request succeeded.
    pub fn report_failure(&self) -> bool {
        let res = get(&format!("{}/{}/fail", HEALTHCHECK_PING_URL, self.uuid))
            .set("User-Agent", &self.user_agent)
            .call();
        res.status() == 200
    }

    /// Start a timer on healthchecks.io, to measure script run times. Official documentation for it is available [here](https://healthchecks.io/docs/measuring_script_run_time/).
    pub fn start_timer(&self) -> bool {
        let res = get(&format!("{}/{}/start", HEALTHCHECK_PING_URL, self.uuid))
            .set("User-Agent", &self.user_agent)
            .call();
        res.status() == 200
    }
}
