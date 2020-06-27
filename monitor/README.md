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
        --token <token>              Healthchecks.io UUID to ping after executing the task
    -u, --user_agent <user_agent>    Custom User-Agent header to uniquely identify the caller in healthchecks.io logs
```
