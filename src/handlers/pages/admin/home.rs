use maud::Markup;

use crate::{
    data::queries::admin,
    handlers::{context::PageContext, errors::HandlerError},
    views::pages::admin as admin_views,
};

pub async fn get_admin_home(ctx: PageContext) -> Result<Markup, HandlerError> {
    let stats = admin::get_admin_stats().await?;

    Ok(admin_views::home(
        &ctx.current_user,
        ctx.flash_ref(),
        ctx.site_name(),
        stats,
    ))
}
