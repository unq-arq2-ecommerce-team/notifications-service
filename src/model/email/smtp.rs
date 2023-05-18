
pub trait SmtpClient {
    fn send(&self, email: Email) -> Result<String, String>;
}

pub struct Email {
    pub to: String,
    pub subject: String,
    pub body: String,
}