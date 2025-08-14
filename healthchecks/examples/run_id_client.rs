use healthchecks::ping::get_client;
use std::result::Result;
use uuid::Uuid;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let uuid = std::env::args()
        .nth(1)
        .expect("Providing a UUID as first parameter is mandatory");

    // Generate a new run ID (or you could use a deterministic method)
    let run_id = Uuid::new_v4();
    println!("Using run ID: {run_id}");

    // Get the client
    let config = get_client(&uuid)?;

    // Start the timer with the run ID
    assert!(
        config.start_timer_with_run_id(Some(&run_id)),
        "Failed to start timer"
    );

    // Simulate some work
    std::thread::sleep(std::time::Duration::from_millis(10_000));

    // Report success with the same run ID
    assert!(
        config.report_success_with_run_id(Some(&run_id)),
        "Failed to report success"
    );

    Ok(())
}
