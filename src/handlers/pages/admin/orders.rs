use axum::extract::Query;
use maud::Markup;
use serde::Deserialize;

use crate::{
    constants::admin::ITEMS_PER_PAGE,
    data::queries::admin,
    handlers::{context::PageContext, errors::HandlerError},
    models::{order::PaymentStatus, pagination::default_page},
    views::pages::admin as admin_views,
};

#[derive(Deserialize)]
pub struct OrdersQuery {
    #[serde(default = "default_page")]
    pub page: i64,
    pub status: Option<PaymentStatus>,
}

pub async fn get_admin_orders(
    ctx: PageContext,
    Query(query): Query<OrdersQuery>,
) -> Result<Markup, HandlerError> {
    let page = query.page.max(1);
    let status_filter = query.status;

    let paginated = admin::get_orders_paginated(status_filter, page, ITEMS_PER_PAGE).await?;

    Ok(admin_views::orders(&ctx.view_context(), paginated, status_filter))
}
