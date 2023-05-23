use std::collections::HashMap;
use std::env;


pub const IS_INTEGRATION_TEST_ENV: &str = "IS_INTEGRATION_TEST_ENV";
pub const SMTP_HOST: &str = "SMTP_HOST";
pub const SMTP_PORT: &str = "SMTP_PORT";
pub const SMTP_USERNAME: &str = "SMTP_USERNAME";
pub const SMTP_PASSWORD: &str = "SMTP_PASSWORD";
pub const SMTP_FROM: &str = "SMTP_FROM";
pub const SELLERS_SERVICE_URL: &str = "SELLERS_SERVICE_URL";
pub const CUSTOMERS_SERVICE_URL: &str = "CUSTOMERS_SERVICE_URL";

pub struct Properties {
    properties: HashMap<String, String>,
}

impl Properties {
    pub fn new() -> Self {
        let tuples = [
            get_env_or_default(IS_INTEGRATION_TEST_ENV, "false"),
            get_env_or_default(SMTP_HOST, "smtp.office365.com"),
            get_env_or_default(SMTP_PORT, "587"),
            get_env_or_default(SMTP_USERNAME, "arq-soft2-unq@outlook.com"),
            get_env_or_default(SMTP_PASSWORD, ""),
            get_env_or_default(SMTP_FROM, "ArqSoft2-TP <arq-soft2-unq@outlook.com>"),
            get_env_or_default(SELLERS_SERVICE_URL, "http://localhost:8081/api/v1"),
            get_env_or_default(CUSTOMERS_SERVICE_URL, "http://localhost:8081/api/v1"),
        ];

        let props = tuples.into_iter().collect();
        Properties { properties: props }
    }

    pub fn get(&self, key: &str) -> String {
        self.properties.get(key).unwrap().to_string()
    }
    pub fn get_bool(&self, key: &str) -> bool {
        self.properties.get(key).unwrap().to_string() == "true"
    }
}

fn get_env_or_default(key: &str, default: &str) -> (String, String) {
    let value = env::var(key).unwrap_or(default.to_string());
    (key.to_string(), value)
}