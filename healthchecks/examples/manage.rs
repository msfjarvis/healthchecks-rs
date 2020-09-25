use healthchecks::manage::create_config;
use healthchecks::model::NewCheck;

fn main() -> anyhow::Result<()> {
    let api_key = std::env::args()
        .nth(1)
        .expect("Providing an API key as the first parameter is required");
    let config = create_config(api_key, None).unwrap();
    for check in config.get_checks()? {
        println!("{:?}", check);
    }
    let new_check = NewCheck::new().build();
    println!("{:?}", config.create_check(new_check)?);
    Ok(())
}
