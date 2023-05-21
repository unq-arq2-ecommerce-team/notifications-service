use rocket::serde::json::Json;
use crate::api::error::ApiError;
use crate::api::notification_request::NotificationRequest;
use crate::model::error::Error;
use crate::model::NotificationStatus;
use crate::NotificationService;


#[post("/notification", data = "<notification>")]
pub fn notification(notification: Json<NotificationRequest>) -> Result<Json<NotificationStatus>, ApiError> {
    let notification_req= notification.into_inner();

    match get_notification_service().send_notification(notification_req) {
        Ok(notification_status) => Ok(Json(notification_status)),
        Err(err) => Err(ApiError::from(err)),
    }
}

fn get_notification_service() -> NotificationService {
    NotificationService::default()
}
