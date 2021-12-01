use clap::{crate_authors, crate_description, crate_name, crate_version, AppSettings, Parser};

#[derive(Debug)]
pub(crate) struct Settings {
    pub(crate) token: String,
    pub(crate) ua: Option<String>,
}

#[derive(Parser)]
#[clap(
    name = crate_name!(),
    version = crate_version!(),
    author = crate_authors!(),
    about = crate_description!(),
    setting = AppSettings::DeriveDisplayOrder,
    setting = AppSettings::SubcommandRequiredElseHelp,
)]
pub(crate) struct Opts {
    #[clap(subcommand)]
    pub(crate) subcommand: SubCommand,
}

#[derive(Parser)]
pub(crate) enum SubCommand {
    List(List),
    Pings(Pings),
    Search(Search),
}

/// Lists the checks in your account with their last ping
#[derive(Parser)]
pub(crate) struct List {}

/// Get the last 10 pings for the given check ID
#[derive(Parser)]
pub(crate) struct Pings {
    /// ID of the check whose pings are being fetched
    pub(crate) check_id: String,
}

/// Search for checks and show their latest pings
#[derive(Parser)]
pub(crate) struct Search {
    /// Search term to find in the list of all pings
    pub(crate) search_term: String,
}