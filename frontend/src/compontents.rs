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

#[component]
pub fn HabitItem(
    #[prop(into)] title: String,
    #[prop(into)] description: String,
    #[prop(into)] color_index: usize,
) -> impl IntoView {
    let bg_class = format!("habit-bg-{}", color_index % 5);
    
    view! {
        <div class=format!("habit-item {}", bg_class)>
            <h3 class="habit-item-title">{title}</h3>
            <p class="habit-item-desc">{description}</p>
        </div>
    }
}

#[component]
pub fn NewHabitModal(
    set_show_modal: WriteSignal<bool>,
    set_refresh_trigger: WriteSignal<()>,
) -> impl IntoView {
    let (new_habit_name, set_new_habit_name) = signal(String::new());
    let (new_habit_desc, set_new_habit_desc) = signal(String::new());
    let (is_loading, set_is_loading) = signal(false);
    let (has_error, set_has_error) = signal(false);

    let auth = use_context::<crate::api::client::AuthFlowClient>()
        .expect("AuthFlowClient should be provided in context");

    let on_add_habit = move |_| {
        let auth = auth.clone();
        set_is_loading.set(true);
        set_has_error.set(false);

        leptos::task::spawn_local(async move {
            let desc = new_habit_desc.get();
            let req = shared::model::habit::NewHabitReq {
                name: new_habit_name.get(),
                description: if desc.is_empty() { None } else { Some(desc) },
            };

            match auth.post::<_, shared::model::habit::HabitData>("/habit", &req).await {
                Ok(_) => {
                    set_show_modal.set(false);
                    set_refresh_trigger.set(());
                }
                Err(e) => {
                    leptos::logging::error!("Create habit error: {}", e);
                    set_has_error.set(true);
                }
            }
            set_is_loading.set(false);
        });
    };

    view! {
        <div class="modal-overlay">
            <div class="modal-card">
                <h2 class="modal-title">"New Habit"</h2>
                <div class="modal-inputs">
                    <MainInput
                        label="Name"
                        placeholder="Habit name"
                        input_type="text"
                        value=new_habit_name
                        set_value=set_new_habit_name
                        has_error=has_error.into()
                    />
                    <MainInput
                        label="Description (optional)"
                        placeholder="Habit description"
                        input_type="text"
                        value=new_habit_desc
                        set_value=set_new_habit_desc
                        has_error=has_error.into()
                    />
                </div>
                {move || if has_error.get() { Some(view! { <div class="error-message">"Failed to create habit"</div> }) } else { None }}
                <div class="modal-actions">
                    <button class="cancel-button" on:click=move |_| set_show_modal.set(false)>"Cancel"</button>
                    <AuthButton
                        text="Create"
                        loading_text="Saving..."
                        is_loading=is_loading.into()
                        on_click=Callback::new(on_add_habit)
                    />
                </div>
            </div>
        </div>
    }
}