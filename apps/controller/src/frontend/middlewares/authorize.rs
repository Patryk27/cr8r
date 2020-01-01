use std::future::Future;

use log::*;

use super::*;

pub struct AuthorizeMiddleware {
    secret: Option<String>,
}

impl AuthorizeMiddleware {
    pub fn new(secret: Option<String>) -> Self {
        if secret.is_none() {
            warn!("You did not configure a secret key, so everybody will be able to connect to this controller");
        }

        Self { secret }
    }

    pub fn handle(
        &self,
        service: &mut impl Service<MiddlewareRequest>,
        request: MiddlewareRequest,
    ) -> impl Future<Output=MiddlewareResponse> {
        let authorized = if let Some(secret) = &self.secret {
            let auth = request
                .headers()
                .get("authorization")
                .map(|h| h.to_str());

            if let Some(Ok(auth)) = auth {
                auth == secret
            } else {
                false
            }
        } else {
            true
        };

        let call = service.call(request);

        async {
            if authorized {
                call.await
            } else {
                drop(call);

                let res = Response::builder()
                    .header("grpc-status", "16")
                    .body(BoxBody::empty())
                    .unwrap();

                Ok(res)
            }
        }
    }
}