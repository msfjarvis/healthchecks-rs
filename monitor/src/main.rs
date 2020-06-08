extern crate clap;
extern crate healthchecks;

use healthchecks::create_config;
use healthchecks::create_config_with_user_agent;
use std::env::var;
use std::io;
use std::io::Write;
use std::process::Command;

use clap::{load_yaml, App};

enum ExitCode {
    SUCCESS,
    FAILURE,
}

fn main() {
    let yaml = load_yaml!("monitor.yml");
    let matches = App::from_yaml(yaml).get_matches();
    let cmds = matches
        .value_of("command")
        .expect("command must be passed")
        .split(" ")
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
        var("HEALTHCHECKS_USERAGENT").unwrap_or(String::from(""))
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
