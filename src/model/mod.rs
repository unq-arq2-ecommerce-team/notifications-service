pub mod notification_service;
pub mod email_notification_channel;
pub mod email;
pub mod user;
pub mod error;
mod test_data;

// define struct for NotificationStatus

#[derive(serde::Serialize)]
pub struct NotificationStatus {
    pub status: String,
}

pub fn notification_sent() -> NotificationStatus {
    NotificationStatus {
        status: "sent".to_string(),
    }
}