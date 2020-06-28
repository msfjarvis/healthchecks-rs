# healthchecks-rs

A simple Rust library that allows pinging [healthchecks.io](https://healthchecks.io/) to indicate success or failure of a task.

## Usage

Usage is super simple!

```rust
use healthchecks::config::create_config;

fn main() {
    let config = create_config("my-uuid-that-is-definitely-not-real", None);
    config.report_failure();
    config.report_success();
}
```

Or if you want to set a custom user agent for filtering purposes (default is `healthcheck-rs/$library_version`)

```rust
use healthchecks::config::create_config;

fn main() {
    let config = create_config_with_user_agent("my-uuid-that-is-definitely-not-real", Some(String::from("very-fancy-useragent")));
    config.report_failure();
    config.report_success();
}

```

You can also start a timer to record durations on [healthchecks.io](https://healthchecks.io/).

```rust
use healthchecks::config::create_config;

fn main() {
    let config = create_config("my-uuid-that-is-definitely-not-real", None);
    config.start_timer();
    do_long_running_task();
    config.report_success();
}

```

## Licensing

Dual licensed under Apache 2.0 or MIT at your option.
