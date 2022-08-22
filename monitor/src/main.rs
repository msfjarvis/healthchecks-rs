mod cli;
mod exec;

use crate::{cli::Opts, exec::run_with_retry};
use clap::Parser;
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
    let opts = Opts::parse();
    let ua = if opts.has_user_agent() {
        match var("HEALTHCHECKS_USERAGENT") {
            Ok(f) => Some(f),
            Err(_) => None,
        }
    } else {
        Some(opts.user_agent)
    };
    let settings = Settings {
        check_id: var(HEALTHCHECKS_CHECK_ID_VAR)?,
        ua,
    };
    let mut client = get_client(&settings.check_id)?;
    if let Some(user_agent) = settings.ua {
        client = client.set_user_agent(&user_agent);
    }
    if opts.timer && !client.start_timer() {
        eprintln!("Failed to start timer");
    }
    let cmd = opts.command.join(" ");
    let command_result = run_with_retry(&cmd, opts.retry_count, opts.save_logs);
    match command_result {
        Ok(_) => {
            if !client.report_success() {
                eprintln!("Failed to report success");
            }
        }
        Err(logs) => {
            if let Some(log) = logs {
                if !client.report_failure_with_logs(&log) {
                    eprintln!("Failed to report failure");
                }
                return Err(eyre!("Failed to run '{}', stdout: {}", &cmd, &log));
            }
            if !client.report_failure() {
                eprintln!("Failed to report failure");
            };
            return Err(eyre!("Failed to run '{}'", &cmd));
        }
    }
    Ok(())
}
