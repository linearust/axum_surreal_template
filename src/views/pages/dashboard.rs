use crate::{
    constants::css,
    views::helpers,
    models::order::OrderSummary,
    paths,
    views::{context::ViewContext, layout::base::base_layout},
};
use maud::{html, Markup};

pub fn dashboard(
    ctx: &ViewContext,
    recent_orders: Vec<OrderSummary>,
) -> Markup {
    let content = html! {
        div class="max-w-4xl mx-auto" {
            h1 class="text-xl mb-3" { "Orders" }

            @if recent_orders.is_empty() {
                p class="text-gray-500 py-4" { "No orders yet" }
            } @else {
                div class="overflow-x-auto" {
                    table class="w-full text-sm" {
                        thead class="border-b" {
                            tr {
                                th class="text-left py-2 px-2" { "Order #" }
                                th class="text-right py-2 px-2" { "Price" }
                                th class="text-center py-2 px-2" { "Status" }
                                th class="text-center py-2 px-2" { "Date" }
                            }
                        }
                        tbody {
                            @for order in recent_orders {
                                (order_row(&order))
                            }
                        }
                    }
                }
            }
        }
    };

    base_layout(ctx, "Orders", "Your order history", content)
}

fn order_row(order: &OrderSummary) -> Markup {
    let status_class = helpers::payment_status_class(&order.payment_status);
    let status_text = order.payment_status.display_text();
    let date_display = helpers::format_datetime(order.created_at);

    html! {
        tr class="border-b" {
            td class="py-2 px-2" {
                a href=(paths::helpers::quote_path(&order.id))
                    class=(css::LINK)
                {
                    (order.order_number)
                }
            }
            td class="py-2 px-2 text-right" { "₩" (order.price_amount) }
            td class="py-2 px-2 text-center" {
                span class={"px-2 py-1 text-xs " (status_class)} {
                    (status_text)
                }
            }
            td class="py-2 px-2 text-center text-gray-600" { (date_display) }
        }
    }
}
