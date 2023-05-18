use std::fmt;

#[derive(serde::Deserialize)]
pub struct NotificationRequest {
    pub event: Event,
    pub channel: Channel,
    pub recipient: Recipient,
}

impl NotificationRequest {
    pub fn get_event_name(&self) -> &EventName {
        &self.event.name
    }

    pub fn get_event_detail(&self) -> &str {
        &*self.event.detail
    }
}

#[derive(serde::Deserialize)]
pub struct Recipient {
    #[serde(rename = "type")]
    pub recipient_type: RecipientType,
    pub id: u32,
}

#[derive(serde::Deserialize)]
pub struct Event {
    pub name: EventName,
    pub detail: String,
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RecipientType {
    Seller,
    Customer,
}

#[derive(serde::Deserialize, Eq, Hash, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum Channel {
    Email,
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EventName {
    PurchaseSuccessful,
    PaymentRejected,
}

impl fmt::Display for EventName {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            EventName::PurchaseSuccessful => write!(f, "purchase_successful"),
            EventName::PaymentRejected => write!(f, "payment_rejected"),
        }
    }
}