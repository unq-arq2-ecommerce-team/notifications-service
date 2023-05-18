use std::sync::Arc;
use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};

use crate::api::notification_request::{Channel, Event, EventName, NotificationRequest};
use crate::model::notification_service::NotificationChannel;
use crate::model::email::body_templates::purchase_mail_template;
use crate::model::email::body_templates::payment_rejected_template;
use crate::model::email::email_templates::{EmailTemplate, PaymentRejectedTemplate, PurchaseSuccessfulTemplate};
use crate::model::email::smtp::Email;
use crate::SmtpClient;

pub struct EmailNotificationChannel {
    pub(crate) smtp_client: Box<dyn SmtpClient>
}

impl NotificationChannel for EmailNotificationChannel {
    fn get_channel(&self) -> Channel {
        Channel::Email
    }

    fn send(&self, notification: &NotificationRequest) {
        println!("Sending email notification: {:?}", notification.event.name.to_string());


        let result = self.smtp_client.send(build_email(notification));

        if result.is_ok() {
            println!("Email sent successfully!");
        } else {
            println!("Failed to send email: {:?}", result.err().unwrap());
        }
    }
}

fn build_email(notification: &NotificationRequest) -> Email {
    let email_template = get_email_template(notification);

    // temporal
    let to_email = "Lucas <lucas.matw@gmail.com>";

    Email {
        to: to_email.to_string(),
        subject: email_template.subject().to_string(),
        body: email_template.body().to_string(),
    }
}



fn get_email_template(notification_request: &NotificationRequest) -> Box<dyn EmailTemplate + '_> {
    let event_name = notification_request.get_event_name();

    match event_name {
        EventName::PurchaseSuccessful => Box::new(PurchaseSuccessfulTemplate { notification: notification_request}),
        EventName::PaymentRejected => Box::new(PaymentRejectedTemplate{ notification: notification_request }),
    }
}

