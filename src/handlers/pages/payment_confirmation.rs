use axum::extract::Path;
use maud::Markup;

use crate::{
    constants::errors,
    data::{errors::DataError, queries},
    handlers::{context::PageContext, errors::HandlerError},
    models::{order::PaymentStatus, OrderId},
    views::pages,
};

pub async fn get_payment_confirmation(
    ctx: PageContext,
    Path(raw_order_id): Path<String>,
) -> Result<Markup, HandlerError> {
    let user_id = ctx.user_id();
    let order_id = OrderId::parse_or_invalid(&raw_order_id)?;

    let order = queries::order::get_order_for_user(&order_id, user_id).await?;

    if !matches!(order.payment_status, PaymentStatus::Paid) {
        return Err(DataError::Unauthorized(errors::PAYMENT_NOT_COMPLETED).into());
    }

    Ok(pages::payment_confirmation(&ctx.view_context(), &order))
}
