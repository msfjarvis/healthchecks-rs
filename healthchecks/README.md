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
use healthchecks::ping::get_client;

fn ping_api() {
    let config = get_client("073305d2-3582-4dd6-b6a3-425e88583ca2").unwrap();
    config.report_failure();
    config.report_success();
}
```

If you want to set a custom user agent for filtering purposes (default is `healthcheck-rs/$library_version`)

```rust
use healthchecks::ping::get_client;

fn custom_user_agent() {
    let config = get_client("073305d2-3582-4dd6-b6a3-425e88583ca2").unwrap().set_user_agent("very-fancy-useragent");
    config.report_failure();
    config.report_success();
}

```

You can also start a timer to record durations on [healthchecks.io](https://healthchecks.io/).

```rust
use healthchecks::ping::get_client;

fn do_long_running_task() {}

fn timer() {
    let config = get_client("073305d2-3582-4dd6-b6a3-425e88583ca2").unwrap();
    config.start_timer();
    do_long_running_task();
    config.report_success();
}

```

## Minimum supported Rust Version

healthchecks' MSRV is 1.82.0

## Licensing

Dual licensed under Apache 2.0 or MIT at your option.
