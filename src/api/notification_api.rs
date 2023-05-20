use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};
use rocket::State;
use crate::api::notification_request::NotificationRequest;
use crate::NotificationService;


#[post("/notification", data = "<notification>")]
pub fn notification(notification: Json<NotificationRequest>) -> String {
    let notification_req= notification.into_inner();

    get_notification_service().send_notification(notification_req);

    "done".to_string()
}

fn get_notification_service() -> NotificationService {
    NotificationService::default()
}
