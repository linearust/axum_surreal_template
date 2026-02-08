use crate::views::{context::ViewContext, layout::base::base_layout};
use maud::{Markup, html};

pub fn not_found(ctx: &ViewContext) -> Markup {
    let content = html! {
        h1 class="text-6xl" { "404" }
    };

    base_layout(ctx, "Page Not Found", "Page not found", content)
}
