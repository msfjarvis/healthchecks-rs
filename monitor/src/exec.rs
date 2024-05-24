use subprocess::{Exec, Redirection};

pub(crate) fn run_with_retry(
    command: &str,
    retries: u8,
    save_logs: bool,
) -> Result<(), Option<String>> {
    let mut logs: Option<String> = None;
    for _ in 0..retries {
        match run_command(command, save_logs) {
            Ok(()) => return Ok(()),
            Err(e) => logs = e,
        }
    }
    Err(logs)
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
