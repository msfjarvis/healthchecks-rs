pub(crate) const fn default_user_agent() -> &'static str {
    concat!("healthchecks-rs/", env!("CARGO_PKG_VERSION"))
}
