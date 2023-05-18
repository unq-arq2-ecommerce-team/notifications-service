
use std::collections::HashMap;
use std::env;

pub struct Properties {
    properties: HashMap<String, String>,
}

impl Properties {
    pub fn new() -> Self {
        let mut properties = HashMap::new();

        properties.insert("SMTP_HOST".to_string(), env::var("SMTP_HOST").unwrap_or("smtp.office365.com".to_string()));
        properties.insert("SMTP_PORT".to_string(), env::var("SMTP_PORT").unwrap_or("587".to_string()));
        properties.insert("SMTP_USERNAME".to_string(), env::var("SMTP_USERNAME").unwrap_or("".to_string()));
        properties.insert("SMTP_PASSWORD".to_string(), env::var("SMTP_PASSWORD").unwrap_or("".to_string()));
        properties.insert("SMTP_FROM".to_string(), env::var("SMTP_FROM").unwrap_or("".to_string()));
        properties.insert("SMTP_TLS".to_string(), env::var("SMTP_TLS").unwrap_or("true".to_string()));

        Properties { properties }
    }

    pub fn get(&self, key: &str) -> String {
        self.properties.get(key).unwrap().to_string()
    }
}