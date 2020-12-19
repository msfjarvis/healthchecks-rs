use anyhow::{anyhow, Context};
use clap::{crate_authors, crate_description, crate_name, crate_version, AppSettings, Clap};
use healthchecks::ping::get_config;
use std::env::var;
use subprocess::Exec;

const HEALTHCHECKS_CHECK_ID_VAR: &str = "HEALTHCHECKS_CHECK_ID";

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
}

fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();
    let ua = match var("HEALTHCHECKS_USERAGENT") {
        Ok(f) => Some(f),
        Err(_) => None,
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
    let mut config = get_config(&settings.check_id)?;
    if let Some(user_agent) = settings.ua {
        config = config.set_user_agent(&user_agent)
    }
    if opts.timer {
        config.start_timer();
    }
    let cmd = opts.command.join(" ");
    let exit_status = Exec::shell(&cmd)
        .join()
        .context(format!("Failed to execute {}", cmd))?;
    if exit_status.success() {
        config.report_success();
    } else {
        config.report_failure();
    }
    Ok(())
}
