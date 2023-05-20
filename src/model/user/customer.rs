use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Customer {
    pub id: i32,
    pub firstname: String,
    pub lastname: String,
    pub email: String,
}

pub trait CustomerRepository {
    fn find_by_id(&self, id: i32) -> Result<Customer, String>;
}