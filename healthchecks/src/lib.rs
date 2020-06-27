use ureq::get;
use uuid::Uuid;

const HEALTHCHECK_PING_URL: &str = "https://hc-ping.com";

const fn default_user_agent() -> &'static str {
    concat!("healthchecks-rs/", env!("CARGO_PKG_VERSION"))
}

/// Struct that encapsulates the UUID that uniquely identifies your
/// healthchecks.io endpoint. Instances of this exposes methods to
/// report status to healthchecks.io
pub struct HealthcheckConfig {
    pub(crate) uuid: String,
    pub(crate) user_agent: String,
}

/// Create an instance of [HealthcheckConfig](struct.HealthcheckConfig.html) from a String UUID
/// and a custom User-Agent header value. Like create_config, this method also runs basic UUID validation
/// and panics if the UUID is not valid.
#[inline]
pub fn create_config(uuid: String, user_agent: Option<String>) -> HealthcheckConfig {
    if Uuid::parse_str(&uuid).is_ok() {
        HealthcheckConfig {
            uuid,
            user_agent: user_agent.unwrap_or(default_user_agent().to_owned()),
        }
    } else {
        panic!("Invalid UUID: {}", uuid)
    }
}

impl HealthcheckConfig {
    /// Report success to healthchecks.io. Returns a boolean indicating whether the request succeeded.
    #[inline]
    pub fn report_success(&self) -> bool {
        let res = get(&format!("{}/{}", HEALTHCHECK_PING_URL, self.uuid))
            .set("User-Agent", &self.user_agent)
            .call();
        res.status() == 200
    }

    /// Report failure to healthchecks.io. Returns a boolean indicating whether the request succeeded.
    #[inline]
    pub fn report_failure(&self) -> bool {
        let res = get(&format!("{}/{}/fail", HEALTHCHECK_PING_URL, self.uuid))
            .set("User-Agent", &self.user_agent)
            .call();
        res.status() == 200
    }

    /// Start a timer on healthchecks.io, to measure script run times. Official documentation for it is available [here](https://healthchecks.io/docs/measuring_script_run_time/).
    #[inline]
    pub fn start_timer(&self) -> bool {
        let res = get(&format!("{}/{}/start", HEALTHCHECK_PING_URL, self.uuid))
            .set("User-Agent", &self.user_agent)
            .call();
        res.status() == 200
    }
}
