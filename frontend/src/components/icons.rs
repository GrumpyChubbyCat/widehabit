use leptos::prelude::*;
use leptos::{component, view, IntoView};

#[component]
pub fn IconPlus() -> impl IntoView {
    let svg = include_str!("../../public/assets/icons/plus.svg");
    view! { <span class="icon-wrapper" inner_html=svg /> }
}

#[component]
pub fn IconSettings() -> impl IntoView {
    let svg = include_str!("../../public/assets/icons/settings.svg");
    view! { <span class="icon-wrapper" inner_html=svg /> }
}

#[component]
pub fn IconEdit() -> impl IntoView {
    let svg = include_str!("../../public/assets/icons/pen.svg");
    view! { <span class="icon-wrapper" inner_html=svg /> }
}

#[component]
pub fn IconTrash() -> impl IntoView {
    let svg = include_str!("../../public/assets/icons/trash.svg");
    view! { <span class="icon-wrapper" inner_html=svg /> }
}
