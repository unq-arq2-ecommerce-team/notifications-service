use lettre::{Message, SmtpTransport, Transport};
use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use crate::api::notification_request::NotificationRequest;
use crate::model::email::smtp::{Email};
use crate::SmtpClient;

pub struct OutlookClient {
    pub(crate) smtp_server: SmtpTransport,
}

impl SmtpClient for OutlookClient {

    fn send(&self, email: Email) -> Result<String, String> {
        let result = self.smtp_server.send(&Self::build_message(email));

        match result {
            Ok(_) => Ok("Email sent successfully".to_string()),
            Err(err) => Err(format!("Failed to send email: {:?}", err))
        }
    }
}

impl OutlookClient {
    pub fn new() -> Self {
        OutlookClient { smtp_server: smtp_server_impl() }
    }

    fn build_message(email: Email) -> Message {
        Message::builder()
            .from("ArqSoft2-TP <arq-soft2-unq@outlook.com>".parse().unwrap())
            .reply_to("ArqSoft2-TP <arq-soft2-unq@outlook.com>".parse().unwrap())
            .to(email.to.parse().unwrap())
            .subject(email.subject.to_string())
            .header(ContentType::TEXT_HTML)
            .body(email.body.to_string())
            .unwrap()
    }
}


impl Default for OutlookClient {
    fn default() -> Self {
        OutlookClient::new()
    }
}

fn smtp_server_impl() -> SmtpTransport {
    let creds = Credentials::new("arq-soft2-unq@outlook.com".to_owned(), "PrimerCuatrimestre2023".to_owned());

    SmtpTransport::starttls_relay("smtp.office365.com")
        .unwrap()
        .credentials(creds)
        .port(587)
        .build()
}





