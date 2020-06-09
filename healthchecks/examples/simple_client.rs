extern crate healthchecks;

use healthchecks::create_config;

fn main() {
    let uuid = std::env::args()
        .nth(1)
        .expect("Providing a UUID as first parameter is mandatory");
    let config = create_config(uuid);
    config.start_timer();
    std::thread::sleep(std::time::Duration::from_millis(10_000));
    config.report_success();
}
