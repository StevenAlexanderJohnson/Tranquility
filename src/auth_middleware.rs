use std::{
    collections::BTreeMap,
    future::{ready, Ready},
};

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
        let token = req
            .headers()
            .get("Authorization")
            .and_then(|header| header.to_str().ok())
            .and_then(|header| {
                let parts = header.splitn(2, " ").collect::<Vec<&str>>();
                if parts.len() == 2 && (parts[0] == "Bearer" || parts[0] == "bearer") {
                    Some(parts[1])
                } else {
                    None
                }
            });

        if let Some(token) = token {
            let key_string = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
            let key: Hmac<Sha256> = hmac::Hmac::new_from_slice(&key_string.as_bytes()).unwrap();

            let claims: Result<BTreeMap<String, String>, _> = token.verify_with_key(&key);
            if let Ok(claims) = claims {
                println!("Claims: {:?}", claims);
            } else {
                return Box::pin(async move {
                    let res = req.into_response(HttpResponse::Unauthorized().finish());
                    return Ok(res);
                });
            }
        }

        let fut = self.service.call(req);

        Box::pin(async move { Ok(fut.await?) })
    }
}
