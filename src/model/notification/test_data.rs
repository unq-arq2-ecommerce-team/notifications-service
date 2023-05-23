use std::sync::Arc;
use maplit::hashmap;
use crate::api::notification_request::{Channel, Event, EventName, NotificationRequest, Recipient, RecipientType};
use crate::model::email::smtp::Email;
use crate::model::error::{Error, msg};
use crate::model::notification::email_notification_channel::EmailNotificationChannel;
use crate::model::notification::notification_service::{NotificationChannel, NotificationService};
use crate::model::notification::{notification_sent, NotificationStatus};
use crate::model::user::customer::{Customer, CustomerRepository};
use crate::model::user::seller::{Seller, SellerRepository};
use crate::SmtpClient;

pub trait Mocked {
    fn mocked() -> Self;
}

impl Mocked for NotificationService {
    fn mocked() -> Self {
        NotificationService {
            notification_channels: {
                let mock_channel: Box<dyn NotificationChannel> = Box::new(MockEmailNotificationChannel {});
                hashmap! { Channel::Email => mock_channel }
            },
        }
    }
}

struct MockEmailNotificationChannel {}
impl NotificationChannel for MockEmailNotificationChannel {
    fn get_channel(&self) -> Channel {
        Channel::Email
    }

    fn send(&self, _: &NotificationRequest) -> Result<NotificationStatus, Error> {
        Ok(notification_sent())
    }
}

impl Mocked for EmailNotificationChannel {
    fn mocked() -> Self {
        EmailNotificationChannel {
            smtp_client: Box::new(MockSmtpClient {}),
            customer_repository: Arc::new(MockCustomerRepository {}),
            seller_repository: Arc::new(MockSellerRepository {}),
        }
    }
}

pub fn create_failing_request() -> NotificationRequest {
    NotificationRequest {
        recipient: Recipient {
            id: 2, // id 2 will cause find_by_id to fail
            recipient_type: RecipientType::Customer,
        },
        event: Event {
            name: EventName::PurchaseSuccessful,
            detail: "detail".to_string(),
        },
        channel: Channel::Email,
    }
}

pub fn create_invalid_channel_request() -> NotificationRequest {
    NotificationRequest {
        recipient: Recipient {
            id: 2,
            recipient_type: RecipientType::Customer,
        },
        event: Event {
            name: EventName::PurchaseSuccessful,
            detail: "detail".to_string(),
        },
        channel: Channel::Whatsapp,
    }
}

pub fn create_succeed_request(recipient: RecipientType, event_name: EventName, event_detail: String) -> NotificationRequest {
    NotificationRequest {
        recipient: Recipient {
            id: 1, // id 1 will cause find_by_id to succeed
            recipient_type: recipient,
        },
        event: Event {
            name: event_name,
            detail: event_detail.to_string(),
        },
        channel: Channel::Email,
    }
}

pub fn create_test_customer(id: i32) -> Customer {
    Customer {
        id,
        firstname: "first_name".to_string(),
        lastname: "last_name".to_string(),
        email: "an_email@some.com".to_string(),
    }
}

pub fn create_test_seller(id: i32) -> Seller {
    Seller {
        id,
        name: "first_name".to_string(),
        email: "email".to_string(),
    }
}

impl SmtpClient for MockSmtpClient {
    fn send(&self, _: Email) -> Result<String, String> {
        Ok("ok".to_string())
    }
}

pub struct MockSmtpClient {}

impl CustomerRepository for MockCustomerRepository {
    fn find_by_id(&self, id: i32) -> Result<Customer, Error> {
        match id {
            1 => Ok(create_test_customer(1)),
            _ => Err(msg("error".to_string()))
        }
    }
}

pub struct MockCustomerRepository {}

impl SellerRepository for MockSellerRepository {
    fn find_by_id(&self, id: i32) -> Result<Seller, Error> {
        match id {
            1 => Ok(create_test_seller(1)),
            _ => Err(msg("error".to_string()))
        }
    }
}

pub struct MockSellerRepository {}