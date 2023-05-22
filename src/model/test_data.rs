use crate::model::email::smtp::Email;
use crate::model::error::{Error, msg};
use crate::model::user::customer::{Customer, CustomerRepository};
use crate::model::user::seller::{Seller, SellerRepository};
use crate::SmtpClient;

pub fn create_test_customer(id: i32) -> Customer {
    Customer {
        id,
        firstname: "first_name".to_string(),
        lastname: "last_name".to_string(),
        email: "email".to_string(),
    }
}

pub fn create_test_seller(id: i32) -> Seller {
    Seller {
        id,
        name: "first_name".to_string(),
        email: "email".to_string(),
    }
}

impl SmtpClient for MockSmtpClient {
    fn send(&self, email: Email) -> Result<String, String> {
        Ok("ok".to_string())
    }
}

pub struct MockSmtpClient {}

impl CustomerRepository for MockCustomerRepository {
    fn find_by_id(&self, id: i32) -> Result<Customer, Error> {
        match id {
            1 => Ok(create_test_customer(1)),
            _ => Err(msg("error".to_string()))
        }
    }
}

pub struct MockCustomerRepository {}

impl SellerRepository for MockSellerRepository {
    fn find_by_id(&self, id: i32) -> Result<Seller, Error> {
        Err(msg("error".to_string()))
    }
}

pub struct MockSellerRepository {}