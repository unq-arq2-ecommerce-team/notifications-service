use std::sync::Arc;
use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport};

use crate::api::notification_request::{Channel, Event, EventName, NotificationRequest, RecipientType};
use crate::model::notification_service::NotificationChannel;
use crate::model::email::body_templates::purchase_mail_template;
use crate::model::email::body_templates::payment_rejected_template;
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

        let result = self.smtp_client.send(self.build_email(notification));

        if result.is_ok() {
            println!("Email sent successfully!");
        } else {
            println!("Failed to send email: {:?}", result.err().unwrap());
        }
    }
}

impl EmailNotificationChannel {
    fn build_email(&self, notification: &NotificationRequest) -> Email {
        let email_template = self.get_email_template(notification);

        let to_email = self.get_to_email(notification);

        Email {
            to: to_email.to_string(),
            subject: email_template.subject().to_string(),
            body: email_template.body().to_string(),
        }
    }

    fn get_to_email(&self, notification: &NotificationRequest) -> String {
        match notification.recipient.recipient_type {
            RecipientType::Seller => {
                let result = self.seller_repository.find_by_id(notification.recipient.id);
                match result {
                    Ok(seller) => seller.email,
                    Err(_) => panic!("Seller not found")
                }
            }
            RecipientType::Customer => {
                let result = self.customer_repository.find_by_id(notification.recipient.id);
                match result {
                    Ok(customer) => customer.email,
                    Err(_) => panic!("Customer not found")
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






