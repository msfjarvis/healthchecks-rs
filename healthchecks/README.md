# healthchecks-rs

A simple Rust library that allows pinging [healthchecks.io](https://healthchecks.io/) to indicate success or failure of a task.

## Supported API methods

### Pinging API

- [x] Signal success
- [x] Signal failure
- [x] Signal start

### Management API

- [x] Get a list of all checks
- [x] Get a single check
- [x] Create a new check
- [x] Update an existing check
- [x] Pause monitoring of a check
- [x] Delete check
- [x] Get a list of check's logged pings
- [x] Get a list of check's status changes
- [x] Get a list of existing integrations

## Usage (pinging API)

```rust
use healthchecks::config::get_config;

fn main() {
    let config = get_config("my-uuid-that-is-definitely-not-real");
    config.report_failure();
    config.report_success();
}
```

If you want to set a custom user agent for filtering purposes (default is `healthcheck-rs/$library_version`)

```rust
use healthchecks::config::get_config;

fn main() {
    let config = get_config("my-uuid-that-is-definitely-not-real").set_user_agent("very-fancy-useragent");
    config.report_failure();
    config.report_success();
}

```

You can also start a timer to record durations on [healthchecks.io](https://healthchecks.io/).

```rust
use healthchecks::config::get_config;

fn main() {
    let config = get_config("my-uuid-that-is-definitely-not-real");
    config.start_timer();
    do_long_running_task();
    config.report_success();
}

```

## Licensing

Dual licensed under Apache 2.0 or MIT at your option.
