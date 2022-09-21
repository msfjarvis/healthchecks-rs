use clap::Parser;

#[derive(Debug)]
pub(crate) struct Settings {
    pub(crate) token: String,
    pub(crate) ua: Option<String>,
}

#[derive(Parser)]
#[command(author, version, about)]
pub(crate) struct Opts {
    #[command(subcommand)]
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

#[cfg(test)]
mod test {
    use super::Opts;

    #[test]
    fn cli_assert() {
        <Opts as clap::CommandFactory>::command().debug_assert();
    }
}
