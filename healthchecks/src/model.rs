use serde::{Deserialize, Serialize};

/// This struct encapsulates a check as represented in the healthchecks.io
/// API. Fields marked optional are either optional in the default API response
/// or can be present or missing if a read-only API key is used.
#[derive(Clone, Debug, Deserialize, Serialize)]
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
    pub n_pings: i64,
    /// Current status of the check.
    pub status: String,
    /// UTC timestamp of the last known ping.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_ping: Option<String>,
    /// UTC timestamp of the next expected ping based on grace period. Will be
    /// None in case no grace period is set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_ping: Option<String>,
    /// Indicates if the ping has been manually paused and will not resume automatically
    /// on a new ping. These checks need to manually be resumed from the web dashboard.
    pub manual_resume: bool,
    /// A GET request to this URL is a valid ping for this check. This field is None if a read-only API token is used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ping_url: Option<String>,
    /// URL to GET this specific check or POST an updated version. This field is None if a read-only API token is used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_url: Option<String>,
    /// URL to pause monitoring for this check. The next ping will resume monitoring.
    /// This field is None if a read-only API token is used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pause_url: Option<String>,
    /// Comma-separated list of IDs of the integration channels associated with this check. Is None when no integrations
    /// are configured.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channels: Option<String>,
    /// Expected period of the check, in seconds. Is None when no timeout is set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<i64>,
    /// A cron expression defining this check's schedule. Is None when no schedule is configured.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule: Option<String>,
    /// The timezone for the server which pings for this check's status. Can be None by itself or when no schedule is configured.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tz: Option<String>,
    /// A stable identifier generated when using a read-only API key. This can be used in place of an exact UUID to get individual checks.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unique_key: Option<String>,
}

/// Represents an integration, like email or sms.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Channel {
    pub id: String,
    pub name: String,
    pub kind: String,
}

/// Represents a new check that is initialized locally then created on healthchecks.io
/// using the admin API. It contains a lot less fields than the [Check](struct.Check.html)
/// struct so we implement it separately.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct NewCheck {
    /// Name of the check.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Space separated list of tags set on this check.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<String>,
    /// Description of the check.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desc: Option<String>,
    /// Expected period of the check, in seconds. Is None when no timeout is set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<i32>,
    /// Grace period in minutes before the check is considered as failed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grace: Option<i32>,
    /// A cron expression defining this check's schedule. Is None when no schedule is configured.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule: Option<String>,
    /// The timezone for the server which pings for this check's status. Can be None by itself or when no schedule is configured.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tz: Option<String>,
    /// Indicates if the ping has been manually paused and will not resume automatically
    /// on a new ping. These checks need to manually be resumed from the web dashboard.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manual_resume: Option<String>,
    /// Comma-separated list of IDs of the integration channels associated with this check. Is None when no integrations
    /// are configured.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channels: Option<String>,
    /// List of fields that must be unique before the check is created.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unique: Option<Vec<String>>,
}

impl Default for NewCheck {
    fn default() -> Self {
        NewCheck {
            name: None,
            tags: None,
            channels: None,
            desc: None,
            timeout: None,
            grace: None,
            schedule: None,
            tz: None,
            unique: None,
            manual_resume: None,
        }
    }
}

/// All fields in this struct are optional and every non-None value signifies that
/// we want the server to replace the existing value with the one we're sending.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdatedCheck {
    /// Name of the check.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Space separated list of tags set on this check.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<String>,
    /// Description of the check.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desc: Option<String>,
    /// Expected period of the check, in seconds. Is None when no timeout is set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<i32>,
    /// Grace period in minutes before the check is considered as failed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grace: Option<i32>,
    /// A cron expression defining this check's schedule. Is None when no schedule is configured.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule: Option<String>,
    /// The timezone for the server which pings for this check's status. Can be None by itself or when no schedule is configured.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tz: Option<String>,
    /// Indicates if the ping has been manually paused and will not resume automatically
    /// on a new ping. These checks need to manually be resumed from the web dashboard.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manual_resume: Option<String>,
    /// Comma-separated list of IDs of the integration channels associated with this check. Is None when no integrations
    /// are configured.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channels: Option<String>,
}

impl Default for UpdatedCheck {
    fn default() -> Self {
        UpdatedCheck {
            name: None,
            tags: None,
            desc: None,
            timeout: None,
            grace: None,
            schedule: None,
            tz: None,
            manual_resume: None,
            channels: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::to_value;

    #[test]
    fn new_check_does_not_serialize_nulls() {
        let new_check: NewCheck = Default::default();
        let value = to_value(new_check);
        assert!(value.is_ok());
        assert!(value.unwrap().to_string().eq("{}"));
    }

    #[test]
    fn updated_check_does_not_serialize_nulls() {
        let updated_check: UpdatedCheck = Default::default();
        let value = to_value(updated_check);
        assert!(value.is_ok());
        assert!(value.unwrap().to_string().eq("{}"));
    }

    #[test]
    fn default_impl_for_updated_check_fills_nulls() {
        let updated_check = UpdatedCheck {
            name: Some("Updated check".to_string()),
            ..Default::default()
        };
        let value = to_value(updated_check);
        assert!(value.is_ok());
        assert_eq!(value.unwrap().to_string(), "{\"name\":\"Updated check\"}");
    }
}
