# healthchecks-rs [![Built with Garnix](https://img.shields.io/endpoint?url=https%3A%2F%2Fgarnix.io%2Fapi%2Fbadges%2Fmsfjarvis%2Fhealthchecks-rs%3Fbranch%3Dmain)](https://garnix.io)

Rust crates for working with [healthchecks.io]. The repository contains these crates:

- [healthchecks]: A library that provides a type-safe way to access to the [healthchecks.io] pinging and management APIs. Currently covers all methods, please file an issue if a new one is added.
- [healthchecks-monitor]: A CLI tool that uses [healthchecks] to interface with the pinging API.
- [hcctl]: Another CLI tool, which utilises a subset of the management API to let users list current checks and get their last 10 pings.

## Licensing

Dual licensed under Apache 2.0 or MIT at your option.

[healthchecks.io]: https://healthchecks.io
[healthchecks]: healthchecks
[healthchecks-monitor]: monitor
[hcctl]: hcctl
