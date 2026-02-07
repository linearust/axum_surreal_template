use serde::Deserialize;

use super::shared::IS_ADMIN_SUBQUERY;
use crate::{data::errors::DataError, db::DB, models::{Role, UserId}};

pub struct UserInfo {
    pub email: String,
    pub is_admin: bool,
}

#[derive(Deserialize)]
struct UserInfoRow {
    email: String,
    is_admin: bool,
}

pub async fn get_user_info(user_id: &UserId) -> Result<Option<UserInfo>, DataError> {
    let mut result = DB
        .query(format!(
            "SELECT email, {IS_ADMIN_SUBQUERY} FROM user WHERE id = $user_id",
        ))
        .bind(("user_id", user_id.clone()))
        .bind(("role", Role::Admin.as_str()))
        .await?;

    let user: Option<UserInfoRow> = result.take(0)?;

    Ok(user.map(|u| UserInfo {
        email: u.email,
        is_admin: u.is_admin,
    }))
}

pub async fn get_user_email(user_id: &UserId) -> Result<Option<String>, DataError> {
    #[derive(Deserialize)]
    struct UserRow {
        email: String,
    }

    let user: Option<UserRow> = DB.select(user_id.clone().into_record_id()).await?;
    Ok(user.map(|u| u.email))
}
