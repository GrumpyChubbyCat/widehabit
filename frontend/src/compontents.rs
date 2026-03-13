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
pub fn IconEdit() -> impl IntoView {
    let svg = include_str!("../public/assets/icons/pen.svg");
    view! { <span class="icon-wrapper" inner_html=svg /> }
}

#[component]
pub fn IconTrash() -> impl IntoView {
    let svg = include_str!("../public/assets/icons/trash.svg");
    view! { <span class="icon-wrapper" inner_html=svg /> }
}

#[component]
pub fn HabitItem(
    #[prop(into)] title: String,
    #[prop(into)] description: String,
    #[prop(into)] color_index: usize,
    #[prop(optional)] on_edit: Option<Callback<ev::MouseEvent>>,
) -> impl IntoView {
    let bg_class = format!("habit-bg-{}", color_index % 5);
    
    view! {
        <div class=format!("habit-item {}", bg_class)>
            <h3 class="habit-item-title">{title}</h3>
            <div class="habit-desc-row">
                <p class="habit-item-desc">{description}</p>
                <div class="habit-item-actions" on:click=move |ev| { if let Some(cb) = on_edit { cb.run(ev); } }>
                    <IconEdit />
                </div>
            </div>
        </div>
    }
}

#[component]
pub fn EditHabitModal(
    habit: shared::model::habit::HabitData,
    on_close: Callback<()>,
    set_refresh_trigger: WriteSignal<()>,
) -> impl IntoView {
    let (new_habit_name, set_new_habit_name) = signal(habit.name.clone());
    let (new_habit_desc, set_new_habit_desc) = signal(habit.description.clone().unwrap_or_default());
    let (is_loading, set_is_loading) = signal(false);
    let (has_error, set_has_error) = signal(false);
    let (show_delete_confirm, set_show_delete_confirm) = signal(false);

    let auth = use_context::<crate::api::client::AuthFlowClient>()
        .expect("AuthFlowClient should be provided in context");

    let auth_for_save = auth.clone();
    let on_save_habit = Callback::new(move |_| {
        let auth = auth_for_save.clone();
        let habit_id = habit.habit_id;
        set_is_loading.set(true);
        set_has_error.set(false);

        leptos::task::spawn_local(async move {
            let desc = new_habit_desc.get();
            let req = shared::model::habit::NewHabitReq {
                name: new_habit_name.get(),
                description: if desc.is_empty() { None } else { Some(desc) },
            };

            let path = format!("/habit/{}", habit_id);
            match auth.patch::<_, shared::model::habit::UpdateHabitRes>(&path, &req).await {
                Ok(_) => {
                    on_close.run(());
                    set_refresh_trigger.set(());
                }
                Err(e) => {
                    leptos::logging::error!("Update habit error: {}", e);
                    set_has_error.set(true);
                }
            }
            set_is_loading.set(false);
        });
    });

    view! {
        {move || if show_delete_confirm.get() {
            view! {
                <DeleteHabitModal
                    habit_id=habit.habit_id
                    on_cancel=Callback::new(move |_| set_show_delete_confirm.set(false))
                    on_success=Callback::new(move |_| {
                        on_close.run(());
                        set_refresh_trigger.set(());
                    })
                />
            }.into_any()
        } else {
            view! {
                <div class="modal-overlay">
                    <div class="modal-card">
                        <div class="modal-header">
                            <h2 class="modal-title">"Edit Habit"</h2>
                            <button class="delete-btn" on:click=move |_| set_show_delete_confirm.set(true)>
                                <IconTrash />
                            </button>
                        </div>
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
                        {move || if has_error.get() { Some(view! { <div class="error-message">"Failed to update habit"</div> }) } else { None }}
                        <div class="modal-actions">
                            <button class="cancel-button" on:click=move |_| on_close.run(())>"Cancel"</button>
                            <AuthButton
                                text="Save"
                                loading_text="Saving..."
                                is_loading=is_loading.into()
                                on_click=on_save_habit
                            />
                        </div>
                    </div>
                </div>
            }.into_any()
        }}
    }
}

#[component]
pub fn DeleteHabitModal(
    habit_id: uuid::Uuid,
    on_cancel: Callback<()>,
    on_success: Callback<()>,
) -> impl IntoView {
    let (is_loading, set_is_loading) = signal(false);
    let (has_error, set_has_error) = signal(false);

    let auth = use_context::<crate::api::client::AuthFlowClient>()
        .expect("AuthFlowClient should be provided in context");

    let on_delete_habit = Callback::new(move |_| {
        let auth = auth.clone();
        set_is_loading.set(true);
        set_has_error.set(false);

        leptos::task::spawn_local(async move {
            let path = format!("/habit/{}", habit_id);
            match auth.delete::<()>(&path).await {
                Ok(_) => {
                    on_success.run(());
                }
                Err(e) => {
                    leptos::logging::error!("Delete habit error: {}", e);
                    set_has_error.set(true);
                }
            }
            set_is_loading.set(false);
        });
    });

    view! {
        <div class="modal-overlay">
            <div class="modal-card">
                <h2 class="modal-title">"Delete Habit?"</h2>
                <p style="margin-bottom: 24px;">"Are you sure you want to delete this habit? This action cannot be undone."</p>
                {move || if has_error.get() { Some(view! { <div class="error-message">"Failed to delete habit"</div> }) } else { None }}
                <div class="modal-actions">
                    <button class="cancel-button" on:click=move |_| on_cancel.run(())>"No"</button>
                    <AuthButton
                        text="Yes, Delete"
                        loading_text="Deleting..."
                        is_loading=is_loading.into()
                        on_click=on_delete_habit
                    />
                </div>
            </div>
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