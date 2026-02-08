use axum::http::StatusCode;
use maud::Markup;

use crate::{handlers::context::PageContext, views::pages};

pub async fn handle_404(ctx: PageContext) -> (StatusCode, Markup) {
    (StatusCode::NOT_FOUND, pages::not_found(&ctx.view_context()))
}
