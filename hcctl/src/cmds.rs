use color_eyre::{eyre::eyre, Result};
use comfy_table::modifiers::UTF8_ROUND_CORNERS;
use comfy_table::presets::UTF8_FULL;
use comfy_table::{ContentArrangement, Table};
use healthchecks::{manage, model::Check};
use time::{format_description::well_known::Rfc3339, OffsetDateTime, UtcOffset};
use uuid::Uuid;

use crate::cli::Settings;
use healthchecks::manage::ManageClient;
use healthchecks::model::Ping;

pub(crate) fn pings(settings: Settings, check_id: &str) -> Result<()> {
    let client = manage::get_client(settings.token, settings.ua)?;
    let pings = match Uuid::parse_str(check_id) {
        Ok(_) => client.list_logged_pings(check_id)?,
        Err(_) => search_pings(&client, check_id)?,
    };

    print_pings(pings)
}

pub(crate) fn list(settings: Settings) -> Result<()> {
    let client = manage::get_client(settings.token, settings.ua)?;
    print_checks(client.get_checks()?)
}

pub(crate) fn search(settings: Settings, search_term: &str) -> Result<()> {
    let client = manage::get_client(settings.token, settings.ua)?;

    match search_checks(&client, search_term) {
        Ok(checks) => print_checks(checks),
        Err(error) => Err(error),
    }
}

fn search_pings(client: &ManageClient, search_term: &str) -> Result<Vec<Ping>> {
    let pings: Vec<Ping> = search_checks(client, search_term)?
        .iter()
        .filter_map(|check| client.list_logged_pings(check.id()?.as_str()).ok())
        .flatten()
        .collect();

    if pings.is_empty() {
        Err(eyre!("No pings matched search term '{}'", search_term))
    } else {
        Ok(pings)
    }
}

fn search_checks(client: &ManageClient, search_term: &str) -> Result<Vec<Check>> {
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
    table
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_header(vec!["Number", "Time", "Type", "Duration"]);
    pings.truncate(10);
    for ping in pings {
        let utc_time = OffsetDateTime::parse(&ping.date, &Rfc3339)?.to_offset(UtcOffset::UTC);
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
            format!("{duration:.3} sec")
        } else {
            "".to_owned()
        };
        table.add_row(vec![
            format!("#{}", ping.n),
            time_str,
            ping.type_field,
            duration_str,
        ]);
    }

    println!("{table}");
    Ok(())
}

fn print_checks(checks: Vec<Check>) -> Result<()> {
    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_header(vec!["ID", "Name", "Last Ping"]);
    let now = OffsetDateTime::now_utc();
    for check in checks {
        let date = if let Some(ref date_str) = check.last_ping {
            human_readable_duration(now, date_str)?
        } else {
            "-".to_owned()
        };
        let id = check.id().unwrap_or_else(|| "-".to_owned());
        table.add_row(vec![id, check.name, date]);
    }

    println!("{table}");
    Ok(())
}

fn human_readable_duration(now: OffsetDateTime, date_str: &str) -> Result<String> {
    let date = OffsetDateTime::parse(date_str, &Rfc3339)?;
    let duration = now - date;
    let hours = duration.whole_hours();
    let minutes = if hours == 0 {
        duration.whole_minutes()
    } else {
        duration.whole_minutes() % hours
    };
    Ok(format!("{hours} hour(s) and {minutes} minute(s) ago"))
}

#[cfg(test)]
mod tests {
    use time::macros::datetime;

    use crate::cmds::human_readable_duration;

    #[test]
    fn duration_parses_correctly() {
        let now = datetime!(2021-01-26 19:38:00 UTC);
        let duration = human_readable_duration(now, "2021-01-26T14:00:24+00:00").unwrap();
        assert_eq!(duration, "5 hour(s) and 2 minute(s) ago");
    }

    #[test]
    fn duration_parses_correctly_with_only_minutes() {
        let now = datetime!(2021-01-26 14:38:00 UTC);
        let duration = human_readable_duration(now, "2021-01-26T14:00:24+00:00").unwrap();
        assert_eq!(duration, "0 hour(s) and 37 minute(s) ago");
    }
}
