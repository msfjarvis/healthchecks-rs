use clap::{crate_version, App, AppSettings, Arg};
use execute::Execute;
use healthchecks::ping::create_config;
use std::env::var;
use std::process::Command;

#[derive(Debug)]
struct Config {
    token: String,
    ua: Option<String>,
}

fn main() -> anyhow::Result<()> {
    let ua = match var("HEALTHCHECKS_USERAGENT") {
        Ok(f) => Some(f),
        Err(_) => None,
    };
    let config = Config {
        token: var("HEALTHCHECKS_TOKEN").expect("HEALTHCHECKS_TOKEN must be set to run monitor"),
        ua,
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
    let mut command = Command::new(&cmds.get(0).expect("Should have at least one command"));
    for cmd in cmds.iter().skip(1) {
        command.arg(cmd);
    }
    if let Some(exit_code) = command.execute_output()?.status.code() {
        if exit_code == 0 {
            config.report_success();
        } else {
            config.report_failure();
        }
    } else {
        eprintln!("Interrupted!");
    };
    Ok(())
}
