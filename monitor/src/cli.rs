use clap::{AppSettings, Parser};

/// This is useful to have a good-looking default in the clap generated help.
const FAKE_EMPTY_STRING: &str = "\"\"";

#[derive(Parser)]
#[clap(author, version, about)]
#[clap(global_setting(AppSettings::DeriveDisplayOrder))]
pub(crate) struct Opts {
    /// command to execute and monitor
    #[clap(short = 'X', long = "exec")]
    pub(crate) command: Vec<String>,
    /// starts a timer before running the command
    #[clap(short = 't', long = "timer")]
    pub(crate) timer: bool,
    /// saves the execution logs with the failure ping to allow debugging on healthchecks.io
    #[clap(short = 'l', long = "logs")]
    pub(crate) save_logs: bool,
    /// number of times to retry the command before logging a failure
    #[clap(short = 'r', long = "retries", default_value = "1", required = false)]
    pub(crate) retry_count: u8,
    /// user agent to be logged at healthchecks.io
    #[clap(
        short = 'u',
        long = "user-agent",
        required = false,
        default_value = FAKE_EMPTY_STRING,
    )]
    pub(crate) user_agent: String,
}

impl Opts {
    pub(crate) fn has_user_agent(self: &Opts) -> bool {
        self.user_agent == FAKE_EMPTY_STRING
    }
}
