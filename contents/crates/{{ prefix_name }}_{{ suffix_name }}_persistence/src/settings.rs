use serde::{Deserialize, Serialize};

const DEFAULT_DATABASE_URL: &str = "mysql://dev:dev@localhost/{{ prefix_name }}_{{ suffix_name }}";

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PersistenceSettings {
    pub url: String,
    pub max_connections: Option<u32>,
    pub run_migrations: Option<bool>,
}

impl Default for PersistenceSettings {
    fn default() -> Self {
        Self {
            url: DEFAULT_DATABASE_URL.to_string(),
            max_connections: None,
            run_migrations: Some(true),
        }
    }
}
