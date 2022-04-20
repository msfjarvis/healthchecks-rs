# Changelog

All notable changes to this project will be documented in this file.

## [Unreleased]

## [3.0.5]

### Changed

- Bump `uuid` dependency to `1.0.0`

## [3.0.4]

### Changed

- Reword and improve documentation throughout the crate

## [3.0.3]

### Added

- Add support to return an existing check while creating if an existing one is found ([#27])

## [3.0.2]

### Added

- Add support for custom API URLs in ping module

## [3.0.1]

### Added

- Add support for custom API URLs in management module ([#22])

## [3.0.0]

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
[3.0.0]: https://github.com/msfjarvis/healthchecks-rs/releases/tag/healthchecks-3.0.0
[3.0.1]: https://github.com/msfjarvis/healthchecks-rs/releases/tag/healthchecks-v3.0.1
[3.0.2]: https://github.com/msfjarvis/healthchecks-rs/releases/tag/healthchecks-v3.0.2
[3.0.3]: https://github.com/msfjarvis/healthchecks-rs/releases/tag/healthchecks-v3.0.3
[3.0.4]: https://github.com/msfjarvis/healthchecks-rs/releases/tag/healthchecks-v3.0.4
[3.0.5]: https://github.com/msfjarvis/healthchecks-rs/releases/tag/healthchecks-v3.0.5

[#22]: https://github.com/msfjarvis/healthchecks-rs/pull/22
[#27]: https://github.com/msfjarvis/healthchecks-rs/pull/27
