# Changelog

All notable changes to this project will be documented in this file.

## Unreleased

## [3.0.7]

### Changed

- Upgrade to `healthchecks` 3.1.7

## [3.0.6]

### Changed

- Upgrade to `healthchecks` 3.1.6

## [3.0.5]

### Changed

- Upgrade to `healthchecks` 3.1.5

## [3.0.4]

### Changed

- Upgrade to `healthchecks` 3.1.4

## [3.0.3]

### Changed

- Revert back to old logging behaviour from before hcctl 3.0.2
- Upgrade to `clap` 4.0

## [3.0.2]

### Changed

- Upgrade to `clap` 3.2
- Rework retries to combine logs of each run in the final failure ping

## [3.0.1]

### Changed

- Upgrade to healthchecks 3.1.0

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
[3.0.0]: https://github.com/msfjarvis/healthchecks-rs/releases/tag/monitor-3.0.0
[3.0.1]: https://github.com/msfjarvis/healthchecks-rs/releases/tag/monitor-3.0.1
[3.0.2]: https://github.com/msfjarvis/healthchecks-rs/releases/tag/healthchecks-monitor-3.0.2
[3.0.3]: https://github.com/msfjarvis/healthchecks-rs/releases/tag/healthchecks-monitor-3.0.3
[3.0.4]: https://github.com/msfjarvis/healthchecks-rs/releases/tag/healthchecks-monitor-3.0.4
[3.0.5]: https://github.com/msfjarvis/healthchecks-rs/releases/tag/healthchecks-monitor-3.0.5
[3.0.6]: https://github.com/msfjarvis/healthchecks-rs/releases/tag/healthchecks-monitor-3.0.6
[3.0.7]: https://github.com/msfjarvis/healthchecks-rs/releases/tag/healthchecks-monitor-3.0.7
