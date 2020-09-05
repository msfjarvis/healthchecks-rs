use nanoserde::{DeJson, SerJson};

/// This struct encapsulates a check as represented in the healthchecks.io
/// API. Fields marked optional are either optional in the default API response
/// or can be present or missing if a read-only API key is used.
#[derive(Clone, Debug, Default, DeJson, SerJson)]
pub struct Check {
    /// Name of the check.
    pub name: String,
    /// Space separated list of tags set on this check.
    pub tags: String,
    /// Description of the check.
    pub desc: String,
    /// Grace period in minutes before the check is considered as failed.
    pub grace: i64,
    /// Number of times the check has pinged healthchecks.io.
    #[nserde(rename = "n_pings")]
    pub n_pings: i64,
    /// Current status of the check.
    pub status: String,
    /// UTC timestamp of the last known ping.
    #[nserde(rename = "last_ping")]
    pub last_ping: Option<String>,
    /// UTC timestamp of the next expected ping based on grace period. Will be
    /// None in case no grace period is set.
    #[nserde(rename = "next_ping")]
    pub next_ping: Option<String>,
    /// Indicates if the ping has been manually paused and will not resume automatically
    /// on a new ping. These checks need to manually be resumed from the web dashboard.
    #[nserde(rename = "manual_resume")]
    pub manual_resume: bool,
    /// A GET request to this URL is a valid ping for this check. This field is None if a read-only API token is used.
    #[nserde(rename = "ping_url")]
    pub ping_url: Option<String>,
    /// URL to GET this specific check or POST an updated version. This field is None if a read-only API token is used.
    #[nserde(rename = "update_url")]
    pub update_url: Option<String>,
    /// URL to pause monitoring for this check. The next ping will resume monitoring.
    /// This field is None if a read-only API token is used.
    #[nserde(rename = "pause_url")]
    pub pause_url: Option<String>,
    /// Comma-separated list of IDs of the integration channels associated with this check. Is None when no integrations
    /// are configured.
    pub channels: Option<String>,
    /// Expected period of the check, in seconds. Is None when no timeout is set.
    pub timeout: Option<i64>,
    /// A cron expression defining this check's schedule. Is None when no schedule is configured.
    pub schedule: Option<String>,
    /// The timezone for the server which pings for this check's status. Can be None by itself or when no schedule is configured.
    pub tz: Option<String>,
    /// A stable identifier generated when using a read-only API key. This can be used in place of an exact UUID to get individual checks.
    pub unique_key: Option<String>,
}

/// Represents an integration, like email or sms.
#[derive(Clone, Debug, Default, DeJson, SerJson)]
pub struct Channel {
    pub id: String,
    pub name: String,
    pub kind: String,
}

/// Represents a new check that is initialized locally then created on healthchecks.io
/// using the admin API. It contains a lot less fields than the [Check](struct.Check.html)
/// struct so we implement it separately.
#[derive(Clone, Debug, Default, DeJson, SerJson)]
pub struct NewCheck {
    /// Name of the check.
    pub name: Option<String>,
    /// Space separated list of tags set on this check.
    pub tags: Option<String>,
    /// Description of the check.
    pub desc: Option<String>,
    /// Expected period of the check, in seconds. Is None when no timeout is set.
    pub timeout: Option<i32>,
    /// Grace period in minutes before the check is considered as failed.
    pub grace: Option<i32>,
    /// A cron expression defining this check's schedule. Is None when no schedule is configured.
    pub schedule: Option<String>,
    /// The timezone for the server which pings for this check's status. Can be None by itself or when no schedule is configured.
    pub tz: Option<String>,
    /// Indicates if the ping has been manually paused and will not resume automatically
    /// on a new ping. These checks need to manually be resumed from the web dashboard.
    pub manual_resume: Option<String>,
    /// Comma-separated list of IDs of the integration channels associated with this check. Is None when no integrations
    /// are configured.
    pub channels: Option<String>,
    /// List of fields that must be unique before the check is created.
    pub unique: Option<Vec<String>>,
}
