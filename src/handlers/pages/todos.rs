use maud::Markup;

use crate::{
    constants::errors,
    data::{errors::DataError, queries},
    handlers::{context::PageContext, errors::HandlerError},
    views::pages,
};

pub async fn get_todos(ctx: PageContext) -> Result<Markup, HandlerError> {
    let user_id = ctx.current_user.require_authenticated()
        .ok_or(DataError::Unauthorized(errors::AUTHENTICATION_REQUIRED))?;

    let todos = queries::todo::get_todos_for_user(user_id).await?;

    Ok(pages::todos(&ctx.current_user, ctx.flash_ref(), ctx.site_name(), todos, None, None))
}
