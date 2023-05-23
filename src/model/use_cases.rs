use crate::api::notification_request::NotificationRequest;
use crate::model::error::Error;
use crate::model::notification::NotificationStatus;

pub trait SendNotification {
    fn send_notification(&self, notification: NotificationRequest) -> Result<NotificationStatus, Error>;
}