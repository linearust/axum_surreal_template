use maud::Markup;

use crate::{
    data::queries,
    handlers::{context::PageContext, errors::HandlerError},
    views::pages,
};

pub async fn get_root(ctx: PageContext) -> Result<Markup, HandlerError> {
    let user_email = match &ctx.current_user {
        crate::auth::CurrentUser::Authenticated { user_id, .. } => {
            queries::user::get_user_email(user_id).await?
        }
        crate::auth::CurrentUser::Guest => None,
    };

    Ok(pages::root(&ctx.current_user, ctx.flash_ref(), ctx.site_name(), user_email.as_deref(), None, None, None))
}
