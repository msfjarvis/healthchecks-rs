use anyhow::Context;
use clap::{crate_version, Clap};
use healthchecks::ping::get_config;
use std::env::var;
use std::process::Command;

#[derive(Debug)]
struct Settings {
    check_id: String,
    ua: Option<String>,
}

/// monitor runs the given command and reports execution result to https://healthchecks.io
#[derive(Clap)]
#[clap(version = crate_version!(), author = "Harsh Shandilya <me@msfjarvis.dev>")]
struct Opts {
    /// command to execute and monitor
    #[clap(short = 'X', long = "exec")]
    command: Vec<String>,
    /// starts a timer before running the command
    #[clap(short = 't', long = "timer")]
    timer: bool,
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
    let opts = Opts::parse();
    let commands: Vec<Vec<String>> = if opts.command.len() == 1 {
        opts.command
            .get(0)
            .expect("This definitely has one command")
            .split(';')
            .map(|c| {
                c.split(' ')
                    .filter(|x| !x.is_empty())
                    .map(|x| x.to_string())
                    .collect()
            })
            .collect()
    } else {
        vec![opts.command]
    };
    let mut config = get_config(&settings.check_id)?;
    if let Some(user_agent) = settings.ua {
        config = config.set_user_agent(&user_agent)
    }
    if opts.timer {
        config.start_timer();
    }
    for cmds in commands {
        let mut command = Command::new(&cmds.get(0).expect("Should have at least one command"));
        for cmd in cmds.iter().skip(1) {
            command.arg(cmd);
        }
        match command
            .status()
            .context(format!("Failed on command: {:?}", cmds.join(" ")))?
            .code()
        {
            Some(code) => {
                if code != 0 {
                    config.report_failure();
                }
            }
            None => {
                eprintln!("Interrupted!");
                config.report_failure();
            }
        };
    }
    config.report_success();
    Ok(())
}
