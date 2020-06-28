# monitor

Simple binary that's designed to execute arbitrary tasks and notify a provided healthchecks.io check about their status.

## Usage

```plaintext
monitor 1.0.0

USAGE:
    monitor [FLAGS/OPTIONS] -X <command>

FLAGS:
    -t, --timer      Starts a timer before running the command
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -X, --exec <cmd>                 Command to execute and monitor
```

You need to set the environment variable `HEALTHCHECKS_TOKEN` with the UUID provided by [healthchecks.io](https://healthchecks.io). By default, `monitor` sets the User-Agent header to `healthchecks-rs/<version>`. To change this, set the `HEALTHCHECKS_USERAGENT` env variable.
