extern crate clap;
extern crate healthchecks;

use healthchecks::create_config;
use healthchecks::create_config_with_user_agent;
use std::env::var;
use std::io;
use std::io::Write;
use std::process::Command;

use clap::{crate_version, App, AppSettings, Arg};

enum ExitCode {
    SUCCESS,
    FAILURE,
}

fn main() {
    let app = App::new("monitor")
        .version(crate_version!())
        .usage("monitor [FLAGS/OPTIONS] -X <command>")
        .setting(AppSettings::ColoredHelp)
        .setting(AppSettings::DeriveDisplayOrder)
        .arg(
            Arg::with_name("command")
                .short("X")
                .long("exec")
                .min_values(1)
                .required(true)
                .allow_hyphen_values(true)
                .value_terminator(";")
                .value_name("cmd")
                .help("Command to execute and monitor"),
        )
        .arg(
            Arg::with_name("token")
                .long("token")
                .takes_value(true)
                .help("Healthchecks.io UUID to ping after executing the task"),
        )
        .arg(
            Arg::with_name("timer")
                .long("timer")
                .short("t")
                .takes_value(false)
                .help("Starts a timer before running the command"),
        )
        .arg(
            Arg::with_name("user_agent")
                .short("u")
                .long("user_agent")
                .takes_value(true)
                .help("Custom User-Agent header to uniquely identify the caller in healthchecks.io logs"),
        );
    let matches = app.get_matches();
    let cmds = matches
        .value_of("command")
        .expect("command must be passed")
        .split(' ')
        .collect::<Vec<&str>>();
    let token = if let Some(token) = matches.value_of("token") {
        String::from(token)
    } else {
        var("HEALTHCHECKS_TOKEN")
            .expect("Either passing --token or specifying HEALTHCHECKS_TOKEN is necessary!")
    };
    let user_agent = if let Some(ua) = matches.value_of("user_agent") {
        String::from(ua)
    } else {
        var("HEALTHCHECKS_USERAGENT").unwrap_or_else(|_| String::from(""))
    };
    let config = if user_agent.is_empty() {
        create_config(token)
    } else {
        create_config_with_user_agent(token, user_agent)
    };
    if matches.is_present("timer") {
        config.start_timer();
    }
    match exec(cmds) {
        ExitCode::SUCCESS => config.report_success(),
        ExitCode::FAILURE => config.report_failure(),
    };
}

fn exec(cmds: Vec<&str>) -> ExitCode {
    let mut cmd = Command::new(cmds[0]);
    for arg in &cmds[1..] {
        cmd.arg(arg);
    }
    let output = cmd.output();
    match output {
        Ok(output) => {
            let stdout = io::stdout();
            let stderr = io::stderr();

            let _ = stdout.lock().write_all(&output.stdout);
            let _ = stderr.lock().write_all(&output.stderr);
            ExitCode::SUCCESS
        }
        Err(ref why) => {
            if why.kind() == io::ErrorKind::NotFound {
                eprintln!("Command not found: {:?}", cmd);
            }
            ExitCode::FAILURE
        }
    }
}
