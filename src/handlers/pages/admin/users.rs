use axum::extract::Query;
use maud::Markup;

use crate::{
    constants::admin::ITEMS_PER_PAGE,
    data::queries::admin,
    handlers::{context::PageContext, errors::HandlerError},
    models::pagination::PaginationQuery,
    views::pages::admin as admin_views,
};

pub async fn get_admin_users(
    ctx: PageContext,
    Query(query): Query<PaginationQuery>,
) -> Result<Markup, HandlerError> {
    let page = query.page.max(1);

    let paginated = admin::get_users_paginated(page, ITEMS_PER_PAGE).await?;

    Ok(admin_views::users(
        &ctx.current_user,
        ctx.flash_ref(),
        ctx.site_name(),
        paginated,
    ))
}
