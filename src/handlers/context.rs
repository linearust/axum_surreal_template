use axum::{
    Extension,
    extract::{FromRef, FromRequestParts, State},
    http::request::Parts,
};

use crate::{auth::CurrentUser, config::AppConfig, session::FlashMessage};

pub struct PageContext {
    pub config: AppConfig,
    pub current_user: CurrentUser,
    pub flash: Option<FlashMessage>,
}

impl PageContext {
    pub fn site_name(&self) -> &str {
        self.config.site_name()
    }

    pub fn flash_ref(&self) -> Option<&FlashMessage> {
        self.flash.as_ref()
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
