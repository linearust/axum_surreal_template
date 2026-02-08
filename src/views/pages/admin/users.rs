use crate::{
    constants::css,
    views::helpers,
    models::admin::UserListItem,
    models::pagination::PaginatedResult,
    paths,
    views::{components::admin::pagination, context::ViewContext, layout::base::base_layout},
};
use maud::{html, Markup};

pub fn users(ctx: &ViewContext, paginated: PaginatedResult<UserListItem>) -> Markup {
    let content = html! {
        div class="max-w-6xl mx-auto" {
            h1 class="text-xl mb-6" { "Users" }

            @if paginated.items.is_empty() {
                p class="text-gray-500 py-4" { "No users found" }
            } @else {
                table class="w-full text-sm" {
                    thead class="border-b" {
                        tr {
                            th class="text-left py-2 px-2" { "Email" }
                            th class="text-left py-2 px-2" { "Role" }
                            th class="text-center py-2 px-2" { "Signup Date" }
                            th class="text-center py-2 px-2" { "Orders" }
                            th class="text-right py-2 px-2" { "Total Spent" }
                            th class="text-center py-2 px-2" { "Actions" }
                        }
                    }
                    tbody {
                        @for user in &paginated.items {
                            (user_row(user))
                        }
                    }
                }

                (pagination(
                    paths::pages::admin::USERS,
                    paginated.page,
                    paginated.total_pages,
                    paginated.has_prev(),
                    paginated.has_next(),
                ))
            }
        }
    };

    base_layout(ctx, "Users", "Browse all users", content)
}

fn user_row(user: &UserListItem) -> Markup {
    let date_display = helpers::format_datetime(user.created_at);

    html! {
        tr class="border-b" {
            td class="py-2 px-2" { (user.email) }
            td class="py-2 px-2" {
                @if user.is_admin {
                    span class="px-2 py-1 text-xs bg-indigo-100 text-indigo-800" {
                        "Admin"
                    }
                }
            }
            td class="py-2 px-2 text-center text-gray-600" { (date_display) }
            td class="py-2 px-2 text-center" { (user.order_count) }
            td class="py-2 px-2 text-right" { "₩" (helpers::format_price(user.total_spent)) }
            td class="py-2 px-2 text-center" {
                a href=(paths::helpers::user_detail_path(&user.id))
                    class=(css::LINK_SM)
                {
                    "View"
                }
            }
        }
    }
}
