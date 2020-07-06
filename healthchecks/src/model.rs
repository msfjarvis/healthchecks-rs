use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ChecksResult {
    pub checks: Vec<Check>,
}
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Check {
    pub name: String,
    pub tags: String,
    pub desc: String,
    pub grace: i64,
    #[serde(rename = "n_pings")]
    pub n_pings: i64,
    pub status: String,
    #[serde(rename = "last_ping")]
    pub last_ping: String,
    #[serde(rename = "next_ping")]
    pub next_ping: Option<String>,
    #[serde(rename = "manual_resume")]
    pub manual_resume: bool,
    #[serde(rename = "ping_url")]
    pub ping_url: Option<String>,
    #[serde(rename = "update_url")]
    pub update_url: Option<String>,
    #[serde(rename = "pause_url")]
    pub pause_url: Option<String>,
    pub channels: Option<String>,
    pub timeout: Option<i64>,
    pub schedule: Option<String>,
    pub tz: Option<String>,
}
