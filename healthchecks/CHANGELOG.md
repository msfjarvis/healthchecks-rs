# Changelog

All notable changes to this project will be documented in this file.

## [Unreleased]

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
