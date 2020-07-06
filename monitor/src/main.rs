use clap::{crate_version, App, AppSettings, Arg};
use healthchecks::ping::create_config;
use pretty_exec_lib::pretty_exec::PrettyExec;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Config {
    #[serde(rename = "healthchecks_token")]
    token: String,
    #[serde(rename = "healthchecks_useragent")]
    ua: Option<String>,
}

fn main() -> anyhow::Result<()> {
    let config: Config = match envy::from_env() {
        Ok(conf) => conf,
        Err(error) => panic!(error),
    };
    let app = App::new("monitor")
        .version(crate_version!())
        .usage("monitor [FLAGS/OPTIONS] -X <command>")
        .setting(AppSettings::ColoredHelp)
        .setting(AppSettings::DeriveDisplayOrder)
        .arg(
            Arg::with_name("command")
                .long("exec")
                .short("X")
                .min_values(1)
                .allow_hyphen_values(true)
                .value_terminator(";")
                .value_name("cmd")
                .required(true)
                .help("Command to execute and monitor"),
        )
        .arg(
            Arg::with_name("timer")
                .long("timer")
                .short("t")
                .takes_value(false)
                .help("Starts a timer before running the command"),
        );
    let matches = app.get_matches();
    let cmds = matches
        .values_of("command")
        .expect("command must be passed")
        .collect::<Vec<&str>>();
    let config = create_config(config.token, config.ua)?;
    if matches.is_present("timer") {
        config.start_timer();
    }
    let mut exec = PrettyExec::new(&cmds.get(0).expect("Should have at least one command"));
    for cmd in cmds[1..cmds.len()].iter() {
        exec.arg(cmd);
    }
    match exec.spawn() {
        Ok(status) => {
            if status.success() {
                config.report_success()
            } else {
                config.report_failure()
            };
        }
        Err(err) => eprintln!("{}", err),
    };
    Ok(())
}
