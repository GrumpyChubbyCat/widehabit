use crate::api::client::AuthFlowClient;
use crate::components::icons::IconTrash;
use crate::components::{AuthButton, MainInput};
use chrono::{DateTime, NaiveTime, Utc};
use leptos::prelude::*;
use leptos::{component, view, IntoView};
use shared::model::habit::{HabitData, NewHabitReq, UpdateHabitRes};
use shared::model::log::{HabitLogData, NewHabitLogReq};
use shared::model::schedule::{ScheduleItemReq, ScheduleRes, SetScheduleReq};
use shared::model::{DayOfWeek, PagedResponse};
use uuid::Uuid;

#[component]
pub fn LogHabitModal(
    habit_id: Option<Uuid>,
    day_idx: usize,
    start_time_str: String,
    end_time_str: String,
    habits: LocalResource<PagedResponse<HabitData>>,
    on_close: Callback<()>,
    set_refresh_trigger: WriteSignal<()>,
) -> impl IntoView {
    let (selected_habit_id, set_selected_habit_id) = signal(habit_id);
    let (start_time, set_start_time) = signal(start_time_str);
    let (end_time, set_end_time) = signal(end_time_str);
    let (comment, set_comment) = signal(String::new());
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

    let on_save = move |_| {
        let auth = auth.clone();
        let habit_id = match selected_habit_id.get() {
            Some(id) => id,
            None => {
                set_has_error.set(true);
                return;
            }
        };

        set_is_loading.set(true);
        set_has_error.set(false);

        let start = start_time.get();
        let end = end_time.get();
        let comment_val = comment.get();

        leptos::task::spawn_local(async move {
            let start_parsed = NaiveTime::parse_from_str(&format!("{}:00", start), "%H:%M:%S")
                .or_else(|_| NaiveTime::parse_from_str(&start, "%H:%M"));
            let end_parsed = NaiveTime::parse_from_str(&format!("{}:00", end), "%H:%M:%S")
                .or_else(|_| NaiveTime::parse_from_str(&end, "%H:%M"));

            if let (Ok(s), Ok(e)) = (start_parsed, end_parsed) {
                // For now, we use today's date for the log, adjusted to the selected day_idx if needed.
                // In a real app, we'd calculate the exact date of the selected week.
                let log_date = Utc::now().date_naive();

                let req = NewHabitLogReq {
                    habit_schedule_id: None, // Optional
                    log_date: Some(log_date),
                    actual_start: Some(DateTime::<Utc>::from_naive_utc_and_offset(
                        log_date.and_time(s),
                        Utc,
                    )),
                    actual_end: Some(DateTime::<Utc>::from_naive_utc_and_offset(
                        log_date.and_time(e),
                        Utc,
                    )),
                    comment: if comment_val.is_empty() {
                        None
                    } else {
                        Some(comment_val)
                    },
                };

                let path = format!("/log/{}", habit_id);
                match auth.post::<_, HabitLogData>(&path, &req).await {
                    Ok(_) => {
                        on_close.run(());
                        set_refresh_trigger.set(());
                    }
                    Err(e) => {
                        leptos::logging::error!("Log habit error: {}", e);
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
                <h2 class="modal-title">"Log Habit"</h2>
                <p class="modal-subtitle">"Day: " {day_name}</p>

                <div class="modal-inputs">
                    {move || {
                        let h_data = habits.get();
                        match h_data {
                            Some(resp) if !resp.items.is_empty() => {
                                view! {
                                    <div class="input-group">
                                        <select
                                            class="modal-select"
                                            on:change=move |ev| {
                                                let val = event_target_value(&ev);
                                                if let Ok(id) = Uuid::parse_str(&val) {
                                                    set_selected_habit_id.set(Some(id));
                                                }
                                            }
                                            prop:value=move || selected_habit_id.get().map(|id| id.to_string()).unwrap_or_default()
                                        >
                                            <option value="" disabled=true selected=selected_habit_id.get().is_none()>"Select a habit"</option>
                                            {resp.items.into_iter().map(|h| {
                                                view! { <option value=h.habit_id.to_string()>{h.name}</option> }
                                            }).collect_view()}
                                        </select>
                                        <label class="input-label">"Habit"</label>
                                    </div>
                                }.into_any()
                            }
                            Some(_) => {
                                view! { <p class="error-message">"You have no habits yet. Create one first!"</p> }.into_any()
                            }
                            None => {
                                view! { <p>"Loading habits..."</p> }.into_any()
                            }
                        }
                    }}

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
                    <MainInput
                        label="Comment (optional)"
                        placeholder="How did it go?"
                        input_type="text"
                        value=comment
                        set_value=set_comment
                    />
                </div>

                {move || if has_error.get() { Some(view! { <div class="error-message">"Failed to log habit. Please check your inputs."</div> }) } else { None }}

                <div class="modal-actions">
                    <button class="cancel-button" on:click=move |_| on_close.run(())>"Cancel"</button>
                    <AuthButton
                        text="Log Time"
                        loading_text="Saving..."
                        is_loading=is_loading.into()
                        on_click=Callback::new(on_save)
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
pub fn LogoutModal(on_cancel: Callback<()>, on_confirm: Callback<()>) -> impl IntoView {
    view! {
        <div class="modal-overlay">
            <div class="modal-card">
                <h2 class="modal-title">"Logout?"</h2>
                <p style="margin-bottom: 24px;">"Are you sure you want to log out?"</p>
                <div class="modal-actions">
                    <button class="cancel-button" on:click=move |_| on_cancel.run(())>"No"</button>
                    <AuthButton
                        text="Yes, Logout"
                        loading_text="Wait..."
                        is_loading=false.into()
                        on_click=Callback::new(move |_| on_confirm.run(()))
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
    #[derive(Clone, Debug, PartialEq)]
    struct DaySchedule {
        day_idx: usize,
        slots: Vec<TimeSlot>,
    }

    #[derive(Clone, Debug, PartialEq)]
    struct TimeSlot {
        id: Uuid,
        start: String,
        end: String,
    }

    let (day_schedules, set_day_schedules) = signal({
        let mut schedules = Vec::new();
        for i in 0..7 {
            let slots = if i == day_idx {
                vec![TimeSlot {
                    id: Uuid::new_v4(),
                    start: start_time_str.clone(),
                    end: end_time_str.clone(),
                }]
            } else {
                vec![]
            };
            schedules.push(DaySchedule { day_idx: i, slots });
        }
        schedules
    });

    let (is_loading, set_is_loading) = signal(false);
    let (has_error, set_has_error) = signal(false);

    let auth =
        use_context::<AuthFlowClient>().expect("AuthFlowClient should be provided in context");

    let days_list = [
        "Monday",
        "Tuesday",
        "Wednesday",
        "Thursday",
        "Friday",
        "Saturday",
        "Sunday",
    ];

    let on_save = move |_| {
        let auth = auth.clone();
        set_is_loading.set(true);
        set_has_error.set(false);

        let schedules_data = day_schedules.get();

        leptos::task::spawn_local(async move {
            let mut schedules = Vec::new();

            for day_sched in schedules_data {
                let day_enum = match day_sched.day_idx {
                    0 => DayOfWeek::Monday,
                    1 => DayOfWeek::Tuesday,
                    2 => DayOfWeek::Wednesday,
                    3 => DayOfWeek::Thursday,
                    4 => DayOfWeek::Friday,
                    5 => DayOfWeek::Saturday,
                    _ => DayOfWeek::Sunday,
                };

                for slot in day_sched.slots {
                    let start_parsed = NaiveTime::parse_from_str(&format!("{}:00", slot.start), "%H:%M:%S")
                        .or_else(|_| NaiveTime::parse_from_str(&slot.start, "%H:%M"));
                    let end_parsed = NaiveTime::parse_from_str(&format!("{}:00", slot.end), "%H:%M:%S")
                        .or_else(|_| NaiveTime::parse_from_str(&slot.end, "%H:%M"));

                    if let (Ok(s), Ok(e)) = (start_parsed, end_parsed) {
                        schedules.push(ScheduleItemReq {
                            day: day_enum,
                            start_time: s,
                            end_time: e,
                        });
                    } else {
                        set_has_error.set(true);
                        set_is_loading.set(false);
                        return;
                    }
                }
            }

            if schedules.is_empty() {
                set_has_error.set(true);
                set_is_loading.set(false);
                return;
            }

            let req = SetScheduleReq {
                habit_id,
                schedules,
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
            set_is_loading.set(false);
        });
    };

    view! {
        <div class="modal-overlay">
            <div class="modal-card" style="width: 400px;">
                <h2 class="modal-title">"Schedule Habit"</h2>
                <p class="modal-subtitle">"Configure times for each day"</p>

                <div class="modal-inputs">
                    <div class="day-schedules-list">
                        {days_list.into_iter().enumerate().map(|(idx, name)| {
                            let day_sched = move || day_schedules.get()[idx].clone();
                            let has_slots = move || !day_sched().slots.is_empty();

                            view! {
                                <div class="day-schedule-item" class:active=has_slots>
                                    <div class="day-schedule-header">
                                        <span class="day-name">{name}</span>
                                        <button
                                            class="add-slot-mini-btn"
                                            on:click=move |_| {
                                                set_day_schedules.update(|schedules| {
                                                    schedules[idx].slots.push(TimeSlot {
                                                        id: Uuid::new_v4(),
                                                        start: "09:00".to_string(),
                                                        end: "10:00".to_string(),
                                                    });
                                                });
                                            }
                                        >
                                            "+ Add time"
                                        </button>
                                    </div>

                                    <div class="day-slots-list">
                                        {move || day_sched().slots.into_iter().map(|slot| {
                                            let slot_id = slot.id;
                                            view! {
                                                <div class="time-slot-row mini">
                                                    <div class="time-slot-inputs">
                                                        <input
                                                            type="time"
                                                            class="modal-time-input"
                                                            prop:value=slot.start
                                                            on:input=move |ev| {
                                                                let val = event_target_value(&ev);
                                                                set_day_schedules.update(|schedules| {
                                                                    if let Some(s) = schedules[idx].slots.iter_mut().find(|s| s.id == slot_id) {
                                                                        s.start = val;
                                                                    }
                                                                });
                                                            }
                                                        />
                                                        <span class="time-separator">"—"</span>
                                                        <input
                                                            type="time"
                                                            class="modal-time-input"
                                                            prop:value=slot.end
                                                            on:input=move |ev| {
                                                                let val = event_target_value(&ev);
                                                                set_day_schedules.update(|schedules| {
                                                                    if let Some(s) = schedules[idx].slots.iter_mut().find(|s| s.id == slot_id) {
                                                                        s.end = val;
                                                                    }
                                                                });
                                                            }
                                                        />
                                                    </div>
                                                    <button class="remove-slot-btn" on:click=move |_| {
                                                        set_day_schedules.update(|schedules| {
                                                            schedules[idx].slots.retain(|s| s.id != slot_id);
                                                        });
                                                    }>
                                                        "×"
                                                    </button>
                                                </div>
                                            }
                                        }).collect_view()}
                                    </div>
                                </div>
                            }
                        }).collect_view()}
                    </div>
                </div>

                {move || if has_error.get() { Some(view! { <div class="error-message">"Failed to schedule habit. Please check your inputs."</div> }) } else { None }}

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
