#[macro_use] extern crate rocket;

use crate::model::email::smtp::SmtpClient;
use crate::ports::smtp::outlook_client::OutlookClient;

mod api;
mod routes;
mod model;
mod ports;
mod config;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![routes::ping])
        .mount("/", routes![api::notification_api::notification])
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
    use crate::model::notification::test_data::{create_succeed_request, create_test_customer};

    #[test]
    fn send_notification_ok() {



        let rt = Runtime::new().unwrap();
        rt.block_on(mock_customer_response());

        let client = Client::new(rocket()).expect("valid rocket instance");
        let notification_request = &create_succeed_request(RecipientType::Customer, EventName::PurchaseSuccessful, "item detail".to_string());

        let response = client.post("/notification")
            .json(notification_request)
            .dispatch();

        assert_eq!(response.status(), Status::Ok);
    }

    async fn mock_customer_response() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/customer/1"))
            .respond_with(ResponseTemplate::new(200).set_body_json(create_test_customer(1)))
            .mount(&mock_server)
            .await;

        std::env::set_var(IS_INTEGRATION_TEST_ENV, "true");
        std::env::set_var(CUSTOMERS_SERVICE_URL, mock_server.uri());
    }
}

