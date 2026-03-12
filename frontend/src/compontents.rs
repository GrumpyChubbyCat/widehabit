use leptos::{ev, prelude::*};
use leptos::{IntoView, component, view};

#[component]
pub fn MainInput(
    #[prop(into)] label: String,
    #[prop(into)] placeholder: String,
    #[prop(into)] input_type: String,
    value: ReadSignal<String>,
    set_value: WriteSignal<String>,
    #[prop(optional)] has_error: Signal<bool>,
) -> impl IntoView {
    view! {
        <div class="input-group">
            <input
                type=input_type
                placeholder=placeholder
                class=move || if has_error.get() { "has-error" } else { "" }
                prop:value=value
                on:input=move |event| set_value.set(event_target_value(&event))
            />
            <label class="input-label">{label}</label>
        </div>
    }
}

#[component]
pub fn AuthButton(
    #[prop(into)] text: String,
    #[prop(into)] loading_text: String,
    is_loading: Signal<bool>,
    on_click: Callback<ev::MouseEvent>,
) -> impl IntoView {
    view! {
        <button
            class="sign-in-button"
            disabled=is_loading
            on:click=move |ev| on_click.run(ev)
        >
            {move || if is_loading.get() {loading_text.clone()} else {text.clone()} }
        </button>
    }
}

#[component]
pub fn IconPlus() -> impl IntoView {
    // Relative path to your Plus square.svg
    let svg = include_str!("../public/assets/icons/plus.svg");
    view! { <span class="icon-wrapper" inner_html=svg /> }
}

#[component]
pub fn IconSettings() -> impl IntoView {
    let svg = include_str!("../public/assets/icons/settings.svg");
    view! { <span class="icon-wrapper" inner_html=svg /> }
}