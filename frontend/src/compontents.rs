use crate::api::client::AuthFlowClient;
use chrono::NaiveTime;
use leptos::{component, view, IntoView};
use leptos::{ev, prelude::*};
use shared::model::habit::{HabitData, NewHabitReq, UpdateHabitRes};
use shared::model::schedule::{ScheduleItemReq, ScheduleRes, SetScheduleReq};
use shared::model::{DayOfWeek, PagedResponse};
use uuid::Uuid;

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
pub fn CalendarHabitItem(
    #[prop(into)] title: String,
    #[prop(into)] color_index: usize,
) -> impl IntoView {
    let bg_class = format!("habit-bg-{}", color_index % 5);

    view! {
        <div class=format!("calendar-habit-item {}", bg_class)>
            <span class="habit-item-title">{title}</span>
        </div>
    }
}

#[component]
pub fn HabitItem(
    #[prop(into)] habit_id: Uuid,
    #[prop(into)] title: String,
    #[prop(into)] description: String,
    #[prop(into)] color_index: usize,
    #[prop(optional)] on_edit: Option<Callback<ev::MouseEvent>>,
) -> impl IntoView {
    let bg_class = format!("habit-bg-{}", color_index % 5);

    view! {
        <div
            class=format!("habit-item {}", bg_class)
            draggable="true"
            on:dragstart=move |ev| {
                if let Some(dt) = ev.data_transfer() {
                    let _ = dt.set_data("text/plain", &habit_id.to_string());
                }
            }
        >
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

#[component]
pub fn CalendarCell(
    day_idx: usize,
    day_enum: DayOfWeek,
    time: String,
    next_time: String,
    schedules: LocalResource<ScheduleRes>,
    habits: LocalResource<PagedResponse<HabitData>>,
    set_schedule_modal_info: WriteSignal<Option<(Uuid, usize, String, String)>>,
) -> impl IntoView {
    let time_inner_for_drop = time.clone();
    let time_inner_for_view = time.clone();
    let next_time_inner = next_time.clone();

    view! {
        <div class="grid-cell"
            on:dragover=move |ev| ev.prevent_default()
            on:drop=move |ev| {
                ev.prevent_default();
                if let Some(dt) = ev.data_transfer() {
                    if let Ok(habit_id_str) = dt.get_data("text/plain") {
                        if let Ok(habit_id) = Uuid::parse_str(&habit_id_str) {
                            set_schedule_modal_info.set(Some((habit_id, day_idx, time_inner_for_drop.clone(), next_time_inner.clone())));
                        }
                    }
                }
            }
        >
            {move || {
                let s_data = schedules.get();
                let h_data = habits.get();
                let time_inner = time_inner_for_view.clone();
                if let (Some(s_resp), Some(h_resp)) = (s_data, h_data) {
                    s_resp.schedules.iter()
                        .filter(|s| {
                            let s_time = s.start_time.format("%H:%M").to_string();
                            s.day == day_enum && s_time == time_inner
                        })
                        .map(|s| {
                            let habit = h_resp.items.iter().find(|h| h.habit_id == s.habit_id);
                            let color_index = h_resp.items.iter().position(|h| h.habit_id == s.habit_id).unwrap_or(0);
                            let title = habit.map(|h| h.name.clone()).unwrap_or_else(|| "Unknown".to_string());
                            view! {
                                <CalendarHabitItem
                                    title=title
                                    color_index=color_index
                                />
                            }.into_any()
                        }).collect::<Vec<_>>()
                } else {
                    vec![]
                }
            }}
        </div>
    }
}

#[component]
pub fn CalendarGrid(
    days: Vec<&'static str>,
    times: Vec<&'static str>,
    schedules: LocalResource<ScheduleRes>,
    habits: LocalResource<PagedResponse<HabitData>>,
    set_schedule_modal_info: WriteSignal<Option<(Uuid, usize, String, String)>>,
) -> impl IntoView {
    let times_for_grid = times.clone();

    view! {
        <main class="calendar-view">
            <div class="calendar-grid">
                // Day headers
                <div></div> // Spacer for the time column
                {days.into_iter().map(|day| view! {
                    <div class="day-header">{day}</div>
                }).collect_view()}

                // Time rows and cells
                {times_for_grid.into_iter().enumerate().map(move |(time_idx, time)| {
                    let time_clone = time.to_string();
                    let next_time = times.get(time_idx + 1).cloned().unwrap_or("23:59");

                    view! {
                        <div class="time-label">{time}</div>
                        { (0..7).map(move |day_idx| {
                            let day_enum = match day_idx {
                                0 => DayOfWeek::Monday,
                                1 => DayOfWeek::Tuesday,
                                2 => DayOfWeek::Wednesday,
                                3 => DayOfWeek::Thursday,
                                4 => DayOfWeek::Friday,
                                5 => DayOfWeek::Saturday,
                                _ => DayOfWeek::Sunday,
                            };

                            view! {
                                <CalendarCell
                                    day_idx=day_idx
                                    day_enum=day_enum
                                    time=time_clone.clone()
                                    next_time=next_time.to_string()
                                    schedules=schedules
                                    habits=habits
                                    set_schedule_modal_info=set_schedule_modal_info
                                />
                            }
                        }).collect_view() }
                    }
                }).collect_view()}
            </div>
        </main>
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
