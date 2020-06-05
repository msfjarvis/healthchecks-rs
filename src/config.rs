use uuid::Uuid;

pub struct HealthcheckConfig {
    pub uuid: String,
}

pub fn create_config(uuid: &str) -> HealthcheckConfig {
    if let Ok(_) = Uuid::parse_str(uuid) {
        HealthcheckConfig {
            uuid: uuid.to_string(),
        }
    } else {
        panic!("Invalid UUID: {}", uuid)
    }
}
