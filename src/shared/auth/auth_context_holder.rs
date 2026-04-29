use std::future::{ready, Ready};
use actix_web::dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::http::Method;
use actix_web::{HttpMessage, HttpResponse};
use actix_web::error::InternalError;
use futures_util::future::LocalBoxFuture;
use crate::shared::errors::error_response_body::ErrorResponseBody;
use crate::shared::auth::jwt_handler::parse_token;
use crate::shared::auth::auth_user::AuthUser;

pub struct AuthContextHolder;
impl<S, B> Transform<S, ServiceRequest> for AuthContextHolder where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = actix_web::Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = actix_web::Error;
    type Transform = AuthMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthMiddleware { service }))
    }
}

pub struct AuthMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for AuthMiddleware<S> where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = actix_web::Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = actix_web::Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        // calls with the http-option method should always be let through without authentication
        if no_auth_needed(&req) {
            let fut = self.service.call(req);
            return Box::pin(async move {
                return Ok(fut.await?)
            });
        }

        let token = req.headers()
            .get("Authorization")
            .and_then(|v| v.to_str().ok())
            .and_then(|t| t.strip_prefix("Bearer "))
            .unwrap_or("");

        let auth = match parse_token(token) {
            Ok(auth) => AuthUser::from(auth),
            Err(_) => {
                let path = req.path();
                return Box::pin(
                    ready(
                        Err(
                            InternalError::from_response("forbidden", forbidden(path)).into()
                        )
                    )
                );
            },
        };

        req.extensions_mut().insert(auth);

        let fut = self.service.call(req);
        Box::pin(async move {
            Ok(fut.await?)
        })
    }
}


fn forbidden(path: &str) -> HttpResponse {
    HttpResponse::Forbidden().json(ErrorResponseBody::forbidden(path))
}

fn no_auth_needed(req: &ServiceRequest) -> bool {
    req.method() == Method::OPTIONS
}