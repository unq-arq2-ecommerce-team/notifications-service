use crate::api::notification_request::NotificationRequest;
use crate::model::email::body_templates::{payment_rejected_template, purchase_mail_template, sale_successful_template};

pub trait EmailTemplate {
    fn body(&self, notification: &NotificationRequest) -> String {
        self.replace_detail(self.template(), notification.get_event_detail())
    }
    fn subject(&self) -> String;
    fn template(&self) -> &str;

    fn replace_detail(&self, template: &str, detail: &str) -> String {
        template.replace("{{event_detail}}", detail)
    }
}


pub struct PurchaseSuccessfulTemplate {}

impl PurchaseSuccessfulTemplate {
    pub fn new() -> Self {
        PurchaseSuccessfulTemplate {}
    }
}

impl EmailTemplate for PurchaseSuccessfulTemplate {
    fn subject(&self) -> String {
        "Compra exitosa".to_string()
    }

    fn template(&self) -> &str {
        purchase_mail_template()
    }
}

pub struct PaymentRejectedTemplate {}

impl PaymentRejectedTemplate {
    pub fn new() -> Self {
        PaymentRejectedTemplate {}
    }
}

impl EmailTemplate for PaymentRejectedTemplate {
    fn subject(&self) -> String {
        "Pago rechazado".to_string()
    }

    fn template(&self) -> &str {
        payment_rejected_template()
    }
}

pub struct SaleSuccessfulTemplate {}

impl SaleSuccessfulTemplate {
    pub fn new() -> Self {
        SaleSuccessfulTemplate {}
    }
}

impl EmailTemplate for SaleSuccessfulTemplate {
    fn subject(&self) -> String {
        "Venta exitosa".to_string()
    }

    fn template(&self) -> &str {
        sale_successful_template()
    }
}

