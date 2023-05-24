#[macro_use]
extern crate rocket;

use rocket::{Build, Rocket};
use utoipa::{OpenApi};
use utoipa_swagger_ui::*;
use crate::docs::ApiDoc;

use crate::model::email::smtp::SmtpClient;
use crate::ports::smtp::outlook_client::OutlookClient;

mod api;
mod routes;
mod model;
mod ports;
mod config;
mod docs;

#[launch]
fn rocket() -> Rocket<Build> {
    rocket::build()
        .mount("/", routes![routes::ping])
        .mount("/", routes![api::notification_api::notification])

        .mount(
            "/",
            SwaggerUi::new("/docs/<_..>").url("/api-docs/openapi.json", ApiDoc::openapi()),
        )
        .register("/", catchers![api::error::not_found, api::error::internal_error, api::error::unprocessable_entity])
}



#[cfg(test)]
mod tests {
    use super::rocket;
    use crate::tests::rocket::local::blocking::Client;
    use rocket::http::Status;
    use rocket::tokio::runtime::Runtime;
    use wiremock::{Mock, MockServer, ResponseTemplate};
    use wiremock::matchers::{method, path};
    use crate::api::notification_request::{EventName, RecipientType};
    use crate::config::properties::{CUSTOMERS_SERVICE_URL, IS_INTEGRATION_TEST_ENV};
    use crate::model::notification::test_data::{create_failing_request, create_succeed_request, create_test_customer};

    #[test]
    fn send_notification_ok() {
        let rt = Runtime::new().unwrap();
        rt.block_on(mock_server());

        let client = Client::new(rocket()).expect("valid rocket instance");
        let notification_request = &create_succeed_request(RecipientType::Customer, EventName::PurchaseSuccessful, "item detail".to_string());

        let response = client.post("/notification")
            .json(notification_request)
            .dispatch();

        assert_eq!(response.status(), Status::Ok);
    }

    #[test]
    fn send_notification_fail_user_not_found() {
        let rt = Runtime::new().unwrap();
        rt.block_on(mock_server());

        let client = Client::new(rocket()).expect("valid rocket instance");
        let notification_request = &create_failing_request();

        let response = client.post("/notification")
            .json(notification_request)
            .dispatch();

        assert_eq!(response.status(), Status::BadRequest);
    }

    async fn mock_server() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/customer/2"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&mock_server)
            .await;

        Mock::given(method("GET"))
            .and(path("/customer/1"))
            .respond_with(ResponseTemplate::new(200).set_body_json(create_test_customer(1)))
            .mount(&mock_server)
            .await;

        std::env::set_var(IS_INTEGRATION_TEST_ENV, "true");
        std::env::set_var(CUSTOMERS_SERVICE_URL, mock_server.uri());
    }
}

