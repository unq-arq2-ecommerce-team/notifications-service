use std::sync::Arc;

use crate::api::notification_request::{Channel, EventName, NotificationRequest, RecipientType};
use crate::model::notification_service::NotificationChannel;
use crate::model::email::email_templates::{EmailTemplate, PaymentRejectedTemplate, PurchaseSuccessfulTemplate};
use crate::model::email::smtp::Email;
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

    fn send(&self, notification: &NotificationRequest) {
        println!("Sending email notification: {:?}", notification.event.name.to_string());

        match self.smtp_client.send(self.build_email(notification)) {
            Ok(_) => println!("Email sent successfully!"),
            Err(err) => println!("Failed to send email: {:?}", err.to_string())
        }
    }
}

impl EmailNotificationChannel {
    fn build_email(&self, notification: &NotificationRequest) -> Email {
        let email_template = self.get_email_template(notification);

        let to_email = self.get_recipient_email(notification);

        Email {
            to: to_email.to_string(),
            subject: email_template.subject().to_string(),
            body: email_template.body().to_string(),
        }
    }

    fn get_recipient_email(&self, notification: &NotificationRequest) -> String {
        match notification.recipient.recipient_type {
            RecipientType::Seller => {
                let result = self.seller_repository.find_by_id(notification.recipient.id);
                match result {
                    Ok(seller) => seller.email,
                    Err(err) => panic!("{} - {}", "Seller not found", err.to_string())
                }
            }
            RecipientType::Customer => {
                let result = self.customer_repository.find_by_id(notification.recipient.id);
                match result {
                    Ok(customer) => customer.email,
                    Err(err) => panic!("{} - {}", "Customer not found", err.to_string())
                }
            }
        }
    }

    fn get_email_template<'a>(&'a self, notification_request: &'a NotificationRequest) -> Box<dyn EmailTemplate + '_> {
        let event_name = notification_request.get_event_name();

        match event_name {
            EventName::PurchaseSuccessful => Box::new(PurchaseSuccessfulTemplate { notification: notification_request }),
            EventName::PaymentRejected => Box::new(PaymentRejectedTemplate { notification: notification_request }),
        }
    }
}






