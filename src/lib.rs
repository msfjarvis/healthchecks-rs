use ureq::get;
use uuid::Uuid;

const USER_AGENT: &'static str = "healthchecks.io/0.1.0";

/// Struct that encapsulates the UUID that uniquely identifies your
/// healthchecks.io endpoint. Instances of this exposes methods to
/// report status to healthchecks.io
pub struct HealthcheckConfig {
    pub(crate) uuid: String,
}

/// Create an instance of [HealthcheckConfig](struct.HealthcheckConfig.html) from a str uuid.
/// The method runs basic UUID validation and will panic if there's a failure parsing the provided
/// uuid.
pub fn create_config(uuid: &str) -> HealthcheckConfig {
    if let Ok(_) = Uuid::parse_str(uuid) {
        HealthcheckConfig {
            uuid: uuid.to_string(),
        }
    } else {
        panic!("Invalid UUID: {}", uuid)
    }
}

impl HealthcheckConfig {
    /// Report success to healthchecks.io. Returns a boolean indicating whether the request succeeded.
    pub fn report_success(&self) -> bool {
        let res = get(&format!("https://hc-ping.com/{}", self.uuid))
            .set("User-Agent", USER_AGENT)
            .call();
        res.status() == 200
    }

    /// Report failure to healthchecks.io. Returns a boolean indicating whether the request succeeded.
    pub fn report_failure(&self) -> bool {
        let res = get(&format!("https://hc-ping.com/{}/fail", self.uuid))
            .set("User-Agent", USER_AGENT)
            .call();
        res.status() == 200
    }

    /// Start a timer on healthchecks.io, to measure script run times. Official documentation for it is available [here](https://healthchecks.io/docs/measuring_script_run_time/).
    pub fn start_timer(&self) -> bool {
        let res = get(&format!("https://hc-ping.com/{}/start", self.uuid))
            .set("User-Agent", USER_AGENT)
            .call();
        res.status() == 200
    }
}
