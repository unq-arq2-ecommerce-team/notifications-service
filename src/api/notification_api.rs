use rocket::serde::json::Json;
use crate::api::error::ApiError;
use crate::api::notification_request::NotificationRequest;
use crate::model::notification::notification_service::NotificationService;
use crate::model::notification::NotificationStatus;
use crate::model::use_cases::SendNotification;

#[utoipa::path(
    request_body = NotificationRequest,
    responses(
        (status = 200,
            description = "Notification sent successfully",
            body = NotificationStatus),
        (status = 400,
            description = "Bad request",
            body = ApiError)
    )
)]
#[post("/notification", data = "<notification>")]
pub fn notification(notification: Json<NotificationRequest>) -> Result<Json<NotificationStatus>, ApiError> {
    let notification_req= notification.into_inner();

    match notify_use_case().send_notification(notification_req) {
        Ok(notification_status) => Ok(Json(notification_status)),
        Err(err) => Err(ApiError::from(err)),
    }
}

fn notify_use_case() -> Box<dyn SendNotification> {
    Box::new(NotificationService::default())
}

