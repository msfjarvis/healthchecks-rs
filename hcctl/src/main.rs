#[macro_use]
extern crate prettytable;

use std::env::var;

use chrono::{
    prelude::{DateTime, Datelike, Timelike},
    Utc,
};
use clap::{crate_authors, crate_description, crate_name, crate_version, AppSettings, Clap};
use color_eyre::{eyre::eyre, Result};
use prettytable::{format, Table};

use healthchecks::manage;

const HEALTHCHECKS_TOKEN_VAR: &str = "HEALTHCHECKS_TOKEN";

#[derive(Debug)]
struct Settings {
    token: String,
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
    setting = AppSettings::SubcommandRequiredElseHelp,
)]
struct Opts {
    #[clap(subcommand)]
    subcommand: SubCommand,
}

#[derive(Clap)]
enum SubCommand {
    List(List),
    Pings(Pings),
}

/// Lists the checks in your account with their last ping
#[derive(Clap)]
#[clap(setting = AppSettings::ColoredHelp)]
struct List {}

/// Get the last 10 pings for the given check ID
#[derive(Clap)]
#[clap(setting = AppSettings::ColoredHelp)]
struct Pings {
    /// ID of the check whose pings are being fetched
    check_id: String,
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let opts = Opts::parse();

    let ua = match var("HEALTHCHECKS_USERAGENT") {
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
            list(settings)?;
        }
        SubCommand::Pings(p) => {
            pings(settings, &p.check_id)?;
        }
    }

    Ok(())
}

fn pings(settings: Settings, check_id: &str) -> Result<()> {
    let client = manage::get_client(settings.token, settings.ua)?;
    let mut pings = client.list_logged_pings(check_id)?;
    let mut table = Table::new();
    table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);
    table.set_titles(row!["Number", "Time", "Type", "Duration"]);
    pings.truncate(10);
    for ping in pings {
        let utc_time = DateTime::parse_from_rfc3339(&ping.date)?.naive_utc();
        let date = utc_time.date();
        let time = utc_time.time();
        let time_str = format!(
            "{}/{} {}:{}",
            date.day(),
            date.month(),
            time.hour(),
            time.minute(),
        );
        let duration_str = if let Some(duration) = ping.duration {
            format!("{0:.3} sec", duration)
        } else {
            "".to_owned()
        };
        table.add_row(row![
            format!("#{}", ping.n),
            time_str,
            ping.type_field,
            duration_str
        ]);
    }
    table.printstd();
    Ok(())
}

fn list(settings: Settings) -> Result<()> {
    let client = manage::get_client(settings.token, settings.ua)?;
    let checks = client.get_checks()?;

    let mut table = Table::new();
    table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);
    table.set_titles(row!["ID", "Name", "Last Ping"]);

    let now = Utc::now();
    for check in checks {
        let date = if let Some(ref date_str) = check.last_ping {
            human_readable_duration(&now, date_str)?
        } else {
            "-".to_owned()
        };
        let id = check.id().unwrap_or_else(|| "-".to_owned());
        table.add_row(row![id, check.name, date]);
    }

    table.printstd();

    Ok(())
}

fn human_readable_duration(now: &DateTime<Utc>, date_str: &str) -> Result<String> {
    let date = DateTime::parse_from_rfc3339(&date_str)?;
    let duration = now.signed_duration_since(date);
    let hours = duration.num_hours();
    Ok(format!(
        "{} hour(s) and {} minute(s) ago",
        hours,
        if hours == 0 {
            duration.num_minutes()
        } else {
            duration.num_minutes() % hours
        }
    ))
}

#[cfg(test)]
mod tests {
    use chrono::TimeZone;

    use super::*;

    #[test]
    fn duration_parses_correctly() {
        let now = &Utc.ymd(2021, 1, 26).and_hms(19, 38, 0);
        let duration =
            human_readable_duration(now, &"2021-01-26T14:00:24+00:00".to_owned()).unwrap();
        assert!(duration == "5 hour(s) and 2 minute(s) ago")
    }

    #[test]
    fn duration_parses_correctly_with_only_minutes() {
        let now = &Utc.ymd(2021, 1, 26).and_hms(14, 38, 0);
        let duration =
            human_readable_duration(now, &"2021-01-26T14:00:24+00:00".to_owned()).unwrap();
        assert!(duration == "0 hour(s) and 37 minute(s) ago")
    }
}
