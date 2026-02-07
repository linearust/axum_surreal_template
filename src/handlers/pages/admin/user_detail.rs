use axum::extract::{Path, Query};
use maud::Markup;

use crate::{
    constants::admin::ITEMS_PER_PAGE,
    data::queries::admin,
    handlers::{context::PageContext, errors::HandlerError},
    models::{pagination::PaginationQuery, UserId},
    views::pages::admin as admin_views,
};

pub async fn get_admin_user_detail(
    ctx: PageContext,
    Path(raw_user_id): Path<String>,
    Query(query): Query<PaginationQuery>,
) -> Result<Markup, HandlerError> {
    let page = query.page.max(1);
    let user_id = UserId::parse_or_invalid(&raw_user_id)?;

    let user = admin::get_user_detail(&user_id).await?;
    let paginated_orders = admin::get_user_orders_paginated(&user_id, page, ITEMS_PER_PAGE).await?;

    Ok(admin_views::user_detail(
        &ctx.current_user,
        ctx.flash_ref(),
        ctx.site_name(),
        user,
        paginated_orders,
    ))
}
