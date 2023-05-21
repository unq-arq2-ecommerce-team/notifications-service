use crate::api::notification_request::NotificationRequest;
use crate::model::email::body_templates::{payment_rejected_template, purchase_mail_template, sale_successful_template};

pub trait EmailTemplate {
    fn body(&self) -> String;
    fn subject(&self) -> String;
}


pub struct PurchaseSuccessfulTemplate<'a> {
    pub notification : &'a NotificationRequest,
}

impl EmailTemplate for PurchaseSuccessfulTemplate<'_> {
    fn body(&self) -> String {
        replace_detail(purchase_mail_template(), self.notification.get_event_detail())
    }

    fn subject(&self) -> String {
        "Compra exitosa".to_string()
    }
}

pub struct PaymentRejectedTemplate<'a> {
    pub notification : &'a NotificationRequest,
}

impl EmailTemplate for PaymentRejectedTemplate<'_> {
    fn body(&self) -> String {
        replace_detail(payment_rejected_template(), self.notification.get_event_detail())
    }

    fn subject(&self) -> String {
        "Pago rechazado".to_string()
    }
}

pub struct SaleSuccessfulTemplate<'a> {
    pub notification : &'a NotificationRequest,
}

impl EmailTemplate for SaleSuccessfulTemplate<'_> {
    fn body(&self) -> String {
        replace_detail(sale_successful_template(), self.notification.get_event_detail())
    }

    fn subject(&self) -> String {
        "Venta exitosa".to_string()
    }
}

fn replace_detail(template: &str, detail: &str) -> String {
    template.replace("{{event_detail}}", detail)
}