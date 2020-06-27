extern crate clap;
extern crate healthchecks;
extern crate pretty_exec_lib;

use healthchecks::create_config;
use pretty_exec_lib::pretty_exec::PrettyExec;
use std::env::var;

use clap::{crate_version, App, AppSettings, Arg};

fn main() {
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
        .values_of("command")
        .expect("command must be passed")
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
    let config = create_config(
        token,
        if user_agent.is_empty() {
            None
        } else {
            Some(user_agent)
        },
    );
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
}
