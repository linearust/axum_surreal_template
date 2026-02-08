use crate::{paths, views::{components::form::submit_button, context::ViewContext, layout::base::base_layout}};
use maud::{Markup, html};

pub fn text_analyzer(ctx: &ViewContext) -> Markup {
    let content = html! {
        div class="max-w-lg mx-auto" {
            h1 class="text-xl mb-3" { "Text Analyzer" }

            form method="post" action=(paths::forms::TEXT_ANALYZER) enctype="multipart/form-data" class="space-y-3" {
                div {
                    label for="file" class="block text-sm mb-1" {
                        "Text File"
                    }
                    input
                        type="file"
                        id="file"
                        name="file"
                        accept=".txt"
                        required
                        class="w-full px-3 py-2 border";
                }

                (submit_button("Get Quote"))
            }
        }
    };

    base_layout(ctx, "Text Analyzer", "Upload files for text analysis", content)
}
