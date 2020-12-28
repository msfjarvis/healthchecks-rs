use anyhow::{anyhow, Context};
use clap::{crate_authors, crate_description, crate_name, crate_version, AppSettings, Clap};
use healthchecks::ping::get_client;
use std::env::var;
use subprocess::{Exec, Redirection};

const HEALTHCHECKS_CHECK_ID_VAR: &str = "HEALTHCHECKS_CHECK_ID";
/// This is useful to have a good-looking default in the clap generated help.
const FAKE_EMPTY_STRING: &str = "\"\"";

#[derive(Debug)]
struct Settings {
    check_id: String,
    ua: Option<String>,
}

#[derive(Clap)]
#[clap(
    name = crate_name!(),
    version = crate_version!(),
    author = crate_authors!(),
    about = crate_description!(),
    setting = AppSettings::ColoredHelp,
    setting = AppSettings::DeriveDisplayOrder,
)]
struct Opts {
    /// command to execute and monitor
    #[clap(short = 'X', long = "exec")]
    command: Vec<String>,
    /// starts a timer before running the command
    #[clap(short = 't', long = "timer")]
    timer: bool,
    /// saves the execution logs with the failure ping to allow debugging on healthchecks.io
    #[clap(short = 'l', long = "logs")]
    save_logs: bool,
    /// user agent to be logged at healthchecks.io
    #[clap(
        short = 'u',
        long = "user-agent",
        required = false,
        default_value = FAKE_EMPTY_STRING,
    )]
    user_agent: String,
}

fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();
    let ua = if opts.user_agent == FAKE_EMPTY_STRING {
        match var("HEALTHCHECKS_USERAGENT") {
            Ok(f) => Some(f),
            Err(_) => None,
        }
    } else {
        Some(opts.user_agent)
    };
    let settings = Settings {
        check_id: if let Ok(token) = var(HEALTHCHECKS_CHECK_ID_VAR) {
            token
        } else {
            return Err(anyhow!(
                "{} must be set to run monitor",
                HEALTHCHECKS_CHECK_ID_VAR
            ));
        },
        ua,
    };
    let mut client = get_client(&settings.check_id)?;
    if let Some(user_agent) = settings.ua {
        client = client.set_user_agent(&user_agent)
    }
    if opts.timer {
        client.start_timer();
    }
    let cmd = opts.command.join(" ");
    if opts.save_logs {
        let capture_data = Exec::shell(&cmd)
            .stdout(Redirection::Pipe)
            .stderr(Redirection::Merge)
            .capture()
            .context(format!("Failed to execute {}", cmd))?;
        if capture_data.success() {
            client.report_success();
        } else {
            client.report_failure_with_logs(&capture_data.stdout_str());
        }
    } else {
        let exit_status = Exec::shell(&cmd)
            .join()
            .context(format!("Failed to execute {}", cmd))?;
        if exit_status.success() {
            client.report_success();
        } else {
            client.report_failure();
        }
    }
    Ok(())
}
