# monitor

Simple binary that's designed to execute arbitrary tasks and notify a provided healthchecks.io check about their status.

## Usage

```plaintext
monitor 0.1.0
Harsh Shandilya <me@msfjarvis.dev>
CLI tool that executes arbitrary commands and notifies healthchecks.io about their status

USAGE:
    monitor [FLAGS] [OPTIONS] --command <command>

FLAGS:
    -h, --help       Prints help information
    -t, --timer      Starts a timer before running the command
    -V, --version    Prints version information

OPTIONS:
    -c, --command <command>          Command to execute and monitor
        --token <token>              Healthchecks.io UUID to ping after executing the task
    -u, --user_agent <user_agent>    Custom User-Agent header to uniquely identify the caller in healthchecks.io logs
```
