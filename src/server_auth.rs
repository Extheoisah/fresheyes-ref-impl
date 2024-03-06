use std::fmt;
use std::sync::Arc;
use tokio::sync::Mutex;
use once_cell::sync::Lazy;
use actix_service::Transform;
use actix_web::{
    HttpResponse,
    dev::{Service,ServiceRequest, ServiceResponse},
    Error, ResponseError,
};
use futures_util::future::{LocalBoxFuture};
use std::future::{ready, Ready};
use std::task::{Context, Poll};



pub struct Authentication;

// This is the shared state where the token will be stored
pub static TOKEN: Lazy<Arc<Mutex<Option<String>>>> = Lazy::new(|| Arc::new(Mutex::new(None)));


impl<S, B> Transform<S, ServiceRequest> for Authentication
    where
        S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
        S::Future: 'static,
        B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = AuthenticationMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthenticationMiddleware { service }))
    }
}

pub struct AuthenticationMiddleware<S> {
    service: S,
}



struct UnauthorizedError;
impl ResponseError for UnauthorizedError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::Unauthorized().json("Unauthorized: Bearer token required")
    }
}

impl fmt::Debug for UnauthorizedError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "UnauthorizedError")
    }
}

impl fmt::Display for UnauthorizedError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Unauthorized: Bearer token required")
    }
}

impl<S, B> Service<ServiceRequest> for AuthenticationMiddleware<S>
    where
        S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
        S::Future: 'static,
        B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }


    fn call(&self, req: ServiceRequest) -> Self::Future {
        let headers = req.headers().clone();
        let token = headers
            .get("Authorization")
            .and_then(|header| header.to_str().ok())
            .unwrap_or_default();

        if token.starts_with("Bearer ") {
            let fut = self.service.call(req);

            // Export the token as an environment variable
            std::env::set_var("BEARER_TOKEN", &token[7..]);

            Box::pin(async move {
                fut.await
            })
        } else {
            println!("No valid token found in request headers");
            Box::pin(async { Err(UnauthorizedError.into()) })
        }
    }
}
