use crate::jwt_handler::verify_token;
use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpMessage, HttpResponse,
};
use futures_util::future::LocalBoxFuture;
use std::{
    collections::BTreeMap,
    future::{ready, Ready},
};

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
            let claims: Result<BTreeMap<String, String>, _> = verify_token(token);
            if let Ok(claims) = claims {
                println!("Claims: {:?}", claims);
                req.extensions_mut().insert(claims);
            } else {
                return Box::pin(async move {
                    let res = req.into_response(HttpResponse::Unauthorized().finish());
                    Ok(res)
                });
            }
        }

        let fut = self.service.call(req);

        Box::pin(fut)
    }
}
