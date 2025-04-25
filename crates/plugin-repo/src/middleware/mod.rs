use actix_web::{Error, FromRequest, HttpRequest};
use std::future;
use std::future::Ready;

pub struct PluginPublishingAuthToken;

impl FromRequest for PluginPublishingAuthToken {
    type Error = Error;
    type Future = Ready<actix_web::Result<Self>>;

    fn from_request(req: &HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
        match req.headers().get("Authorization") {
            Some(header_value) => match header_value.to_str() {
                Ok(val) if val == *crate::PASSWORD => future::ready(Ok(PluginPublishingAuthToken)),
                _ => future::ready(Err(actix_web::error::ErrorUnauthorized("Invalid token"))),
            },
            None => future::ready(Err(actix_web::error::ErrorUnauthorized("Missing token"))),
        }
    }
}
