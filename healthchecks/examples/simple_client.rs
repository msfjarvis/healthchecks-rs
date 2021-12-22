use healthchecks::ping::get_client;
use std::ops::Not;
use std::result::Result;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let uuid = std::env::args()
        .nth(1)
        .expect("Providing a UUID as first parameter is mandatory");
    let config = get_client(&uuid)?;
    if config.start_timer().not() {
        panic!("Failed to start timer");
    };
    std::thread::sleep(std::time::Duration::from_millis(10_000));
    if config.report_success().not() {
        panic!("Failed to report success");
    };
    Ok(())
}
