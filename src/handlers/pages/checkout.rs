use axum::extract::Path;
use maud::Markup;

use crate::{
    constants::errors,
    data::{errors::DataError, queries},
    handlers::{context::PageContext, errors::HandlerError},
    models::OrderId,
    views::pages,
};

pub async fn get_checkout(
    ctx: PageContext,
    Path(raw_order_id): Path<String>,
) -> Result<Markup, HandlerError> {
    let user_id = ctx.current_user.require_authenticated()
        .ok_or(DataError::Unauthorized(errors::AUTHENTICATION_REQUIRED))?;
    let order_id = OrderId::parse(&raw_order_id)
        .ok_or(DataError::NotFound(errors::ORDER_NOT_FOUND))?;

    let order = queries::order::get_order_for_user(&order_id, user_id).await?;

    Ok(pages::checkout(
        &ctx.current_user,
        ctx.flash_ref(),
        ctx.site_name(),
        &order,
        ctx.config.payment().toss_client_key(),
    ))
}
