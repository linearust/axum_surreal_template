use crate::{
    constants::css,
    views::helpers,
    models::admin::OrderDetail,
    paths,
    views::{components::admin::{info_field, info_field_mono}, context::ViewContext, layout::base::base_layout},
};
use maud::{html, Markup};

pub fn order_detail(ctx: &ViewContext, order: OrderDetail) -> Markup {
    let content = html! {
        div class="max-w-6xl mx-auto" {
            div class="mb-4" {
                a href=(paths::pages::admin::ORDERS)
                    class=(css::LINK_SM)
                {
                    "← Back to Orders"
                }
            }

            h1 class="text-xl mb-6" { "Order Details" }

            div class="mb-8 border p-4" {
                h2 class="text-lg mb-3" { "Order Information" }
                div class="space-y-2 text-sm" {
                    (info_field("Order Number: ", &order.order_number))
                    (info_field_mono("Order ID: ", &order.id))
                    div {
                        span class="text-gray-600" { "Status: " }
                        span class={"px-2 py-1 text-xs " (helpers::payment_status_class(&order.payment_status))} {
                            (order.payment_status.display_text())
                        }
                    }
                    (info_field("Amount: ", format!("₩{}", helpers::format_price(order.price_amount))))
                    (info_field("Created: ", helpers::format_datetime(order.created_at)))
                    @if let Some(paid_at) = order.paid_at {
                        (info_field("Paid: ", helpers::format_datetime(paid_at)))
                    }
                }
            }

            div class="mb-8 border p-4" {
                h2 class="text-lg mb-3" { "User Information" }
                div class="space-y-2 text-sm" {
                    div {
                        span class="text-gray-600" { "Email: " }
                        a href=(paths::with_param(paths::pages::admin::USER_DETAIL, "user_id", &order.user))
                            class=(css::LINK)
                        {
                            (order.user_email)
                        }
                    }
                    (info_field_mono("User ID: ", &order.user))
                }
            }

            @if order.payment_key.is_some() {
                div class="mb-8 border p-4" {
                    h2 class="text-lg mb-3" { "Payment Information" }
                    div class="space-y-2 text-sm" {
                        @if let Some(payment_key) = &order.payment_key {
                            (info_field_mono("Payment Key: ", payment_key))
                        }
                    }
                }
            }

            div class="border p-4" {
                h2 class="text-lg mb-3" { "File Information" }
                div class="space-y-2 text-sm" {
                    (info_field("Filename: ", &order.filename))
                    (info_field("Character Count: ", order.text_length))
                    (info_field("Price Calculation: ", format!("{} characters × ₩1 = ₩{}", order.text_length, helpers::format_price(order.price_amount))))
                }
            }
        }
    };

    base_layout(ctx, "Order Details", &format!("Details for order {}", order.order_number), content)
}
