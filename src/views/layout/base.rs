use super::navigation;
use crate::{constants::cdn, paths, views::{components, context::ViewContext}};
use maud::{html, Markup, DOCTYPE};

pub fn base_layout(ctx: &ViewContext, title: &str, meta_description: &str, content: Markup) -> Markup {
    html! {
        (DOCTYPE)
        html lang="en" {
            head {
                meta charset="UTF-8";
                meta name="viewport" content="width=device-width, initial-scale=1.0";
                title { (title) " - " (ctx.site_name) }
                meta name="description" content=(meta_description);

                link rel="icon" type="image/svg+xml" href=(paths::static_files::FAVICON);

                script src=(cdn::TAILWIND_CSS_URL) {}

                script src=(cdn::HTMX_URL)
                    integrity=(cdn::HTMX_INTEGRITY)
                    crossorigin="anonymous" {}

                script src=(cdn::HYPERSCRIPT_URL) {}
            }
            body class="min-h-screen flex flex-col" {
                (navigation::navbar(ctx.current_user))
                main class="flex-grow container mx-auto px-4 py-8" {
                    (components::flash::flash(ctx.flash))
                    (content)
                }
            }
        }
    }
}
