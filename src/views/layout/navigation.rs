use crate::{auth::CurrentUser, constants::css, paths};
use maud::{html, Markup};

pub fn navbar(current_user: &CurrentUser) -> Markup {
    html! {
        header class="border-b" {
            nav class="container mx-auto px-4 py-4" {
                div class="flex justify-between items-center" {
                    div class="flex gap-4" {
                        a href=(paths::pages::ROOT) class=(css::NAV_LINK) { "Home" }
                        @match current_user {
                            CurrentUser::Authenticated { .. } => {
                                a href=(paths::pages::DASHBOARD) class=(css::NAV_LINK) { "Dashboard" }
                                a href=(paths::pages::TEXT_ANALYZER) class=(css::NAV_LINK) { "Text Analyzer" }
                                a href=(paths::pages::TODOS) class=(css::NAV_LINK) { "Todos" }
                            }
                            CurrentUser::Guest => {}
                        }
                    }
                    div class="flex gap-4 items-center" {
                        @match current_user {
                            CurrentUser::Authenticated { .. } => {
                                @if current_user.is_admin() {
                                    a href=(paths::pages::admin::HOME) class=(css::NAV_LINK) { "Admin" }
                                }
                                form method="post" action=(paths::actions::SIGN_OUT) class="inline" {
                                    button type="submit" class=(css::NAV_LINK) { "Sign Out" }
                                }
                            }
                            CurrentUser::Guest => {
                                a href=(paths::pages::SIGN_IN) class=(css::NAV_LINK) { "Sign In" }
                            }
                        }
                    }
                }
            }
        }
    }
}