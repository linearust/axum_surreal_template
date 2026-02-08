use maud::Markup;

use crate::{
    constants,
    data::queries,
    handlers::{context::PageContext, errors::HandlerError},
    views::pages,
};

pub async fn get_dashboard(ctx: PageContext) -> Result<Markup, HandlerError> {
    let user_id = ctx.user_id();

    let recent_orders =
        queries::order::get_orders_for_user(user_id, constants::dashboard::RECENT_ORDERS_LIMIT)
            .await?;

    Ok(pages::dashboard(&ctx.view_context(), recent_orders))
}
