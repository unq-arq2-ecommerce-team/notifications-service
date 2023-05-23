use std::sync::Arc;

use crate::api::notification_request::{Channel, EventName, NotificationRequest, RecipientType};
use crate::model::email::email_templates::{EmailTemplate, PaymentRejectedTemplate, PurchaseSuccessfulTemplate, SaleSuccessfulTemplate};
use crate::model::email::smtp::Email;
use crate::model::error::{Error, msg};
use crate::model::notification::notification_service::NotificationChannel;
use crate::model::notification::{notification_sent, NotificationStatus};
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
                    Err(err) => {
                        Err(err)
                    }
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
    use crate::model::email::body_templates::{payment_rejected_template, purchase_mail_template, sale_successful_template};
    use crate::model::notification::test_data::{create_failing_request, create_succeed_request, Mocked};
    use super::*;

    #[test]
    fn build_email_should_return_error_when_get_recipient_email_fails() {
        let notification = create_failing_request();

        let email_notification_channel = EmailNotificationChannel::mocked();

        let result = email_notification_channel.build_email(&notification);

        assert!(result.is_err());
    }

    #[test]
    fn build_purchase_customer_email_ok() {
        let event_detail = "Nike Air Max 90".to_string();
        let notification = create_succeed_request(RecipientType::Customer, EventName::PurchaseSuccessful, event_detail.clone());

        let email_notification_channel = EmailNotificationChannel::mocked();

        let result = email_notification_channel.build_email(&notification);

        assert!(result.is_ok());

        let expected_body_email = purchase_mail_template().replace("{{event_detail}}", event_detail.as_str());
        assert!(result.unwrap().body.contains(expected_body_email.as_str()));
    }

    #[test]
    fn build_purchase_seller_email_ok() {
        let event_detail = "Nike Air Max 90".to_string();
        let notification = create_succeed_request(RecipientType::Seller, EventName::PurchaseSuccessful, event_detail.clone());

        let email_notification_channel = EmailNotificationChannel::mocked();

        let result = email_notification_channel.build_email(&notification);

        assert!(result.is_ok());

        let expected_body_email = sale_successful_template().replace("{{event_detail}}", event_detail.as_str());
        assert!(result.unwrap().body.contains(expected_body_email.as_str()));
    }

    #[test]
    fn build_rejected_payment_email_ok() {
        let event_detail = "id: 1234 - ARS 15000".to_string();
        let notification = create_succeed_request(RecipientType::Customer, EventName::PaymentRejected, event_detail.clone());

        let email_notification_channel = EmailNotificationChannel::mocked();

        let result = email_notification_channel.build_email(&notification);

        assert!(result.is_ok());

        let expected_body_email = payment_rejected_template().replace("{{event_detail}}", event_detail.as_str());
        assert!(result.unwrap().body.contains(expected_body_email.as_str()));
    }
}






