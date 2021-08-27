# Changelog

All notable changes to this project will be documented in this file.

## [Unreleased]

## [3.0.0]

### Added

- Add `-l/--logs` flag to attach execution logs to failure ping.
- Add `-u/--user-agent` back as an alternative for setting `HEALTHCHECKS_USER_AGENT`. When both are provided, `--user-agent` takes precedence.

## [2.0.0]

### Changed

- Improve help command
- Better error handling and reporting
- Require commands to be quoted when being passed

## [1.0.1]

### Fixed

- Properly specify when default user agent is requested

## [1.0.0]

Initial release

[1.0.0]: https://github.com/msfjarvis/healthchecks-rs
[1.0.1]: https://github.com/msfjarvis/healthchecks-rs
[2.0.0]: https://github.com/msfjarvis/healthchecks-rs/releases/tag/monitor-2.0.0
[2.0.0]: https://github.com/msfjarvis/healthchecks-rs/releases/tag/monitor-3.0.0
