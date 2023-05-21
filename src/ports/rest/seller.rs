use crate::model::error::Error;
use crate::model::user::seller::{Seller, SellerRepository};
use crate::ports::rest::rest_client::RestClient;

pub struct SellerRestClient {
    rest_client: RestClient,
}

impl SellerRepository for SellerRestClient {
    fn find_by_id(&self, id: i32) -> Result<Seller, Error> {
        let endpoint = format!("seller/{}", id);
        self.rest_client.get(&endpoint)
    }
}


impl SellerRestClient {
    pub fn new(base_url: String) -> Self {
        SellerRestClient { rest_client: RestClient::new(base_url) }
    }
}