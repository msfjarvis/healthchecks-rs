use serde_derive::{Deserialize, Serialize};

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

    /// When a check is late, how long to wait until an alert is sent.
    pub grace: i64,

    /// Number of times the check has pinged healthchecks.io.
    pub n_pings: i64,

    /// Current status of the check.
    pub status: String,

    #[cfg(feature = "v2")]
    pub started: bool,

    #[cfg(feature = "v3")]
    pub slug: Option<String>,

    /// UTC timestamp of the last known ping.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_ping: Option<String>,

    /// UTC timestamp of the next expected ping based on grace period. Will be
    /// [`None`] in case no grace period is set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_ping: Option<String>,

    /// Indicates if the ping has been manually paused and will not resume automatically
    /// on a new ping. These checks need to manually be resumed from the web dashboard.
    pub manual_resume: bool,

    /// A GET request to this URL is a valid ping for this check. This field is [`None`] if a read-only API token is used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ping_url: Option<String>,

    /// URL to GET this specific check or POST an updated version. This field is [`None`] if a read-only API token is used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_url: Option<String>,

    /// URL to pause monitoring for this check. The next ping will resume monitoring.
    /// This field is [`None`] if a read-only API token is used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pause_url: Option<String>,

    /// Comma-separated list of IDs of the integration channels associated with this check. Is [`None`] when no integrations
    /// are configured.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channels: Option<String>,

    /// Expected time between pings, in seconds. Is [`None`] when no timeout is set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<i64>,

    /// A cron expression defining this check's schedule. Is [`None`] when no schedule is configured.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule: Option<String>,

    /// The timezone for the server which pings for this check's status. Can be [`None`] by itself or when no schedule is configured.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tz: Option<String>,

    /// A stable identifier generated when using a read-only API key. This can be used in place of an exact UUID to get individual checks.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unique_key: Option<String>,
}

impl Check {
    /// Get the unique identifier of a [`Check`].
    #[must_use]
    pub fn id(&self) -> Option<String> {
        if let Some(ref url) = self.ping_url {
            url.split('/').last().map(std::borrow::ToOwned::to_owned)
        } else {
            None
        }
    }
}

/// Represents an integration, like email or sms.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Channel {
    /// UUID for this integration.
    pub id: String,
    /// "Pretty name" for the integration, typically shown in the healthchecks web UI.
    pub name: String,
    /// Type of integration, such as "telegram" or "email".
    pub kind: String,
}

/// Represents a new check that is initialized locally then created on healthchecks.io
/// using the admin API. It skips over many fields from the [`Check`]
/// struct that are generated server-side by healthchecks.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
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

    /// Expected time between pings, in seconds. Is [`None`] when no timeout is set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<i32>,

    /// When a check is late, how long to wait until an alert is sent. Value in minutes, is [`None`] when no timeout is set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grace: Option<i32>,

    /// A cron expression defining this check's schedule. Is [`None`] when no schedule is configured.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule: Option<String>,

    /// The timezone for the server which pings for this check's status. Can be [`None`] by itself or when no schedule is configured.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tz: Option<String>,

    /// Indicates if the ping has been manually paused and will not resume automatically
    /// on a new ping. These checks need to manually be resumed from the web dashboard.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manual_resume: Option<String>,

    /// Comma-separated list of IDs of the integration channels associated with this check. Is [`None`] when no integrations
    /// are configured.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channels: Option<String>,

    /// List of fields that must be unique before the check is created.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unique: Option<Vec<String>>,
}

/// Represents an existing check which needs some or all of its values updated
/// on the healthchecks server. Fields that do not need updates should be set to
/// [`None`](std::option::Option::None) and will be skipped from serialization.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
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

    /// Expected time between pings, in seconds. Is [`None`] when no timeout is set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<i32>,

    /// When a check is late, how long to wait until an alert is sent. Value in minutes, is [`None`] when no timeout is set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grace: Option<i32>,

    /// A cron expression defining this check's schedule. Is [`None`] when no schedule is configured.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule: Option<String>,

    /// The timezone for the server which pings for this check's status. Can be [`None`] by itself or when no schedule is configured.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tz: Option<String>,

    /// Indicates if the ping has been manually paused and will not resume automatically
    /// on a new ping. These checks need to manually be resumed from the web dashboard.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manual_resume: Option<String>,

    /// Comma-separated list of IDs of the integration channels associated with this check. Is [`None`] when no integrations
    /// are configured.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channels: Option<String>,
}

/// Represents a ping that a check has received.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Ping {
    /// Type of ping: one of 'start', 'success' or 'failure'. 'start' indicates the beginning
    /// of a timer that is counted server-side until the next 'success' or 'failure' ping.
    #[serde(rename = "type")]
    pub type_field: String,

    /// RFC3339-style timestamp for when this ping occured.
    pub date: String,

    /// Index of the ping. The healthchecks.io dashboard keeps track of all pings and assigns
    /// them each an index that is sequentially incremented.
    pub n: i64,

    /// The method used to make this ping. As far as I can tell, is one of 'http' or 'https'.
    /// I do not use email for my pings and am not aware if it affects the scheme field, let
    /// me know if you are.
    pub scheme: String,

    /// The IP address of the remote machine that initiated the ping.
    pub remote_addr: String,

    /// The HTTP method used to initiate this ping. Might differ for email pings, again, let me know
    /// if you know.
    pub method: String,

    /// User-Agent header for the network request that created this ping.
    pub ua: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Duration for which this ping ran, Will be [None] for untimed pings.
    pub duration: Option<f64>,
}

/// Represents a "flip" in state this check has experienced. This event
/// is generated when a check transitions between the up and down state.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Flip {
    /// RFC3339 timestamp for when the change occured
    pub timestamp: String,

    /// 1 or 0 depending on whether or not the 'flip' changed the check's status to 'up' or 'down' respectively.
    pub up: i64,
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
        assert_eq!(value.unwrap().to_string(), "{}");
    }

    #[test]
    fn updated_check_does_not_serialize_nulls() {
        let updated_check: UpdatedCheck = Default::default();
        let value = to_value(updated_check);
        assert!(value.is_ok());
        assert_eq!(value.unwrap().to_string(), "{}");
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
