use std::{collections::BTreeMap, future::{ready, Ready}};

use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpResponse,
};
use futures_util::future::LocalBoxFuture;
use hmac::{Hmac, Mac};
use jwt::VerifyWithKey;
use sha2::Sha256;

pub struct Auth;

impl<S> Transform<S, ServiceRequest> for Auth
where
    S: Service<ServiceRequest, Response = ServiceResponse, Error = Error>,
    S::Future: 'static,
{
    type Response = ServiceResponse;
    type Error = Error;
    type InitError = ();
    type Transform = AuthMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthMiddleware { service }))
    }
}

pub struct AuthMiddleware<S> {
    service: S,
}

impl<S> Service<ServiceRequest> for AuthMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse, Error = Error>,
    S::Future: 'static,
{
    type Response = ServiceResponse;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let header = match req.headers().get("Authorization") {
            Some(header) => header,
            None => {
                return Box::pin(async move {
                    let res = req.into_response(HttpResponse::Unauthorized().finish());
                    return Ok(res);
                });
            }
        };

        let header = match header.to_str() {
            Ok(header) => header,
            Err(_) => {
                return Box::pin(async move {
                    let res = req.into_response(HttpResponse::Unauthorized().finish());
                    return Ok(res);
                });
            }
        };
        let header_parts = header.splitn(2, " ").collect::<Vec<&str>>();
        let token = match (header_parts.first(), header_parts.last()) {
            (Some(&"Bearer"), Some(&token)) | (Some(&"bearer"), Some(&token)) => token,
            _ => {
                return Box::pin(async move {
                    let res = req.into_response(HttpResponse::Unauthorized().finish());
                    return Ok(res);
                });
            }
        };

        let key_string = std::env::var("JWT_SECRET").unwrap();
        let key: Hmac<Sha256> = hmac::Hmac::new_from_slice(&key_string.as_bytes()).unwrap();

        let claims: BTreeMap<String, String> = match token.verify_with_key(&key) {
            Ok(b) => b,
            Err(e) => {
                println!("Failed to verify key: {}", e);
                return Box::pin(async move {
                    let res = req.into_response(HttpResponse::Unauthorized().finish());
                    return Ok(res);
                });
            }
        };

        println!("Claims: {:?}", claims);

        let fut = self.service.call(req);

        Box::pin(async move {
            Ok(fut.await?)
        })
    }
}
