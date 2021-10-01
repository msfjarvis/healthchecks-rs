mod cli;
mod cmds;

use std::env::var;

use clap::Clap;
use cli::{Opts, Settings, SubCommand};
use color_eyre::{eyre::eyre, Result};

const HEALTHCHECKS_TOKEN_VAR: &str = "HEALTHCHECKS_TOKEN";
const HEALTHCHECKS_USERAGENT: &str = "HEALTHCHECKS_USERAGENT";

fn main() -> Result<()> {
    color_eyre::install()?;
    let opts = Opts::parse();

    let ua = match var(HEALTHCHECKS_USERAGENT) {
        Ok(f) => Some(f),
        Err(_) => None,
    };
    let settings = Settings {
        token: if let Ok(token) = var(HEALTHCHECKS_TOKEN_VAR) {
            token
        } else {
            return Err(eyre!("{} must be set to run hcctl", HEALTHCHECKS_TOKEN_VAR));
        },
        ua,
    };
    match opts.subcommand {
        SubCommand::List(_) => {
            cmds::list(settings)?;
        }
        SubCommand::Pings(p) => {
            cmds::pings(settings, &p.check_id)?;
        }
        SubCommand::Search(s) => {
            cmds::search(settings, s.search_term)?;
        }
    }

    Ok(())
}
