use log::*;
use tonic::{Interceptor, Request, Status};

pub struct AuthorizingInterceptor {
    expected_meta: Option<String>,
}

impl AuthorizingInterceptor {
    pub fn new(secret: Option<String>) -> Self {
        if secret.is_none() {
            warn!("You did not configure a secret key, so everyone will be able to connect to this controller");
        }

        let expected_meta = secret.map(|secret| {
            format!("Bearer {}", secret)
        });

        Self { expected_meta }
    }

    fn allows(&self, meta: Option<&str>) -> bool {
        match &self.expected_meta {
            Some(expected_meta) => {
                meta == Some(expected_meta)
            }

            None => {
                true
            }
        }
    }
}

impl Into<Interceptor> for AuthorizingInterceptor {
    fn into(self) -> Interceptor {
        Interceptor::new(move |req: Request<()>| {
            let meta = req
                .metadata()
                .get("authorization")
                .map(|meta| meta.to_str().ok())
                .flatten();

            if self.allows(meta) {
                Ok(req)
            } else {
                Err(Status::unauthenticated("Invalid secret provided"))
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod when_secret_has_been_set {
        use super::*;

        fn auth() -> AuthorizingInterceptor {
            AuthorizingInterceptor::new(Some("valid-token".to_string()))
        }

        #[test]
        fn then_user_with_valid_token_is_accepted() {
            assert!(
                auth().allows(Some("Bearer valid-token"))
            );
        }

        #[test]
        fn then_user_with_invalid_token_is_rejected() {
            assert!(
                !auth().allows(Some("Bearer invalid-token"))
            );
        }

        #[test]
        fn then_user_with_empty_token_is_rejected() {
            assert!(
                !auth().allows(None)
            );
        }
    }

    mod when_secret_has_not_been_set {
        use super::*;

        fn auth() -> AuthorizingInterceptor {
            AuthorizingInterceptor::new(None)
        }

        #[test]
        fn then_user_with_token_can_connect() {
            assert!(
                auth().allows(Some("Bearer some-token"))
            );
        }

        #[test]
        fn then_user_without_token_can_connect() {
            assert!(
                auth().allows(None)
            );
        }
    }
}