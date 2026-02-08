use crate::views::{context::ViewContext, layout::base::base_layout};
use maud::{Markup, html};

pub fn server_error(ctx: &ViewContext, message: &str) -> Markup {
    let content = html! {
        h1 class="text-6xl mb-3" { "500" }
        p class="text-red-600" { (message) }
    };

    base_layout(ctx, "Server Error", "Server error", content)
}
