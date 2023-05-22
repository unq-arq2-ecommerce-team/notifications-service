use std::sync::Arc;

use crate::api::notification_request::{Channel, EventName, NotificationRequest, RecipientType};
use crate::model::notification_service::NotificationChannel;
use crate::model::email::email_templates::{EmailTemplate, PaymentRejectedTemplate, PurchaseSuccessfulTemplate, SaleSuccessfulTemplate};
use crate::model::email::smtp::Email;
use crate::model::error::{Error, msg};
use crate::model::{notification_sent, NotificationStatus};
use crate::model::user::customer::CustomerRepository;
use crate::model::user::seller::SellerRepository;
use crate::SmtpClient;

pub struct EmailNotificationChannel {
    pub(crate) smtp_client: Box<dyn SmtpClient>,
    pub(crate) customer_repository: Arc<dyn CustomerRepository>,
    pub(crate) seller_repository: Arc<dyn SellerRepository>,
}

impl NotificationChannel for EmailNotificationChannel {
    fn get_channel(&self) -> Channel {
        Channel::Email
    }

    fn send(&self, notification: &NotificationRequest) -> Result<NotificationStatus, Error> {
        println!("Sending email notification: {:?}", notification.event.name.to_string());

        let email = self.build_email(notification);

        match email {
            Ok(email) => match self.smtp_client.send(email) {
                Ok(_) => Ok(notification_sent()),
                Err(err) => Err(msg(err.to_string()))
            },
            Err(err) => Err(err)
        }
    }
}

impl EmailNotificationChannel {
    fn build_email(&self, notification: &NotificationRequest) -> Result<Email, Error> {
        match self.get_recipient_email(notification) {
            Ok(to_email) => {
                let email_template = self.get_email_template(notification);
                Ok(Email {
                    to: to_email.to_string(),
                    subject: email_template.subject().to_string(),
                    body: email_template.body(notification).to_string(),
                })
            }
            Err(err) => Err(err)
        }
    }

    fn get_recipient_email(&self, notification: &NotificationRequest) -> Result<String, Error> {
        match notification.recipient.recipient_type {
            RecipientType::Seller => {
                let result = self.seller_repository.find_by_id(notification.recipient.id);
                match result {
                    Ok(seller) => Ok(seller.email),
                    Err(err) => Err(err)
                }
            }
            RecipientType::Customer => {
                let result = self.customer_repository.find_by_id(notification.recipient.id);
                match result {
                    Ok(customer) => Ok(customer.email),
                    Err(err) => Err(err)
                }
            }
        }
    }

    fn get_email_template<'a>(&'a self, notification_request: &'a NotificationRequest) -> Box<dyn EmailTemplate + '_> {
        let event_name = notification_request.get_event_name();

        match event_name {
            EventName::PurchaseSuccessful => match notification_request.recipient.recipient_type {
                RecipientType::Seller => Box::new(SaleSuccessfulTemplate::new()),
                RecipientType::Customer => Box::new(PurchaseSuccessfulTemplate::new()),
            },

            EventName::PaymentRejected => Box::new(PaymentRejectedTemplate::new()),
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::api::notification_request::{Event, Recipient};
    use crate::model::test_data::{create_test_customer, MockCustomerRepository, MockSellerRepository, MockSmtpClient};
    use super::*;

    #[test]
    fn build_email_should_return_error_when_get_recipient_email_fails() {
        let notification = NotificationRequest {
            recipient: Recipient {
                id: 2, // id 2 will cause find_by_id to fail
                recipient_type: RecipientType::Customer,
            },
            event: Event {
                name: EventName::PurchaseSuccessful,
                detail: "detail".to_string(),
            },
            channel: Channel::Email,
        };

        let email_notification_channel = EmailNotificationChannel {
            smtp_client: Box::new(MockSmtpClient {}),
            customer_repository: Arc::new(MockCustomerRepository {}),
            seller_repository: Arc::new(MockSellerRepository {}),
        };

        let result = email_notification_channel.build_email(&notification);

        assert!(result.is_err());
    }

    #[test]
    fn build_email_ok() {
        let notification = NotificationRequest {
            recipient: Recipient {
                id: 1, // id 1 will cause find_by_id to succeed
                recipient_type: RecipientType::Customer,
            },
            event: Event {
                name: EventName::PurchaseSuccessful,
                detail: "detail".to_string(),
            },
            channel: Channel::Email,
        };

        let email_notification_channel = EmailNotificationChannel {
            smtp_client: Box::new(MockSmtpClient {}),
            customer_repository: Arc::new(MockCustomerRepository {}),
            seller_repository: Arc::new(MockSellerRepository {}),
        };

        let result = email_notification_channel.build_email(&notification);

        assert!(result.is_ok());
    }
}






