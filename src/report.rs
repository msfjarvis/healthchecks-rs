use crate::config::HealthcheckConfig;
use ureq::get;

impl HealthcheckConfig {
    pub fn report_success(&self) -> bool {
        let res = get(&format!("https://hc-ping.com/{}", self.uuid)).call();
        res.status() == 200
    }

    pub fn report_failure(&self) -> bool {
        let res = get(&format!("https://hc-ping.com/{}/fail", self.uuid)).call();
        res.status() == 200
    }
}
