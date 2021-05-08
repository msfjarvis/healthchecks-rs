use subprocess::{Exec, Redirection};

pub(crate) fn run_command(
    command: &str,
    save_logs: bool,
) -> std::result::Result<(), Option<String>> {
    return if save_logs {
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
    };
}
