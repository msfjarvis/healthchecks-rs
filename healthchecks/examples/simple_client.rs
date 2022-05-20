use healthchecks::ping::get_client;
use std::ops::Not;
use std::result::Result;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let uuid = std::env::args()
        .nth(1)
        .expect("Providing a UUID as first parameter is mandatory");
    let config = get_client(&uuid)?;
    assert!(!config.start_timer().not(), "Failed to start timer");
    std::thread::sleep(std::time::Duration::from_millis(10_000));
    assert!(!config.report_success().not(), "Failed to report success");
    Ok(())
}
