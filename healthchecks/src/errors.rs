use thiserror::Error;

/// Errors raised by API operations
#[derive(Debug, Error)]
pub enum HealthchecksApiError {
    /// Server rejected this API key
    #[error("invalid API key")]
    InvalidApiKey,
    /// The server rejected this API key from a write operation, so it is possible
    /// this is a read-only key
    #[error("invalid API key, make sure you're not using a read-only key")]
    PossibleReadOnlyKey,
    /// Access denied
    #[error("access denied")]
    AccessDenied,
    /// No check found for the given check ID
    #[error("no check found with id: {0}")]
    NoCheckFound(String),
    /// Creating a new check failed because an existing check matched the data
    #[error("an existing check was matched based on the \"unique\" parameter")]
    ExistingCheckMatched,
    /// The request body was invalid. This shouldn't ever be hit by users, but this is just in case
    #[error("the request is not well-formed, violates schema, or uses invalid field values")]
    NotWellFormed,
    /// The check limit for the account was reached
    #[error("the account's check limit has been reached")]
    CheckLimitReached,
    /// Unexpected error in network operation
    #[error("unexpected transport error: {0:?}")]
    TransportError(Box<ureq::Transport>),
    /// Unexpected error from API, please file an issue if you ever run into this
    #[error("unexpected error: {0}")]
    UnexpectedError(String),
    /// Unexpected IO errors, please file an issue if you ever run into this
    #[error("unexpected IO error")]
    Io {
        #[from]
        source: std::io::Error,
    },
    /// Unexpected JSON parsing errors, please file an issue if you ever run into this
    #[error("unexpected error while (de)serializing JSON response")]
    Json {
        #[from]
        source: serde_json::Error,
    },
}

/// Errors raised by invalid configuration when creating an instance of [`ManageClient`](crate::manage::ManageClient) or [`PingClient`](crate::ping::PingClient)
#[derive(Debug, Error)]
pub enum HealthchecksConfigError {
    /// Empty API key
    #[error("API key must not be empty")]
    EmptyApiKey,
    /// Empty User Agent
    #[error("User Agent must not be empty")]
    EmptyUserAgent,
    /// Empty API url
    #[error("API url must not be empty")]
    EmptyApiUrl,
    /// Invalid UUID
    #[error("invalid UUID: {0}")]
    InvalidUuid(String),
}
