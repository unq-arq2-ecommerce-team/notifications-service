use utoipa::{OpenApi};
use utoipa_swagger_ui::*;

use crate::api;
use crate::model;

#[derive(OpenApi)]
#[openapi(
    paths(
        api::notification_api::notification
    ),
    components(
        schemas(
            api::notification_request::NotificationRequest,
            api::notification_request::Channel,
            api::notification_request::Recipient,
            api::notification_request::RecipientType,
            api::notification_request::Event,
            api::notification_request::EventName,
            model::notification::NotificationStatus,
            api::error::ApiError,
            model::error::ErrorKind
        )
    ),
    tags(
     (name = "Notification", description = "Notification management endpoints.")
    )
)]
pub struct ApiDoc;