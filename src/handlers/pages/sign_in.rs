use maud::Markup;

use crate::{
    handlers::{context::PageContext, errors::HandlerError},
    views::pages,
};

pub async fn get_sign_in(ctx: PageContext) -> Result<Markup, HandlerError> {
    Ok(pages::sign_in(&ctx.current_user, ctx.flash_ref(), ctx.site_name(), None, None))
}
