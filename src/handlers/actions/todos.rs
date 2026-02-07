use axum::extract::Path;

use crate::{
    auth::AuthenticatedUser,
    data::commands,
    handlers::errors::HandlerResult,
    models::TodoId,
    views::{pages, response as htmx},
};

pub async fn delete_actions_todos_todo_id(
    user: AuthenticatedUser,
    Path(raw_todo_id): Path<String>,
) -> HandlerResult {
    let todo_id = TodoId::parse_or_invalid(&raw_todo_id)?;

    commands::todo::delete_todo(&user.user_id, &todo_id).await?;

    Ok(htmx::empty_ok_response())
}

pub async fn patch_actions_todos_todo_id_toggle(
    user: AuthenticatedUser,
    Path(raw_todo_id): Path<String>,
) -> HandlerResult {
    let todo_id = TodoId::parse_or_invalid(&raw_todo_id)?;

    let todo = commands::todo::toggle_todo_completion(&user.user_id, &todo_id).await?;
    Ok(htmx::html_fragment(pages::todo_item(&todo)))
}
