use axum::{
    Extension,
    extract::FromRequestParts,
    http::request::Parts,
};

use crate::models::UserId;

pub const SESSION_USER_ID_KEY: &str = "authenticated_user_id";

/// Injected via Extension by load_session_extensions middleware.
///
/// Protected routes use `require_authentication` middleware which redirects
/// guests before handlers run.
#[derive(Clone, Debug)]
pub enum CurrentUser {
    Authenticated {
        user_id: UserId,
        email: String,
        is_admin: bool,
    },
    Guest,
}

impl CurrentUser {
    pub fn is_admin(&self) -> bool {
        match self {
            CurrentUser::Authenticated { is_admin, .. } => *is_admin,
            CurrentUser::Guest => false,
        }
    }

    pub fn is_authenticated(&self) -> bool {
        matches!(self, CurrentUser::Authenticated { .. })
    }
}

/// Extractor for protected routes — panics on Guest (programming error).
/// Only use in routes behind `require_authentication` middleware.
#[derive(Clone, Debug)]
pub struct AuthenticatedUser {
    pub user_id: UserId,
    pub email: String,
    pub is_admin: bool,
}

impl AuthenticatedUser {
    pub fn as_current_user(&self) -> CurrentUser {
        CurrentUser::Authenticated {
            user_id: self.user_id.clone(),
            email: self.email.clone(),
            is_admin: self.is_admin,
        }
    }
}

impl<S> FromRequestParts<S> for AuthenticatedUser
where
    S: Send + Sync,
{
    type Rejection = std::convert::Infallible;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let Extension(current_user) =
            Extension::<CurrentUser>::from_request_parts(parts, state)
                .await
                .expect("CurrentUser extension missing — check middleware ordering");

        match current_user {
            CurrentUser::Authenticated { user_id, email, is_admin } => {
                Ok(AuthenticatedUser { user_id, email, is_admin })
            }
            CurrentUser::Guest => {
                panic!("AuthenticatedUser extracted in unprotected route — add to protected_routes()")
            }
        }
    }
}
