#![doc = include_str!("../README.md")]
/// Error types for public API
pub mod errors;
/// Functions for interacting with the Healthchecks management API.
pub mod manage;
/// API response models for data returned from the various Healthchecks APIs.
pub mod model;
/// Functions for interacting with the Healthchecks pinging API.
pub mod ping;
/// The default User-Agent header value for the library
pub(crate) const DEFAULT_USER_AGENT: &str = concat!("healthchecks-rs", "/", env!("CARGO_PKG_VERSION"));
