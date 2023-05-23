use std::fmt;
use serde::Deserialize;
use serde::Serialize;

#[derive(Deserialize, Serialize)]
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

#[derive(Deserialize, Serialize)]
pub struct Recipient {
    #[serde(rename = "type")]
    pub recipient_type: RecipientType,
    pub id: i32,
}

#[derive(Deserialize, Serialize)]
pub struct Event {
    pub name: EventName,
    pub detail: String,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum RecipientType {
    Seller,
    Customer,
}

#[derive(Deserialize, Serialize, Eq, Hash, PartialEq, Debug)]
#[serde(rename_all = "snake_case")]
pub enum Channel {
    Email,
    Whatsapp,
}

#[derive(Deserialize, Serialize)]
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

impl fmt::Display for Channel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Channel::Email => write!(f, "email"),
            Channel::Whatsapp => write!(f, "whatsapp"),
        }
    }
}