use crate::api::client::AuthFlowClient;
use crate::components::icons::IconTrash;
use crate::components::{AuthButton, MainInput};
use chrono::NaiveTime;
use leptos::prelude::*;
use leptos::{component, view, IntoView};
use shared::model::habit::{HabitData, NewHabitReq, UpdateHabitRes};
use shared::model::schedule::{ScheduleItemReq, ScheduleRes, SetScheduleReq};
use shared::model::DayOfWeek;
use uuid::Uuid;

#[component]
pub fn NewHabitModal(
    set_show_modal: WriteSignal<bool>,
    set_refresh_trigger: WriteSignal<()>,
) -> impl IntoView {
    let (new_habit_name, set_new_habit_name) = signal(String::new());
    let (new_habit_desc, set_new_habit_desc) = signal(String::new());
    let (is_loading, set_is_loading) = signal(false);
    let (has_error, set_has_error) = signal(false);

    let auth =
        use_context::<AuthFlowClient>().expect("AuthFlowClient should be provided in context");

    let on_add_habit = move |_| {
        let auth = auth.clone();
        set_is_loading.set(true);
        set_has_error.set(false);

        leptos::task::spawn_local(async move {
            let desc = new_habit_desc.get();
            let req = NewHabitReq {
                name: new_habit_name.get(),
                description: if desc.is_empty() { None } else { Some(desc) },
            };

            match auth.post::<_, HabitData>("/habit", &req).await {
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
                <h2 class="modal-title" style="margin-bottom: 24px;">"New Habit"</h2>

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

#[component]
pub fn EditHabitModal(
    habit: HabitData,
    on_close: Callback<()>,
    set_refresh_trigger: WriteSignal<()>,
) -> impl IntoView {
    let (new_habit_name, set_new_habit_name) = signal(habit.name.clone());
    let (new_habit_desc, set_new_habit_desc) =
        signal(habit.description.clone().unwrap_or_default());
    let (is_loading, set_is_loading) = signal(false);
    let (has_error, set_has_error) = signal(false);
    let (show_delete_confirm, set_show_delete_confirm) = signal(false);

    let auth =
        use_context::<AuthFlowClient>().expect("AuthFlowClient should be provided in context");

    let auth_for_save = auth.clone();
    let on_save_habit = Callback::new(move |_| {
        let auth = auth_for_save.clone();
        let habit_id = habit.habit_id;
        set_is_loading.set(true);
        set_has_error.set(false);

        leptos::task::spawn_local(async move {
            let desc = new_habit_desc.get();
            let req = NewHabitReq {
                name: new_habit_name.get(),
                description: if desc.is_empty() { None } else { Some(desc) },
            };

            let path = format!("/habit/{}", habit_id);
            match auth.patch::<_, UpdateHabitRes>(&path, &req).await {
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
                <div class="modal-header" style="margin-bottom: 24px;">
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
    habit_id: Uuid,
    on_cancel: Callback<()>,
    on_success: Callback<()>,
) -> impl IntoView {
    let (is_loading, set_is_loading) = signal(false);
    let (has_error, set_has_error) = signal(false);

    let auth =
        use_context::<AuthFlowClient>().expect("AuthFlowClient should be provided in context");

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
pub fn ScheduleHabitModal(
    habit_id: Uuid,
    day_idx: usize,
    start_time_str: String,
    end_time_str: String,
    on_close: Callback<()>,
    set_refresh_trigger: WriteSignal<()>,
) -> impl IntoView {
    let (start_time, set_start_time) = signal(start_time_str);
    let (end_time, set_end_time) = signal(end_time_str);
    let (is_loading, set_is_loading) = signal(false);
    let (has_error, set_has_error) = signal(false);

    let auth =
        use_context::<AuthFlowClient>().expect("AuthFlowClient should be provided in context");

    let day_name = [
        "Monday",
        "Tuesday",
        "Wednesday",
        "Thursday",
        "Friday",
        "Saturday",
        "Sunday",
    ][day_idx];
    let day_enum = match day_idx {
        0 => DayOfWeek::Monday,
        1 => DayOfWeek::Tuesday,
        2 => DayOfWeek::Wednesday,
        3 => DayOfWeek::Thursday,
        4 => DayOfWeek::Friday,
        5 => DayOfWeek::Saturday,
        _ => DayOfWeek::Sunday,
    };

    let on_save = move |_| {
        let auth = auth.clone();
        set_is_loading.set(true);
        set_has_error.set(false);

        let start = start_time.get();
        let end = end_time.get();

        leptos::task::spawn_local(async move {
            // Parse times
            let start_parsed = NaiveTime::parse_from_str(&format!("{}:00", start), "%H:%M:%S")
                .or_else(|_| NaiveTime::parse_from_str(&start, "%H:%M"));
            let end_parsed = NaiveTime::parse_from_str(&format!("{}:00", end), "%H:%M:%S")
                .or_else(|_| NaiveTime::parse_from_str(&end, "%H:%M"));

            if let (Ok(s), Ok(e)) = (start_parsed, end_parsed) {
                let req = SetScheduleReq {
                    habit_id,
                    schedules: vec![ScheduleItemReq {
                        day: day_enum,
                        start_time: s,
                        end_time: e,
                    }],
                };

                match auth.put::<_, ScheduleRes>("/schedule", &req).await {
                    Ok(_) => {
                        on_close.run(());
                        set_refresh_trigger.set(());
                    }
                    Err(e) => {
                        leptos::logging::error!("Schedule habit error: {}", e);
                        set_has_error.set(true);
                    }
                }
            } else {
                set_has_error.set(true);
            }
            set_is_loading.set(false);
        });
    };

    view! {
        <div class="modal-overlay">
            <div class="modal-card">
                <h2 class="modal-title">"Schedule Habit"</h2>
                <p class="modal-subtitle">"Day: " {day_name}</p>

                <div class="modal-inputs">
                    <MainInput
                        label="Start Time"
                        placeholder="08:00"
                        input_type="time"
                        value=start_time
                        set_value=set_start_time
                        has_error=has_error.into()
                    />
                    <MainInput
                        label="End Time"
                        placeholder="09:00"
                        input_type="time"
                        value=end_time
                        set_value=set_end_time
                        has_error=has_error.into()
                    />
                </div>

                {move || if has_error.get() { Some(view! { <div class="error-message">"Failed to schedule habit. Ensure start time is before end time."</div> }) } else { None }}

                <div class="modal-actions">
                    <button class="cancel-button" on:click=move |_| on_close.run(())>"Cancel"</button>
                    <AuthButton
                        text="Schedule"
                        loading_text="Saving..."
                        is_loading=is_loading.into()
                        on_click=Callback::new(on_save)
                    />
                </div>
            </div>
        </div>
    }
}
