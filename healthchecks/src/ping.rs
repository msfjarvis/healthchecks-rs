use crate::errors::HealthchecksConfigError;
use crate::util::default_user_agent;
use std::result::Result;
use std::time::Duration;
use ureq::{Agent, AgentBuilder};
use uuid::Uuid;

const HEALTHCHECK_PING_URL: &str = "https://hc-ping.com";
// This number is sourced from a blog post on healthchecks.io that attempts
// a statistical analysis of what cURL options improve reliability by the biggest
// factor: https://blog.healthchecks.io/2020/01/fighting-packet-loss-with-curl/
const MAX_RETRIES: i8 = 20;

/// Struct that encapsulates the UUID that uniquely identifies your
/// healthchecks.io endpoint. Instances of this expose methods to
/// report status to healthchecks.io
pub struct PingClient {
    pub(crate) uuid: String,
    pub(crate) user_agent: String,
    pub(crate) ureq_agent: Agent,
    pub(crate) api_url: String,
}

/// Create an instance of [`PingClient`] from a String UUID
/// and a custom User-Agent header value. This method runs basic UUID validation and returns Err
/// when the UUID is invalid.
///
/// # Usage
///
/// Starts a timer, sleeps for a second and then reports success.
///
/// ```rust
/// use healthchecks::ping::get_client;
///
/// let client = get_client("2d0a34bd-854d-490e-be2c-1493f7053460").unwrap();
/// ```
pub fn get_client(uuid: &str) -> Result<PingClient, HealthchecksConfigError> {
    get_client_with_url(uuid, HEALTHCHECK_PING_URL)
}

pub fn get_client_with_url(
    uuid: &str,
    api_url: &str,
) -> Result<PingClient, HealthchecksConfigError> {
    if Uuid::parse_str(uuid).is_err() {
        Err(HealthchecksConfigError::InvalidUuid(uuid.to_string()))
    } else if api_url.is_empty() {
        Err(HealthchecksConfigError::EmptyApiUrl)
    } else {
        Ok(PingClient {
            uuid: uuid.to_owned(),
            user_agent: default_user_agent().to_owned(),
            ureq_agent: AgentBuilder::new().timeout(Duration::from_secs(5)).build(),
            api_url: HEALTHCHECK_PING_URL.to_owned(),
        })
    }
}

impl PingClient {
    /// Set the user agent for the given config
    #[must_use]
    pub fn set_user_agent(mut self, user_agent: &str) -> PingClient {
        self.user_agent = user_agent.to_owned();
        self
    }

    /// Report success to healthchecks.io. Returns a boolean indicating whether the request succeeded.
    ///
    /// # Example usage with timer
    ///
    /// ```rust
    /// # use healthchecks::ping::get_client;
    /// # use std::thread::sleep;
    /// # use std::time::Duration;
    /// #
    /// # let client = get_client("2d0a34bd-854d-490e-be2c-1493f7053460").unwrap();
    /// client.start_timer();
    /// std::thread::sleep(Duration::from_millis(1000));
    /// client.report_success();
    /// ```
    #[must_use]
    pub fn report_success(&self) -> bool {
        let mut retries: i8 = 0;
        let request = self
            .ureq_agent
            .get(&format!("{}/{}", self.api_url, self.uuid))
            .set("User-Agent", &self.user_agent);
        while retries < MAX_RETRIES {
            let resp = request.clone().call();
            if resp.is_ok() {
                return true;
            }
            retries += 1;
        }
        if request.call().is_ok() {
            return true;
        }
        false
    }

    /// Report failure to healthchecks.io. Returns a boolean indicating whether the request succeeded.
    #[must_use]
    pub fn report_failure(&self) -> bool {
        let mut retries: i8 = 0;
        let request = self
            .ureq_agent
            .get(&format!("{}/{}/fail", self.api_url, self.uuid))
            .set("User-Agent", &self.user_agent);
        while retries < MAX_RETRIES {
            let resp = request.clone().call();
            if resp.is_ok() {
                return true;
            }
            retries += 1;
        }
        if request.call().is_ok() {
            return true;
        }
        false
    }

    /// Report failure to healthchecks.io with an accompanying log snippet to help debug the failure. Returns
    /// a boolean indicating whether the request succeeded.
    ///
    /// # Example usage with timer
    ///
    /// ```rust
    /// # use healthchecks::ping::get_client;
    /// # use std::thread::sleep;
    /// # use std::time::Duration;
    /// #
    /// # let client = get_client("2d0a34bd-854d-490e-be2c-1493f7053460").unwrap();
    /// client.start_timer();
    /// std::thread::sleep(Duration::from_millis(1000));
    /// client.report_failure_with_logs("slept too much...zzzzzzz");
    /// ```
    #[must_use]
    pub fn report_failure_with_logs(&self, data: &str) -> bool {
        let mut retries: i8 = 0;
        let request = self
            .ureq_agent
            .post(&format!("{}/{}/fail", self.api_url, self.uuid))
            .set("User-Agent", &self.user_agent);
        while retries < MAX_RETRIES {
            let resp = request.clone().send_string(data);
            if resp.is_ok() {
                return true;
            }
            retries += 1;
        }
        if request.send_string(data).is_ok() {
            return true;
        }
        false
    }

    /// Start a timer on healthchecks.io, to measure script run times. Official documentation for it is available [here](https://healthchecks.io/docs/measuring_script_run_time/).
    #[must_use]
    pub fn start_timer(&self) -> bool {
        let mut retries: i8 = 0;
        let request = self
            .ureq_agent
            .get(&format!("{}/{}/start", self.api_url, self.uuid))
            .set("User-Agent", &self.user_agent);
        while retries < MAX_RETRIES {
            let resp = request.clone().call();
            if resp.is_ok() {
                return true;
            }
            retries += 1;
        }
        if request.call().is_ok() {
            return true;
        }
        false
    }
}
