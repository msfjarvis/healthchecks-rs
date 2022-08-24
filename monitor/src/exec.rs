use subprocess::{Exec, Redirection};

pub(crate) fn run_with_retry(
    command: &str,
    retries: u8,
    save_logs: bool,
) -> Result<(), Option<String>> {
    let mut logs = String::new();
    for _ in 0..retries {
        match run_command(command, save_logs) {
            Ok(_) => return Ok(()),
            Err(Some(e)) => logs.push_str(&e),
            Err(_) => {}
        }
    }
    Err(Some(logs))
}

fn run_command(command: &str, save_logs: bool) -> Result<(), Option<String>> {
    if save_logs {
        let capture_data = Exec::shell(command)
            .stdout(Redirection::Pipe)
            .stderr(Redirection::Merge)
            .capture()
            .expect("Shell creation must never fail");
        if capture_data.success() {
            Ok(())
        } else {
            let stdout = capture_data.stdout_str();
            Err(Some(stdout))
        }
    } else {
        let exit_status = Exec::shell(command)
            .join()
            .expect("Shell creation must never fail");
        if exit_status.success() {
            Ok(())
        } else {
            Err(None)
        }
    }
}
