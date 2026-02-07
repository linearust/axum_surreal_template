use crate::{
    constants::errors,
    data::errors::DataError,
    db::DB,
    models::{
        admin::{AdminStats, OrderDetail, OrderListItem, UserListItem},
        order::PaymentStatus,
        pagination::{self, PaginatedResult},
        OrderId, Role, UserId,
    },
};

use super::shared::{CountResult, SumResult, IS_ADMIN_SUBQUERY, ORDER_COUNT_SUBQUERY, TOTAL_SPENT_SUBQUERY};

// ───────────────────────────────────────────────────────────────────────────────
// Admin Stats
// ───────────────────────────────────────────────────────────────────────────────

pub async fn get_admin_stats() -> Result<AdminStats, DataError> {
    let mut result = DB
        .query(
            r#"
            SELECT count() as count FROM user GROUP ALL;
            SELECT count() as count FROM order WHERE payment_status = $paid_status GROUP ALL;
            SELECT math::sum(price_amount) as total FROM order WHERE payment_status = $paid_status GROUP ALL;
            SELECT count() as count FROM order WHERE payment_status = $paid_status AND created_at >= time::now() - 7d GROUP ALL;
            "#,
        )
        .bind(("paid_status", PaymentStatus::Paid.as_str()))
        .await?;

    let total_users: Option<CountResult> = result.take(0)?;
    let total_orders: Option<CountResult> = result.take(1)?;
    let total_revenue: Option<SumResult> = result.take(2)?;
    let orders_last_7_days: Option<CountResult> = result.take(3)?;

    Ok(AdminStats {
        total_users: CountResult::unwrap_or_zero(total_users),
        total_orders: CountResult::unwrap_or_zero(total_orders),
        total_revenue: SumResult::unwrap_or_zero(total_revenue) as i32,
        orders_last_7_days: CountResult::unwrap_or_zero(orders_last_7_days),
    })
}

// ───────────────────────────────────────────────────────────────────────────────
// Users (Paginated)
// ───────────────────────────────────────────────────────────────────────────────

pub async fn get_users_paginated(
    page: i64,
    per_page: i64,
) -> Result<PaginatedResult<UserListItem>, DataError> {
    let offset = pagination::offset(page, per_page);

    let mut result = DB
        .query(format!(
            "SELECT id, email, created_at, {IS_ADMIN_SUBQUERY}, {ORDER_COUNT_SUBQUERY}, {TOTAL_SPENT_SUBQUERY}
            FROM user ORDER BY created_at DESC LIMIT $limit START $offset;
            SELECT count() as count FROM user GROUP ALL;",
        ))
        .bind(("role", Role::Admin.as_str()))
        .bind(("paid", PaymentStatus::Paid.as_str()))
        .bind(("limit", per_page))
        .bind(("offset", offset))
        .await?;

    let users: Vec<UserListItem> = result.take(0)?;
    let total: Option<CountResult> = result.take(1)?;
    let total_count = CountResult::unwrap_or_zero(total);

    Ok(PaginatedResult::new(users, total_count, page, per_page))
}

// ───────────────────────────────────────────────────────────────────────────────
// User Detail
// ───────────────────────────────────────────────────────────────────────────────

pub async fn get_user_detail(user_id: &UserId) -> Result<UserListItem, DataError> {
    let mut result = DB
        .query(format!(
            "SELECT id, email, created_at, {IS_ADMIN_SUBQUERY}, {ORDER_COUNT_SUBQUERY}, {TOTAL_SPENT_SUBQUERY}
            FROM user WHERE id = $user_id",
        ))
        .bind(("user_id", user_id.clone().into_record_id()))
        .bind(("role", Role::Admin.as_str()))
        .bind(("paid", PaymentStatus::Paid.as_str()))
        .await?;

    let user: Option<UserListItem> = result.take(0)?;
    user.ok_or(DataError::NotFound(errors::USER_NOT_FOUND))
}

// ───────────────────────────────────────────────────────────────────────────────
// User Orders (Paginated)
// ───────────────────────────────────────────────────────────────────────────────

pub async fn get_user_orders_paginated(
    user_id: &UserId,
    page: i64,
    per_page: i64,
) -> Result<PaginatedResult<OrderListItem>, DataError> {
    let offset = pagination::offset(page, per_page);

    let mut result = DB
        .query(
            r#"
            SELECT id, order_number, user_email, price_amount, payment_status, created_at
            FROM order
            WHERE user = $user
            ORDER BY created_at DESC
            LIMIT $limit START $offset;
            SELECT count() as count FROM order WHERE user = $user GROUP ALL;
            "#,
        )
        .bind(("user", user_id.clone().into_record_id()))
        .bind(("limit", per_page))
        .bind(("offset", offset))
        .await?;

    let orders: Vec<OrderListItem> = result.take(0)?;
    let total: Option<CountResult> = result.take(1)?;
    let total_count = CountResult::unwrap_or_zero(total);

    Ok(PaginatedResult::new(orders, total_count, page, per_page))
}

// ───────────────────────────────────────────────────────────────────────────────
// Orders (Paginated, with optional status filter)
// ───────────────────────────────────────────────────────────────────────────────

fn build_status_filter_clause(status_filter: &Option<PaymentStatus>) -> &'static str {
    match status_filter {
        Some(_) => "WHERE payment_status = $status",
        None => "",
    }
}

pub async fn get_orders_paginated(
    status_filter: Option<PaymentStatus>,
    page: i64,
    per_page: i64,
) -> Result<PaginatedResult<OrderListItem>, DataError> {
    let offset = pagination::offset(page, per_page);
    let where_clause = build_status_filter_clause(&status_filter);

    let query = format!(
        r#"
        SELECT id, order_number, user_email, price_amount, payment_status, created_at
        FROM order
        {where_clause}
        ORDER BY created_at DESC
        LIMIT $limit START $offset;
        SELECT count() as count FROM order {where_clause} GROUP ALL;
        "#
    );

    let mut result = DB
        .query(&query)
        .bind(("status", status_filter.map(|s| s.as_str().to_string())))
        .bind(("limit", per_page))
        .bind(("offset", offset))
        .await?;

    let orders: Vec<OrderListItem> = result.take(0)?;
    let total: Option<CountResult> = result.take(1)?;
    let total_count = CountResult::unwrap_or_zero(total);

    Ok(PaginatedResult::new(orders, total_count, page, per_page))
}

// ───────────────────────────────────────────────────────────────────────────────
// Order Detail
// ───────────────────────────────────────────────────────────────────────────────

pub async fn get_order_detail(order_id: &OrderId) -> Result<OrderDetail, DataError> {
    let mut result = DB
        .query(
            "SELECT id, order_number, user, user_email, price_amount, payment_status, created_at, paid_at, payment_key, filename, text_length
             FROM order
             WHERE id = $order_id",
        )
        .bind(("order_id", order_id.clone().into_record_id()))
        .await?;

    let order: Option<OrderDetail> = result.take(0)?;
    order.ok_or(DataError::NotFound(errors::ORDER_NOT_FOUND))
}
