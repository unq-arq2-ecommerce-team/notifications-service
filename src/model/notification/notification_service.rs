use std::collections::HashMap;
use std::sync::Arc;
use maplit::hashmap;
use crate::api::notification_request::{Channel, NotificationRequest};
use crate::{OutlookClient, SmtpClient};
use crate::ports::rest::customer::CustomerRestClient;
use crate::ports::rest::seller::SellerRestClient;
use super::email_notification_channel::EmailNotificationChannel;
use crate::config::properties::{CUSTOMERS_SERVICE_URL, IS_INTEGRATION_TEST_ENV, Properties, SELLERS_SERVICE_URL};
use crate::model::error::{bad_input, Error};
use crate::model::notification::NotificationStatus;
use crate::model::use_cases::SendNotification;
use crate::ports::smtp::outlook_client::TestEnvironment;


pub struct NotificationService {
    pub(crate) notification_channels: HashMap<Channel, Box<dyn NotificationChannel>>,
}

impl SendNotification for NotificationService {
    fn send_notification(&self, notification: NotificationRequest) -> Result<NotificationStatus, Error> {
        self.notification_channels.get(&notification.channel)
            .ok_or(bad_input(format!("Channel not found: {}", notification.channel))) // Result<&Box<dyn NotificationChannel>, Error>
            .and_then(|channel| channel.send(&notification)) // Result<NotificationStatus, Error>
    }
}

impl NotificationService {
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
        smtp_client: build_smtp_client(&properties),
        customer_repository: Arc::new(CustomerRestClient::new(properties.get(CUSTOMERS_SERVICE_URL))),
        seller_repository: Arc::new(SellerRestClient::new(properties.get(SELLERS_SERVICE_URL))),
    })
}

fn build_smtp_client(properties: &Properties) -> Box<dyn SmtpClient> {
    if properties.get_bool(IS_INTEGRATION_TEST_ENV) {
        Box::new(OutlookClient::test_env())
    } else {
        Box::new(OutlookClient::default())
    }
}

#[cfg(test)]
mod tests {
    use crate::api::notification_request::{EventName, RecipientType};
    use crate::model::error::ErrorKind;
    use crate::model::notification::test_data::{create_invalid_channel_request, create_succeed_request, Mocked};
    use super::*;

    #[test]
    fn send_notification_returns_error_when_channel_not_configured() {
        let notification = create_invalid_channel_request();

        let notification_service = NotificationService::mocked();

        let result = notification_service.send_notification(notification);

        assert!(result.is_err());
        let (kind, _) = result.unwrap_err().split();
        assert_eq!(kind, ErrorKind::BadInput);
    }

    #[test]
    fn send_notification_returns_ok() {
        let notification = create_succeed_request(RecipientType::Customer,
                                                  EventName::PurchaseSuccessful, "detail".to_string());

        let notification_service = NotificationService::mocked();

        let result = notification_service.send_notification(notification);

        assert!(result.is_ok());
    }
}

