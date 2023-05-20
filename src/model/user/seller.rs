use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Seller {
    pub id: i32,
    pub name: String,
    pub email: String,
}

pub trait SellerRepository {
    fn find_by_id(&self, id: i32) -> Result<Seller, String>;
}