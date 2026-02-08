use axum::extract::Path;
use maud::Markup;

use crate::{
    data::queries,
    handlers::{context::PageContext, errors::HandlerError},
    models::OrderId,
    views::pages,
};

pub async fn get_checkout(
    ctx: PageContext,
    Path(raw_order_id): Path<String>,
) -> Result<Markup, HandlerError> {
    let user_id = ctx.user_id();
    let order_id = OrderId::parse_or_invalid(&raw_order_id)?;

    let order = queries::order::get_order_for_user(&order_id, user_id).await?;

    Ok(pages::checkout(
        &ctx.view_context(),
        &order,
        ctx.config.payment().toss_client_key(),
    ))
}
