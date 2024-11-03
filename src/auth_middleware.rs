use crate::jwt_handler::verify_token;
use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    http::header,
    Error, HttpMessage, HttpResponse,
};
use futures_util::future::LocalBoxFuture;
use std::future::{ready, Ready};

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

fn get_auth_header<'a>(req: &'a ServiceRequest) -> Option<String> {
    let header = req.headers().get(header::AUTHORIZATION)?;

    let bearer = header.to_str().ok()?.split("Bearer ").skip(1).collect::<Vec<&str>>();

    if let Some(&token) = bearer.first() {
        return Some(token.to_string())
    }
    return None
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
        if !req.path().starts_with("/api/auth/login") && !req.path().starts_with("/api/auth/register") {
            let token = req
                .cookie("auth_token")
                .map(|cookie| cookie.value().to_string())
                .or(get_auth_header(&req));
            if let Some(token) = token {
                match verify_token(&token) {
                    Ok(claims) => {
                        req.extensions_mut().insert(claims);
                    }
                    Err(e) => {
                        println!("{:?}", e);
                        return Box::pin(async move {
                            let res = req.into_response(HttpResponse::Unauthorized().finish());
                            Ok(res)
                        });
                    }
                }
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
