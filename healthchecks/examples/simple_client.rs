use healthchecks::errors::HealthchecksConfigError;
use healthchecks::ping::get_config;
use std::result::Result;

fn main() -> Result<(), HealthchecksConfigError> {
    let uuid = std::env::args()
        .nth(1)
        .expect("Providing a UUID as first parameter is mandatory");
    let config = get_config(&uuid)?;
    config.start_timer();
    std::thread::sleep(std::time::Duration::from_millis(10_000));
    config.report_success();
    Ok(())
}
