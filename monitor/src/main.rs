use clap::{App, AppSettings, Arg};
use healthchecks::ping::get_config;
use std::env::var;
use std::process::Command;
use anyhow::Context;

#[derive(Debug)]
struct Settings {
    check_id: String,
    ua: Option<String>,
}

fn main() -> anyhow::Result<()> {
    let ua = match var("HEALTHCHECKS_USERAGENT") {
        Ok(f) => Some(f),
        Err(_) => None,
    };
    let settings = Settings {
        check_id: var("HEALTHCHECKS_CHECK_ID")
            .expect("HEALTHCHECKS_CHECK_ID must be set to run monitor"),
        ua,
    };
    let app = App::new("monitor")
        .version(env!("CARGO_PKG_VERSION"))
        .about("monitor runs the given command and reports execution result to https://healthchecks.io")
        .setting(AppSettings::ColoredHelp)
        .setting(AppSettings::DeriveDisplayOrder)
        .arg(
            Arg::new("command")
                .long("exec")
                .short('X')
                .min_values(1)
                .allow_hyphen_values(true)
                .value_terminator(";")
                .value_name("cmd")
                .required(true)
                .about("Command to execute and monitor"),
        )
        .arg(
            Arg::new("timer")
                .long("timer")
                .short('t')
                .takes_value(false)
                .about("Starts a timer before running the command"),
        );
    let matches = app.get_matches();
    let cmds = matches
        .values_of("command")
        .expect("command must be passed")
        .collect::<Vec<&str>>();
    let commands: Vec<Vec<&str>> = if cmds.len() == 1 {
        cmds.get(0)
            .expect("This definitely has one command")
            .split(';')
            .map(|c| {
                c.split(' ')
                .filter(|x| !x.is_empty())
                .collect()
            })
            .collect()
    } else {
        vec![cmds]
    };
    let mut config = get_config(&settings.check_id)?;
    if let Some(user_agent) = settings.ua {
        config = config.set_user_agent(&user_agent)
    }
    if matches.is_present("timer") {
        config.start_timer();
    }
    for cmds in commands {
        let mut command = Command::new(&cmds.get(0).expect("Should have at least one command"));
        for cmd in cmds.iter().skip(1) {
            command.arg(cmd);
        }
        match command.status().context(format!("Failed on command: {:?}", cmds.join(" ")))?.code() {
            Some(code) => {
                if code != 0 {
                    config.report_failure();
                }
            },
            None => {
                eprintln!("Interrupted!");
                config.report_failure();
            }
        };
    }
    config.report_success();
    Ok(())
}
