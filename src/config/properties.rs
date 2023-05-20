
use std::collections::HashMap;
use std::env;

pub struct Properties {
    properties: HashMap<String, String>,
}

struct PropKeyValue {
    key: String,
    value: String,
}

impl Properties {
    pub fn new() -> Self {
        let tuples =  [
            get_env_or_default("SMTP_HOST", "smtp.office365.com"),
            get_env_or_default("SMTP_PORT", "587"),
            get_env_or_default("SMTP_USERNAME", "arq-soft2-unq@outlook.com"),
            get_env_or_default("SMTP_PASSWORD", ""),
            get_env_or_default("SMTP_FROM", "ArqSoft2-TP <arq-soft2-unq@outlook.com>"),
            get_env_or_default("SMTP_TLS", "true"),
        ];

        let props = tuples.into_iter().collect();
        Properties { properties: props }
    }

    pub fn get(&self, key: &str) -> String {
        self.properties.get(key).unwrap().to_string()
    }
}

fn get_env_or_default(key: &str, default: &str) -> (String, String) {
    let value = env::var(key).unwrap_or(default.to_string());
    (key.to_string(), value)
}