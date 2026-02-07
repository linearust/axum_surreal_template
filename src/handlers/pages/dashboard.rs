use maud::Markup;

use crate::{
    constants::{self, errors},
    data::{errors::DataError, queries},
    handlers::{context::PageContext, errors::HandlerError},
    views::pages,
};

pub async fn get_dashboard(ctx: PageContext) -> Result<Markup, HandlerError> {
    let user_id = ctx.current_user.require_authenticated()
        .ok_or(DataError::Unauthorized(errors::AUTHENTICATION_REQUIRED))?;

    let recent_orders =
        queries::order::get_orders_for_user(user_id, constants::dashboard::RECENT_ORDERS_LIMIT)
            .await?;

    Ok(pages::dashboard(
        &ctx.current_user,
        ctx.flash_ref(),
        ctx.site_name(),
        recent_orders,
    ))
}
