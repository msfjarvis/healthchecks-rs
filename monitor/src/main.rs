mod cli;

use clap::Clap;
use color_eyre::{eyre::eyre, eyre::WrapErr, Result};
use healthchecks::ping::get_client;
use std::env::var;
use subprocess::{Exec, Redirection};

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
    if opts.save_logs {
        let capture_data = Exec::shell(&cmd)
            .stdout(Redirection::Pipe)
            .stderr(Redirection::Merge)
            .capture()
            .context(format!("Failed to execute {}", cmd))?;
        if capture_data.success() {
            client.report_success();
        } else {
            let stdout = capture_data.stdout_str();
            client.report_failure_with_logs(&stdout);
            return Err(eyre!(
                "Failed to run '{}', stdout: {}",
                opts.command.join(" "),
                stdout
            ));
        }
    } else {
        let exit_status = Exec::shell(&cmd)
            .join()
            .context(format!("Failed to execute {}", cmd))?;
        if exit_status.success() {
            client.report_success();
        } else {
            client.report_failure();
            return Err(eyre!("Failed to run '{}'", opts.command.join(" ")));
        }
    }
    Ok(())
}
