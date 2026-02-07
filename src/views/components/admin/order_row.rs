use crate::{constants::css, views::helpers, models::admin::OrderListItem, paths};
use maud::{html, Markup};

pub fn order_row(order: &OrderListItem, show_user: bool) -> Markup {
    let status_class = helpers::payment_status_class(&order.payment_status);
    let status_text = order.payment_status.display_text();
    let date_display = helpers::format_datetime(order.created_at);

    html! {
        tr class="border-b" {
            td class="py-2 px-2" {
                a href=(paths::helpers::order_detail_path(&order.id))
                    class=(css::LINK)
                {
                    (order.order_number)
                }
            }
            @if show_user {
                td class="py-2 px-2 text-gray-600" { (order.user_email) }
            }
            td class="py-2 px-2 text-right" { "₩" (order.price_amount) }
            td class="py-2 px-2 text-center" {
                span class={"px-2 py-1 text-xs " (status_class)} {
                    (status_text)
                }
            }
            td class="py-2 px-2 text-center text-gray-600" { (date_display) }
            @if show_user {
                td class="py-2 px-2 text-center" {
                    a href=(paths::helpers::order_detail_path(&order.id))
                        class=(css::LINK_SM)
                    {
                        "View"
                    }
                }
            }
        }
    }
}
