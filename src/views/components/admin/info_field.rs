use maud::{html, Markup};
use std::fmt::Display;

pub fn info_field(label: &str, value: impl Display) -> Markup {
    let value = value.to_string();
    html! {
        div {
            span class="text-gray-600" { (label) }
            span { (value) }
        }
    }
}

pub fn info_field_mono(label: &str, value: impl Display) -> Markup {
    let value = value.to_string();
    html! {
        div {
            span class="text-gray-600" { (label) }
            span class="font-mono text-xs" { (value) }
        }
    }
}
