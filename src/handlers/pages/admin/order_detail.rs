use axum::extract::Path;
use maud::Markup;

use crate::{
    constants::errors,
    data::{errors::DataError, queries::admin},
    handlers::{context::PageContext, errors::HandlerError},
    models::OrderId,
    views::pages::admin as admin_views,
};

pub async fn get_admin_order_detail(
    ctx: PageContext,
    Path(raw_order_id): Path<String>,
) -> Result<Markup, HandlerError> {
    let order_id = OrderId::parse(&raw_order_id)
        .ok_or(DataError::NotFound(errors::ORDER_NOT_FOUND))?;
    let order = admin::get_order_detail(&order_id).await?;

    Ok(admin_views::order_detail(
        &ctx.current_user,
        ctx.flash_ref(),
        ctx.site_name(),
        order,
    ))
}
