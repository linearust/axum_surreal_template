use axum::extract::Path;
use maud::Markup;

use crate::{
    data::queries::admin,
    handlers::{context::PageContext, errors::HandlerError},
    models::OrderId,
    views::pages::admin as admin_views,
};

pub async fn get_admin_order_detail(
    ctx: PageContext,
    Path(raw_order_id): Path<String>,
) -> Result<Markup, HandlerError> {
    let order_id = OrderId::parse_or_invalid(&raw_order_id)?;
    let order = admin::get_order_detail(&order_id).await?;

    Ok(admin_views::order_detail(&ctx.view_context(), order))
}
