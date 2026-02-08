use maud::Markup;

use crate::{
    handlers::{context::PageContext, errors::HandlerError},
    views::pages,
};

pub async fn get_sign_in(ctx: PageContext) -> Result<Markup, HandlerError> {
    Ok(pages::sign_in(&ctx.view_context(), None, None))
}
