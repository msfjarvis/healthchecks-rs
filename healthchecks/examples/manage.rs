use healthchecks::manage::create_config;

fn main() -> anyhow::Result<()> {
    let api_key = std::env::args()
        .nth(1)
        .expect("Providing an API key as the first parameter is required");
    let config = create_config(api_key, None);
    for check in config.get_checks()? {
        println!("{:?}", check);
    }
    Ok(())
}
