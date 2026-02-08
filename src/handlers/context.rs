use axum::{
    Extension,
    extract::{FromRef, FromRequestParts, State},
    http::request::Parts,
};

use crate::{auth::CurrentUser, config::AppConfig, models::UserId, session::FlashMessage, views::context::ViewContext};

pub struct PageContext {
    pub config: AppConfig,
    pub current_user: CurrentUser,
    pub flash: Option<FlashMessage>,
}

impl PageContext {
    pub fn view_context(&self) -> ViewContext<'_> {
        ViewContext {
            current_user: &self.current_user,
            flash: self.flash.as_ref(),
            site_name: self.config.site_name(),
        }
    }

    /// Panics on Guest — only call in protected routes.
    pub fn user_id(&self) -> &UserId {
        match &self.current_user {
            CurrentUser::Authenticated { user_id, .. } => user_id,
            CurrentUser::Guest => panic!("user_id() called in unprotected route — add to protected_routes()"),
        }
    }
}

impl<S> FromRequestParts<S> for PageContext
where
    AppConfig: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = std::convert::Infallible;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let State(config) = State::<AppConfig>::from_request_parts(parts, state)
            .await
            .expect("AppConfig missing from state");

        let Extension(current_user) =
            Extension::<CurrentUser>::from_request_parts(parts, state)
                .await
                .expect("CurrentUser extension missing — check middleware ordering");

        let Extension(flash) =
            Extension::<Option<FlashMessage>>::from_request_parts(parts, state)
                .await
                .expect("FlashMessage extension missing — check middleware ordering");

        Ok(PageContext {
            config,
            current_user,
            flash,
        })
    }
}
