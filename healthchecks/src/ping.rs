use crate::{errors::HealthchecksConfigError, DEFAULT_USER_AGENT};
use std::result::Result;
use std::time::Duration;
use ureq::{Agent, AgentBuilder};
use uuid::Uuid;

const HEALTHCHECK_PING_URL: &str = "https://hc-ping.com";
// This number is sourced from a blog post on healthchecks.io that attempts
// a statistical analysis of what cURL options improve reliability by the biggest
// factor: https://blog.healthchecks.io/2020/01/fighting-packet-loss-with-curl/
const MAX_RETRIES: i8 = 20;

/// Client type for communication with the healthchecks.io ping API for a single
/// check.
#[allow(clippy::module_name_repetitions)]
pub struct PingClient {
    pub(crate) uuid: String,
    pub(crate) user_agent: String,
    pub(crate) ureq_agent: Agent,
    pub(crate) api_url: String,
}

/// Create an instance of [`PingClient`] from the UUID of a check.
///
/// # Errors
/// - Returns [`HealthchecksConfigError::InvalidUuid`] if `uuid` is not
///   a valid UUID
pub fn get_client(uuid: &str) -> Result<PingClient, HealthchecksConfigError> {
    get_client_with_url(uuid, HEALTHCHECK_PING_URL)
}

/// Same as [`get_client`](crate::ping::get_client), with the ability to use a custom instance of the
/// healthchecks server.
/// # Errors
///
/// - Returns [`HealthchecksConfigError::InvalidUuid`] if `uuid` is not
///   a valid UUID
/// - Returns [`HealthchecksConfigError::EmptyApiUrl`] if `api_url` is empty.
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
            user_agent: DEFAULT_USER_AGENT.to_string(),
            ureq_agent: AgentBuilder::new().timeout(Duration::from_secs(5)).build(),
            api_url: HEALTHCHECK_PING_URL.to_owned(),
        })
    }
}

/// Helper method to add `run_id` to URL if it's provided
fn maybe_add_run_id(url: String, run_id: Option<&Uuid>) -> String {
    if let Some(rid) = run_id {
        format!("{url}?rid={rid}")
    } else {
        url
    }
}

impl PingClient {
    /// Set the user agent for the given config
    #[must_use]
    pub fn set_user_agent(mut self, user_agent: &str) -> PingClient {
        user_agent.clone_into(&mut self.user_agent);
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
        self.report_success_with_run_id(None)
    }

    /// Report success to healthchecks.io with a specific run ID to track concurrent executions.
    /// Returns a boolean indicating whether the request succeeded.
    ///
    /// The run ID allows healthchecks.io to correctly calculate execution times when multiple
    /// instances of the same job run concurrently. See the [healthchecks.io documentation](https://healthchecks.io/docs/measuring_script_run_time/)
    /// for more information about run IDs.
    ///
    /// # Example usage with run ID
    ///
    /// ```rust
    /// # use healthchecks::ping::get_client;
    /// # use uuid::Uuid;
    /// #
    /// # let client = get_client("2d0a34bd-854d-490e-be2c-1493f7053460").unwrap();
    /// let run_id = Uuid::new_v4();
    /// client.start_timer_with_run_id(Some(&run_id));
    /// // Do work
    /// client.report_success_with_run_id(Some(&run_id));
    /// ```
    #[must_use]
    pub fn report_success_with_run_id(&self, run_id: Option<&Uuid>) -> bool {
        let mut retries: i8 = 0;
        let url = maybe_add_run_id(format!("{}/{}", self.api_url, self.uuid), run_id);
        let request = self
            .ureq_agent
            .get(&url)
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
        self.report_failure_with_run_id(None)
    }

    /// Report failure to healthchecks.io with a specific run ID to track concurrent executions.
    /// Returns a boolean indicating whether the request succeeded.
    ///
    /// The run ID allows healthchecks.io to correctly calculate execution times when multiple
    /// instances of the same job run concurrently.
    #[must_use]
    pub fn report_failure_with_run_id(&self, run_id: Option<&Uuid>) -> bool {
        let mut retries: i8 = 0;
        let url = maybe_add_run_id(format!("{}/{}/fail", self.api_url, self.uuid), run_id);
        let request = self
            .ureq_agent
            .get(&url)
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
        self.report_failure_with_logs_and_run_id(data, None)
    }

    /// Report failure to healthchecks.io with an accompanying log snippet and a specific run ID to help debug the failure.
    /// Returns a boolean indicating whether the request succeeded.
    ///
    /// The run ID allows healthchecks.io to correctly calculate execution times when multiple
    /// instances of the same job run concurrently.
    #[must_use]
    pub fn report_failure_with_logs_and_run_id(&self, data: &str, run_id: Option<&Uuid>) -> bool {
        let mut retries: i8 = 0;
        let url = maybe_add_run_id(format!("{}/{}/fail", self.api_url, self.uuid), run_id);
        let request = self
            .ureq_agent
            .post(&url)
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
        self.start_timer_with_run_id(None)
    }

    /// Start a timer on healthchecks.io with a specific run ID, to measure script run times.
    /// Official documentation for it is available [here](https://healthchecks.io/docs/measuring_script_run_time/).
    ///
    /// The run ID allows healthchecks.io to correctly calculate execution times when multiple
    /// instances of the same job run concurrently.
    #[must_use]
    pub fn start_timer_with_run_id(&self, run_id: Option<&Uuid>) -> bool {
        let mut retries: i8 = 0;
        let url = maybe_add_run_id(format!("{}/{}/start", self.api_url, self.uuid), run_id);
        let request = self
            .ureq_agent
            .get(&url)
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
