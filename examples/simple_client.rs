extern crate healthchecks;

use healthchecks::create_config;

fn main() {
    let config = create_config("my-uuid-that-is-definitely-not-real");
    config.start_timer();
    std::thread::sleep(std::time::Duration::from_millis(10_000));
    config.report_success();
}
