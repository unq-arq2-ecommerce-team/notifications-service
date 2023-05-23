pub mod notification_service;
pub mod email_notification_channel;
pub mod test_data;

#[derive(serde::Serialize, Debug)]
pub struct NotificationStatus {
    pub status: String,
}

pub fn notification_sent() -> NotificationStatus {
    NotificationStatus {
        status: "sent".to_string(),
    }
}