# healthchecks-rs

A simple Rust library that allows pinging [healthchecks.io](https://healthchecks.io/) to indicate success or failure of a task.

## Usage

Usage is super simple!

```rust
extern crate healthchecks;

use healthchecks::config::create_config;

fn main() {
    let config = create_config("my-uuid-that-is-definitely-not-real");
    config.report_success();
    config.report_failure();
}
```

## Licensing

Dual licensed under Apache 2.0 or MIT at your option.
