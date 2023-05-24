pub mod notification_service;
pub mod email_notification_channel;
pub mod test_data;
use utoipa::ToSchema;

#[derive(serde::Serialize, Debug, ToSchema)]
pub struct NotificationStatus {
    pub status: String,
}

pub fn notification_sent() -> NotificationStatus {
    NotificationStatus {
        status: "sent".to_string(),
    }
}