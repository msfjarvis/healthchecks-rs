use healthchecks::errors::HealthchecksApiError;
use healthchecks::manage::get_client;
use std::result::Result;

fn main() -> Result<(), HealthchecksApiError> {
    let api_key = std::env::args()
        .nth(1)
        .expect("Providing an API key as the first parameter is required");
    let config = get_client(api_key, None).unwrap();
    for check in config.get_checks()? {
        println!("{:?}", check);
    }
    for channel in config.get_channels()? {
        println!("{:?}", channel);
    }
    Ok(())
}
