extern crate healthchecks;

use healthchecks::create_config;

fn main() {
    let config = create_config("my-uuid-that-is-definitely-not-real");
    config.report_success();
    config.report_failure();
}
