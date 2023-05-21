use std::collections::HashMap;
use std::sync::Arc;
use maplit::hashmap;
use crate::api::notification_request::{Channel, NotificationRequest};
use crate::OutlookClient;
use crate::ports::rest::customer::CustomerRestClient;
use crate::ports::rest::seller::SellerRestClient;
use super::email_notification_channel::EmailNotificationChannel;
use crate::config::properties::Properties;
use crate::model::error::Error;
use crate::model::NotificationStatus;


pub struct NotificationService {
    pub(crate) notification_channels: HashMap<Channel, Box<dyn NotificationChannel>>,
}

impl NotificationService {
    pub fn new() -> Self {
        NotificationService { notification_channels: build_channel_configs() }
    }
    pub(crate) fn send_notification(&self, notification: NotificationRequest) -> Result<NotificationStatus, Error>{
        self.notification_channels
            .get(&notification.channel)
            .unwrap()
            .send(&notification)
    }
}

impl Default for NotificationService {
    fn default() -> Self {
        NotificationService::new()
    }
}

pub trait NotificationChannel {
    fn get_channel(&self) -> Channel;
    fn send(&self, notification: &NotificationRequest) -> Result<NotificationStatus, Error>;
}


fn build_channel_configs() -> HashMap<Channel, Box<dyn NotificationChannel>> {
    let email_channel: Box<dyn NotificationChannel> = build_email_channel(Properties::new());

    return hashmap! {
        Channel::Email => email_channel,
    };
}

fn build_email_channel(properties: Properties) -> Box<dyn NotificationChannel> {
    Box::new(EmailNotificationChannel {
        smtp_client: Box::new(OutlookClient::default()),
        customer_repository: Arc::new(CustomerRestClient::new(properties.get("CUSTOMERS_SERVICE_URL"))),
        seller_repository: Arc::new(SellerRestClient::new(properties.get("SELLERS_SERVICE_URL"))),
    })
}

