use crate::model::user::customer::{Customer, CustomerRepository};

pub struct CustomerRestClient {
}

impl CustomerRepository for CustomerRestClient {
    fn find_by_id(&self, id: i32) -> Result<Customer, String> {
        Ok(Customer {
            id,
            firstname: "".to_string(),
            lastname: "".to_string(),
            email: "".to_string()
        })
    }
}

impl CustomerRestClient {
    pub fn new() -> Self {
        CustomerRestClient {}
    }
}