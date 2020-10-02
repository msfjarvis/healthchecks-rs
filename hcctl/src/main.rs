#[macro_use]
extern crate prettytable;

use anyhow::{anyhow, Result};
use healthchecks::manage;
use prettytable::{format, Table};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Opts {
    /// API token to use when interacting with the healthchecks API
    #[structopt(
        long = "api-token",
        global = true,
        env = "HEALTHCHECKS_API_TOKEN",
        // This is annoying but you can't set a global option without a default
        default_value = ""
    )]
    api_token: String,

    #[structopt(subcommand)]
    command: Command,
}

#[derive(Debug, StructOpt)]
enum Command {
    /// List the checks in your account
    List,
}

fn main() -> Result<()> {
    let opts = Opts::from_args();

    if opts.api_token.is_empty() {
        return Err(anyhow!("No API token given"));
    }

    match opts.command {
        Command::List => list(&opts.api_token)?,
    };

    Ok(())
}

fn list(api_token: &str) -> Result<()> {
    let api = manage::get_config(api_token.to_string(), None)?;
    let checks = api.get_checks()?;

    let mut table = Table::new();
    table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);
    table.set_titles(row!["Name", "Last Ping"]);

    for check in checks {
        table.add_row(row![check.name, check.last_ping.unwrap_or("-".to_string())]);
    }

    table.printstd();

    Ok(())
}
