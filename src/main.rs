#[macro_use] extern crate rocket;

use std::collections::HashMap;
use std::sync::Arc;
use lettre::SmtpTransport;
use lettre::transport::smtp::authentication::Credentials;
use maplit::hashmap;
use crate::api::notification_request::Channel;
use crate::model::email::smtp::SmtpClient;
use crate::model::email_notification_channel::EmailNotificationChannel;
use crate::model::notification_service::{NotificationChannel, NotificationService};
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

