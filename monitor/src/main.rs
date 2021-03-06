mod cli;
mod exec;

use clap::Clap;
use color_eyre::{eyre::eyre, Result};
use healthchecks::ping::get_client;
use std::env::var;

const HEALTHCHECKS_CHECK_ID_VAR: &str = "HEALTHCHECKS_CHECK_ID";

#[derive(Debug)]
struct Settings {
    check_id: String,
    ua: Option<String>,
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let opts = cli::Opts::parse();
    let ua = if opts.has_user_agent() {
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
            return Err(eyre!(
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
    let mut tries = 1;
    let mut command_result = exec::run_command(&cmd, opts.save_logs);
    while tries < opts.retry_count && command_result.is_err() {
        command_result = exec::run_command(&cmd, opts.save_logs);
        tries += 1;
    }
    match command_result {
        Ok(_) => {
            client.report_success();
        }
        Err(logs) => match logs {
            Some(log) => {
                client.report_failure_with_logs(&log);
                return Err(eyre!("Failed to run '{}', stdout: {}", &cmd, &log));
            }
            None => {
                client.report_failure();
                return Err(eyre!("Failed to run '{}'", &cmd));
            }
        },
    }
    Ok(())
}
