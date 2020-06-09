extern crate healthchecks;

use healthchecks::create_config_with_user_agent;

fn main() {
    let uuid = std::env::args()
        .nth(1)
        .expect("Providing a UUID as first parameter is mandatory");
    let config = create_config_with_user_agent(uuid, String::from("healthchecks-rs/simple_client"));
    config.start_timer();
    std::thread::sleep(std::time::Duration::from_millis(10_000));
    config.report_success();
}
