use maud::Markup;

use crate::{
    handlers::{context::PageContext, errors::HandlerError},
    views::pages,
};

pub async fn get_text_analyzer(ctx: PageContext) -> Result<Markup, HandlerError> {
    Ok(pages::text_analyzer(&ctx.current_user, ctx.flash_ref(), ctx.site_name()))
}
