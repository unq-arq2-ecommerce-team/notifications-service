use rocket::tokio;
use serde::de::DeserializeOwned;

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

    pub fn get<T>(&self, endpoint: &str) -> Result<T, String>
        where
            T: DeserializeOwned,
    {
        let url = self.build_url(endpoint);
        println!("getting from url: {}", url);

        tokio::task::block_in_place(|| {
            let result = match reqwest::blocking::get(url) {
                Ok(response) => match response.json::<T>() {
                    Ok(ok) => Ok(ok),
                    Err(err) => Err(format!("Failed to decode response JSON: {}", err)),
                },
                Err(err) => Err(format!("Request failed: {}", err)),
            };
            result
        })
    }
}