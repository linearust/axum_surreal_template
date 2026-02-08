use crate::{auth::CurrentUser, session::FlashMessage};

pub struct ViewContext<'a> {
    pub current_user: &'a CurrentUser,
    pub flash: Option<&'a FlashMessage>,
    pub site_name: &'a str,
}
