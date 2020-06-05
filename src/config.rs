pub struct HealthcheckConfig {
    pub uuid: String,
}

pub fn create_config(uuid: &str) -> HealthcheckConfig {
    HealthcheckConfig {
        uuid: uuid.to_string(),
    }
}
