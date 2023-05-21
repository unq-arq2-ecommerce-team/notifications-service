use std::borrow::Borrow;
use reqwest::StatusCode;
use rocket::response::status;
use rocket::tokio;
use serde::de::DeserializeOwned;
use crate::model::error::{bad_input, Error, msg};

pub struct RestClient {
    base_url: String,
}

impl RestClient {
    pub fn new(base_url: String) -> Self {
        RestClient {
            base_url,
        }
    }

    pub fn build_url(&self, endpoint: &str) -> String {
        format!("{}/{}", self.base_url.to_string(), endpoint)
    }

    pub fn get<T>(&self, endpoint: &str) -> Result<T, Error>
        where
            T: DeserializeOwned,
    {
        let url = &self.build_url(endpoint);
        println!("getting from url: {}", url);

        tokio::task::block_in_place(|| {
            let result = match reqwest::blocking::get(url) {
                Ok(response) => {
                    if response.status().is_success() {
                        match response.json::<T>() {
                            Ok(ok) => Ok(ok),
                            Err(err) => Err(msg(format!("Failed to decode response JSON: {}", err))),
                        }
                    } else if response.status() == StatusCode::NOT_FOUND {
                        Err(bad_input(format!("Resource not found: {}", url.to_string())))
                    } else {
                        Err(msg(format!("Request failed: {}", response.status())))
                    }
                }
                Err(err) => Err(msg(format!("Request failed: {}", err))),
            };
            result
        })
    }
}