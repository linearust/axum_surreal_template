use serde::Deserialize;

use crate::{data::errors::DataError, db::DB, models::{Role, UserId}};

// Reusable SurrealQL subqueries operating on $parent user row.
// Bind $role and $paid when using these.
pub const IS_ADMIN_SUBQUERY: &str = "(SELECT count() FROM user_role WHERE user = $parent.id AND role = $role GROUP ALL)[0].count OR 0 > 0 AS is_admin";
pub const ORDER_COUNT_SUBQUERY: &str = r#"(SELECT count() FROM order WHERE user = $parent.id AND payment_status = $paid GROUP ALL)[0].count OR 0 AS order_count"#;
pub const TOTAL_SPENT_SUBQUERY: &str = r#"(SELECT math::sum(price_amount) FROM order WHERE user = $parent.id AND payment_status = $paid GROUP ALL)[0]["math::sum"] OR 0 AS total_spent"#;

/// Query result type for COUNT() aggregations.
/// Use with: SELECT count() as count FROM ... GROUP ALL
#[derive(Deserialize)]
pub struct CountResult {
    pub count: i64,
}

impl CountResult {
    pub fn unwrap_or_zero(result: Option<Self>) -> i64 {
        result.map(|c| c.count).unwrap_or(0)
    }
}

/// Query result type for SUM() aggregations.
/// Use with: SELECT math::sum(field) as total FROM ... GROUP ALL
#[derive(Deserialize)]
pub struct SumResult {
    pub total: Option<i64>,
}

impl SumResult {
    pub fn unwrap_or_zero(result: Option<Self>) -> i64 {
        result.and_then(|s| s.total).unwrap_or(0)
    }
}

pub async fn check_user_is_admin(user_id: &UserId) -> Result<bool, DataError> {
    let mut result = DB
        .query("SELECT count() as count FROM user_role WHERE user = $user AND role = $role GROUP ALL")
        .bind(("user", user_id.clone()))
        .bind(("role", Role::Admin.as_str()))
        .await?;
    let admin_check: Option<CountResult> = result.take(0)?;
    Ok(CountResult::unwrap_or_zero(admin_check) > 0)
}
