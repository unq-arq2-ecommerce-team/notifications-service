use crate::model::user::seller::{Seller, SellerRepository};
use reqwest::blocking::Client;
use rocket::serde::json::serde_json;
use rocket::tokio;
use crate::ports::rest::rest_client::RestClient;

pub struct SellerRestClient {
    base_url: String,
}

impl SellerRepository for SellerRestClient {
    fn find_by_id(&self, id: i32) -> Result<Seller, String> {
        // tokio::task::block_in_place(|| {
        //     let result = match reqwest::blocking::get(self.build_url(id)) {
        //         Ok(response) => match response.json::<Seller>() {
        //             Ok(seller) => Ok(seller),
        //             Err(err) => Err(format!("Failed to decode response JSON: {}", err)),
        //         },
        //         Err(err) => Err(format!("Request failed: {}", err)),
        //     };
        //     result
        // })

        let client = RestClient::new("https://17189a67-b759-40af-b333-7229f3564402.mock.pstmn.io");
        client.get("users/1")
    }
}


impl SellerRestClient {
    pub fn new() -> Self {
        SellerRestClient { base_url: "https://17189a67-b759-40af-b333-7229f3564402.mock.pstmn.io/".to_string() }
    }

    // fn build_url(&self, id: i32) -> String {
    //     format!("{}users/{}", self.base_url, id)
    // }
}