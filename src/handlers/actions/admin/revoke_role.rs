use axum::extract::Path;
use tower_sessions::Session;

use crate::{
    auth::AuthenticatedUser,
    constants::messages,
    data::commands::admin,
    session::FlashMessage,
    handlers::errors::HandlerResult,
    models::UserId,
    paths::helpers,
};

pub async fn delete_actions_admin_users_user_id_revoke_role(
    Path(raw_user_id): Path<String>,
    user: AuthenticatedUser,
    session: Session,
) -> HandlerResult {
    let user_id = UserId::parse_or_invalid(&raw_user_id)?;

    admin::revoke_admin_role(&user_id, &user.user_id).await?;

    Ok(FlashMessage::success(messages::ADMIN_ROLE_REVOKED)
        .set_and_redirect(&session, &helpers::user_detail_path(&user_id))
        .await?)
}
