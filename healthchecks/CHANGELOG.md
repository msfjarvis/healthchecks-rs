# Changelog

All notable changes to this project will be documented in this file.

## [Unreleased]

### Added

- Support specifying run IDs when pinging healthchecks.io (closes [#64])

## [3.1.7]

### Changed

- Update `serde`

## [3.1.6]

### Changed

- Update `serde`

## [3.1.5]

### Added

- Support [v3 management API] behind the `v3` feature flag. `v3` automatically selects `v2`.

## [3.1.4]

### Added

- Support [v2 management API] behind the `v2` feature flag

## [3.1.3]

### Changed

- Raise MSRV to 1.64.0

## [3.1.2]

### Changed

- Set minimum required version of `thiserror` to `v1.0.2` to fix builds with `-Z minimal-versions`

## [3.1.1]

### Changed

- Bump `ureq` version requirement to `~2.5`

## [3.1.0]

### Changed

- Bump `uuid` dependency to `1.1.x`

## [3.0.6]

### Changed

- Declare MSRV as `1.58.0`. This was always the case, but it is now conveyed through the `rust-version` field.
- Add an `# Errors` section to the rustdoc of all `Result`-returning public methods

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
[3.0.6]: https://github.com/msfjarvis/healthchecks-rs/releases/tag/healthchecks-v3.0.6
[3.1.0]: https://github.com/msfjarvis/healthchecks-rs/releases/tag/healthchecks-v3.1.0
[3.1.1]: https://github.com/msfjarvis/healthchecks-rs/releases/tag/healthchecks-v3.1.1
[3.1.2]: https://github.com/msfjarvis/healthchecks-rs/releases/tag/healthchecks-v3.1.2
[3.1.3]: https://github.com/msfjarvis/healthchecks-rs/releases/tag/healthchecks-v3.1.3
[3.1.4]: https://github.com/msfjarvis/healthchecks-rs/releases/tag/healthchecks-v3.1.4
[3.1.5]: https://github.com/msfjarvis/healthchecks-rs/releases/tag/healthchecks-v3.1.5
[3.1.6]: https://github.com/msfjarvis/healthchecks-rs/releases/tag/healthchecks-v3.1.6
[3.1.7]: https://github.com/msfjarvis/healthchecks-rs/releases/tag/healthchecks-v3.1.7


[#22]: https://github.com/msfjarvis/healthchecks-rs/pull/22
[#27]: https://github.com/msfjarvis/healthchecks-rs/pull/27
[#64]: https://github.com/msfjarvis/healthchecks-rs/issues/64
[v2 management api]: https://healthchecks.io/docs/api/
[v3 management api]: https://healthchecks.io/docs/api/
