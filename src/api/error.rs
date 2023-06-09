use std::{
    error,
    fmt,
    io::Cursor,
};

use serde::Serializer;
use utoipa::ToSchema;
use rocket::{
    http,
    request::Request,
    response::{
        self,
        content,
        Responder,
        Response,
    },
};
use rocket::serde::json::serde_json;
use thiserror::Error;
use serde::Serialize;

use crate::model;

/** The main application error. */
#[derive(Error, Debug, ToSchema)]
pub enum ApiError {
    #[error("the user input was invalid")]
    BadRequest(#[source] Box<dyn error::Error + Send + Sync>),
    #[error("an unexpected error occurred")]
    Other(#[source] Box<dyn error::Error + Send + Sync>),
}

impl<'r, 'o: 'r> Responder<'r, 'o> for ApiError {
    fn respond_to(self, _: &Request) -> response::Result<'o> {
        let (status, err) = match self {
            ApiError::BadRequest(err) => {
                debug!("request failed with {:?}", err);
                (http::Status::BadRequest, err)
            }
            ApiError::Other(err) => {
                error!("request failed with {:?}", err);
                (http::Status::InternalServerError, err)
            }
        };

        let err = serde_json::to_vec(&SerializeError { msg: &err }).unwrap_or_else(|_| Vec::new());

        Response::build()
            .sized_body(None::<usize>, Cursor::new(err))
            .header(http::ContentType::JSON)
            .status(status)
            .ok()
    }
}

impl From<model::error::Error> for ApiError {
    fn from(err: model::error::Error) -> Self {
        use crate::model::error::ErrorKind::*;

        match err.split() {
            (BadInput, err) => ApiError::BadRequest(Box::try_from(err).unwrap()),
            (_, err) => ApiError::Other(Box::try_from(err).unwrap()),
        }
    }
}

impl From<Box<dyn error::Error + Send + Sync>> for ApiError {
    fn from(err: Box<dyn error::Error + Send + Sync>) -> Self {
        ApiError::Other(err)
    }
}

#[derive(Serialize)]
struct SerializeError<'a> {
    #[serde(serialize_with = "serialize_msg")]
    msg: &'a dyn fmt::Display,
}

fn serialize_msg<S>(msg: &&dyn fmt::Display, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
{
    s.collect_str(msg)
}

#[catch(500)]
pub fn internal_error(_: &Request) -> content::RawJson<Vec<u8>> {
    let err = serde_json::to_vec(&SerializeError {
        msg: &"an internal error occurred",
    })
        .unwrap_or_else(|_| Vec::new());

    content::RawJson(err)
}

#[catch(404)]
pub fn not_found(_: &Request) -> content::RawJson<Vec<u8>> {
    let err =
        serde_json::to_vec(&SerializeError { msg: &"not found" }).unwrap_or_else(|_| Vec::new());

    content::RawJson(err)
}

#[catch(422)]
pub fn unprocessable_entity(_: &Request) -> content::RawJson<Vec<u8>> {
    let err =
        serde_json::to_vec(&SerializeError { msg: &"422 Unprocessable Entity" }).unwrap_or_else(|_| Vec::new());

    content::RawJson(err)
}