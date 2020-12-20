# Changelog

All notable changes to this project will be documented in this file.

## [Unreleased]

### Added

- Add `healthchecks::ping::report_failure_with_logs` to attach `&str` data as debug information..

## [2.0.0]

### Changed

- Removed all illogical inlining
- Add support for all management APIs
- Move pinging API to `crate::ping` namespace
- Switch to custom error types for improved error handling

## [1.0.1]

### Fixed

- Fix default user agent

## [1.0.0]

### Changed

- Made all public methods inline
- Removed `create_config_with_user_agent` and made `create_config` take an `Option<String>` parameter


## [0.1.0]

Initial release

[0.1.0]: https://github.com/msfjarvis/healthchecks-rs/releases/tag/v0.1.0
[1.0.0]: https://github.com/msfjarvis/healthchecks-rs
[1.0.1]: https://github.com/msfjarvis/healthchecks-rs
[2.0.0]: https://github.com/msfjarvis/healthchecks-rs/releases/tag/healthchecks-2.0.0
