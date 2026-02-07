use std::collections::HashMap;

use axum::{Form, extract::State, http::StatusCode, response::IntoResponse};
use tower_sessions::Session;
use validator::Validate;

use crate::{
    auth::AuthenticatedUser,
    config::AppConfig,
    constants::messages,
    data::{commands, queries},
    session::FlashMessage,
    handlers::errors::HandlerResult,
    models::{todo::{CreateTodoForm, FIELD_TASK}, UserId},
    paths::pages,
    views::pages as view,
};

use super::parse_validation_errors;

pub async fn post_forms_todos(
    State(config): State<AppConfig>,
    user: AuthenticatedUser,
    session: Session,
    Form(form): Form<CreateTodoForm>,
) -> HandlerResult {
    if let Err(validation_errors) = form.validate() {
        let errors = parse_validation_errors(&validation_errors);
        let current_user = user.as_current_user();
        return render_validation_errors(&current_user, config.site_name(), &user.user_id, &form, errors).await;
    }

    commands::todo::create_todo(&user.user_id, form.task.trim()).await?;
    Ok(FlashMessage::success(messages::TODO_CREATED)
        .set_and_redirect(&session, pages::TODOS)
        .await?)
}

async fn render_validation_errors(
    current_user: &crate::auth::CurrentUser,
    site_name: &str,
    user_id: &UserId,
    form: &CreateTodoForm,
    errors: HashMap<String, String>,
) -> HandlerResult {
    let todos = queries::todo::get_todos_for_user(user_id).await?;

    Ok((
        StatusCode::BAD_REQUEST,
        view::todos(
            current_user,
            None,
            site_name,
            todos,
            Some(&form.task),
            errors.get(FIELD_TASK).map(String::as_str),
        ),
    )
        .into_response())
}
