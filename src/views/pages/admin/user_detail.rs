use crate::{
    auth::CurrentUser,
    constants::css,
    session::FlashMessage,
    views::helpers,
    models::admin::{OrderListItem, PaginatedResult, UserListItem},
    paths,
    views::{components::admin::{info_field, info_field_mono, order_row, pagination}, layout::base::base_layout},
};
use maud::{html, Markup};

pub fn user_detail(
    current_user: &CurrentUser,
    flash: Option<&FlashMessage>,
    site_name: &str,
    user: UserListItem,
    paginated_orders: PaginatedResult<OrderListItem>,
) -> Markup {
    let content = html! {
        div class="max-w-6xl mx-auto" {
            div class="mb-4" {
                a href=(paths::pages::admin::USERS)
                    class=(css::LINK_SM)
                {
                    "← Back to Users"
                }
            }

            h1 class="text-xl mb-6" { "User Details" }

            (user_info_section(&user))
            (admin_role_section(&user))
            (user_orders_section(&user, &paginated_orders))
        }
    };

    base_layout(
        current_user,
        flash,
        site_name,
        "User Details",
        &format!("Details for {}", user.email),
        content,
    )
}


fn user_info_section(user: &UserListItem) -> Markup {
    html! {
        div class="mb-8 border p-4" {
            h2 class="text-lg mb-3" { "User Information" }
            div class="space-y-2 text-sm" {
                (info_field("Email: ", &user.email))
                (info_field_mono("User ID: ", &user.id))
                (info_field("Signup Date: ", helpers::format_datetime(user.created_at)))
                (info_field("Total Orders: ", user.order_count))
                (info_field("Total Spent: ", format!("₩{}", helpers::format_price(user.total_spent))))
            }
        }
    }
}

fn admin_role_section(user: &UserListItem) -> Markup {
    html! {
        div class="mb-8 border p-4" {
            h2 class="text-lg mb-3" { "Admin Role" }
            @if user.is_admin {
                div class="mb-3" {
                    span class="px-2 py-1 text-xs bg-indigo-100 text-indigo-800" {
                        "Admin"
                    }
                }
                form method="post"
                    action=(paths::with_param(paths::actions::admin::REVOKE_ROLE, "user_id", &user.id))
                    hx-delete=(paths::with_param(paths::actions::admin::REVOKE_ROLE, "user_id", &user.id))
                    hx-target="body"
                    hx-swap="outerHTML"
                {
                    button type="submit"
                        class="text-sm text-red-600 hover:underline"
                    {
                        "Revoke Admin Role"
                    }
                }
            } @else {
                p class="text-sm text-gray-600 mb-3" { "This user is not an admin" }
                form method="post"
                    action=(paths::with_param(paths::forms::admin::GRANT_ROLE, "user_id", &user.id))
                {
                    button type="submit"
                        class="text-sm text-indigo-600 hover:underline"
                    {
                        "Grant Admin Role"
                    }
                }
            }
        }
    }
}

fn user_orders_section(user: &UserListItem, paginated_orders: &PaginatedResult<OrderListItem>) -> Markup {
    html! {
        div {
            h2 class="text-lg mb-3" { "Orders" }
            @if paginated_orders.items.is_empty() {
                p class="text-gray-500 py-4" { "No orders yet" }
            } @else {
                table class="w-full text-sm" {
                    thead class="border-b" {
                        tr {
                            th class="text-left py-2 px-2" { "Order #" }
                            th class="text-right py-2 px-2" { "Amount" }
                            th class="text-center py-2 px-2" { "Status" }
                            th class="text-center py-2 px-2" { "Date" }
                        }
                    }
                    tbody {
                        @for order in &paginated_orders.items {
                            (order_row(order, false))
                        }
                    }
                }

                (pagination(
                    &paths::with_param(paths::pages::admin::USER_DETAIL, "user_id", &user.id),
                    paginated_orders.page,
                    paginated_orders.total_pages,
                    paginated_orders.has_prev(),
                    paginated_orders.has_next(),
                ))
            }
        }
    }
}
