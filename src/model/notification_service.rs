use std::collections::HashMap;
use std::sync::Arc;
use lettre::SmtpTransport;
use lettre::transport::smtp::authentication::Credentials;
use maplit::hashmap;
use crate::api::notification_request::{Channel, NotificationRequest};
use crate::OutlookClient;
use super::email_notification_channel::EmailNotificationChannel;


pub struct NotificationService {
    pub(crate) notification_channels: HashMap<Channel, Box<dyn NotificationChannel>>,
}

impl NotificationService {

    pub(crate) fn send_notification(&self, notification: NotificationRequest) {

        self.notification_channels
            .get(&notification.channel)
            .unwrap()
            .send(&notification);

        println!("Notification sent");
    }

    pub fn new() -> Self {
        NotificationService { notification_channels: build_channel_configs() }
    }
}

impl Default for NotificationService {
    fn default() -> Self {
        NotificationService::new()
    }
}

pub trait NotificationChannel {
    fn get_channel(&self) -> Channel;
    fn send(&self, notification: &NotificationRequest);
}


fn build_channel_configs() -> HashMap<Channel, Box<dyn NotificationChannel>> {
    let email_channel: Box<dyn NotificationChannel> = Box::new(EmailNotificationChannel {
        smtp_client: Box::new(OutlookClient::default()),
    });

    return hashmap! {
        Channel::Email => email_channel,
    };
}


