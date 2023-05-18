use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};

#[get("/ping")]
pub fn ping() -> &'static str {
    "pong"
}