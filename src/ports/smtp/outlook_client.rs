use lettre::{Message, SmtpTransport, Transport};
use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use crate::api::notification_request::NotificationRequest;
use crate::config::properties::Properties;
use crate::model::email::smtp::{Email};
use crate::SmtpClient;

pub struct OutlookClient {
    pub(crate) smtp_server: SmtpTransport,
    properties: Properties,
}

impl SmtpClient for OutlookClient {

    fn send(&self, email: Email) -> Result<String, String> {
        let result = self.smtp_server.send(&self.build_message(email));

        match result {
            Ok(_) => Ok("Email sent successfully".to_string()),
            Err(err) => Err(format!("Failed to send email: {:?}", err))
        }
    }
}

impl OutlookClient {
    pub fn new() -> Self {
        OutlookClient {
            properties: Properties::new(),
            smtp_server: smtp_server_impl(Properties::new()),
        }
    }

    fn build_message(&self, email: Email) -> Message {
        Message::builder()
            .from(self.properties.get("SMTP_FROM").parse().unwrap())
            .reply_to(self.properties.get("SMTP_FROM").parse().unwrap())
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

fn smtp_server_impl(properties :Properties) -> SmtpTransport {

    let username = properties.get("SMTP_USERNAME");
    let password = properties.get("SMTP_PASSWORD");
    let port = properties.get("SMTP_PORT").parse::<u16>().unwrap();
    let host = &properties.get("SMTP_HOST");

    let creds = Credentials::new(username, password);

    SmtpTransport::starttls_relay(host)
        .unwrap()
        .credentials(creds)
        .port(port)
        .build()
}





