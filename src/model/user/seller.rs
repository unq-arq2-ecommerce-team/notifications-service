use rocket::serde::{Deserialize, Serialize};
use crate::model::error::Error;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Seller {
    pub id: i32,
    pub name: String,
    pub email: String,
}

pub trait SellerRepository {
    fn find_by_id(&self, id: i32) -> Result<Seller, Error>;
}