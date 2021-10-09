use chrono::{
    prelude::{DateTime, Datelike, Timelike},
    Utc,
};
use color_eyre::{eyre::eyre, Result};
use healthchecks::{manage, model::Check};
use prettytable::{cell, format, row, Table};
use uuid::Uuid;

use crate::cli::Settings;
use healthchecks::manage::ManageClient;
use healthchecks::model::Ping;

pub(crate) fn pings(settings: Settings, check_id: &str) -> Result<()> {
    let client = manage::get_client(settings.token, settings.ua)?;
    let uuid_result = Uuid::parse_str(check_id);
    let pings = match uuid_result {
        Ok(_) => client.list_logged_pings(check_id)?,
        Err(_) => search_pings(client, check_id.to_string())?,
    };

    print_pings(pings)
}

pub(crate) fn list(settings: Settings) -> Result<()> {
    let client = manage::get_client(settings.token, settings.ua)?;
    let checks = client.get_checks()?;

    print_checks(checks)
}

pub(crate) fn search(settings: Settings, search_term: String) -> Result<()> {
    let client = manage::get_client(settings.token, settings.ua)?;
    let checks_result = search_checks(client, search_term);

    match checks_result {
        Ok(checks) => print_checks(checks),
        Err(error) => Err(error),
    }
}

fn search_pings(client: ManageClient, search_term: String) -> Result<Vec<Ping>> {
    let pings: Vec<Ping> = search_checks(client.clone(), search_term.to_owned())?
        .iter()
        .flat_map(|check| {
            if check.id().is_none() {
                return None;
            }
            let id = check.id().unwrap();
            return Some(client.list_logged_pings(id.as_str()));
        })
        .take_while(|result| result.is_ok())
        .map(|api_result| api_result.unwrap())
        .flatten()
        .collect();

    if pings.is_empty() {
        Err(eyre!("No pings matched search term '{}'", search_term))
    } else {
        Ok(pings)
    }
}

fn search_checks(client: ManageClient, search_term: String) -> Result<Vec<Check>> {
    let checks: Vec<Check> = client
        .get_checks()?
        .into_iter()
        .filter(|check| {
            check
                .name
                .to_lowercase()
                .contains(&search_term.to_lowercase())
        })
        .collect();
    if checks.is_empty() {
        Err(eyre!("No checks matched search term '{}'", search_term))
    } else {
        Ok(checks)
    }
}

fn print_pings(mut pings: Vec<Ping>) -> Result<()> {
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

fn print_checks(checks: Vec<Check>) -> Result<()> {
    let mut table = Table::new();
    table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);
    table.set_titles(prettytable::row!["ID", "Name", "Last Ping"]);

    let now = Utc::now();
    for check in checks {
        let date = if let Some(ref date_str) = check.last_ping {
            human_readable_duration(&now, date_str)?
        } else {
            "-".to_owned()
        };
        let id = check.id().unwrap_or_else(|| "-".to_owned());
        table.add_row(prettytable::row![id, check.name, date]);
    }

    table.printstd();

    Ok(())
}

fn human_readable_duration(now: &DateTime<Utc>, date_str: &str) -> Result<String> {
    let date = DateTime::parse_from_rfc3339(date_str)?;
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
        let duration = human_readable_duration(now, "2021-01-26T14:00:24+00:00").unwrap();
        assert_eq!(duration, "5 hour(s) and 2 minute(s) ago")
    }

    #[test]
    fn duration_parses_correctly_with_only_minutes() {
        let now = &Utc.ymd(2021, 1, 26).and_hms(14, 38, 0);
        let duration = human_readable_duration(now, "2021-01-26T14:00:24+00:00").unwrap();
        assert_eq!(duration, "0 hour(s) and 37 minute(s) ago")
    }
}
