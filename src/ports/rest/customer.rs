use crate::model::user::customer::{Customer, CustomerRepository};
use crate::ports::rest::rest_client::RestClient;

pub struct CustomerRestClient {
    rest_client: RestClient,
}

impl CustomerRepository for CustomerRestClient {
    fn find_by_id(&self, id: i32) -> Result<Customer, String> {
        let endpoint = format!("customer/{}", id);
        self.rest_client.get(&endpoint)
    }
}


impl CustomerRestClient {
    pub fn new(base_url: String) -> Self {
        CustomerRestClient { rest_client: RestClient::new(base_url) }
    }
}