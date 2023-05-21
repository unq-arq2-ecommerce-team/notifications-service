#[macro_use] extern crate rocket;
#[macro_use] extern crate thiserror;

use crate::model::email::smtp::SmtpClient;
use crate::model::notification_service::{NotificationService};
use crate::ports::smtp::outlook_client::OutlookClient;

mod api;
mod routes;
mod model;
mod ports;
mod config;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![routes::ping])
        .mount("/", routes![api::notification_api::notification])
}

