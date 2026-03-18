pub mod icons;
pub mod modals;

use crate::api::client::AuthFlowClient;
use crate::components::icons::IconEdit;
use leptos::prelude::*;
use leptos::{component, ev, view, IntoView, logging};
use shared::model::habit::HabitData;
use shared::model::log::HabitStats;
use shared::model::schedule::ScheduleRes;
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
pub fn AuthTextLink(
    #[prop(into)] text: String,
    on_click: Callback<ev::MouseEvent>,
) -> impl IntoView {
    view! {
        <button
            type="button"
            class="auth-text-link"
            on:click=move |ev| on_click.run(ev)
        >
            {text}
        </button>
    }
}

#[component]
pub fn CalendarHabitItem(
    #[prop(into)] habit_id: Uuid,
    #[prop(into)] title: String,
    #[prop(into)] color_index: usize,
    #[prop(optional)] hide_text: bool,
    #[prop(into)] day_idx: usize,
    #[prop(into)] start_time: String,
    #[prop(into)] end_time: String,
    set_log_modal_info: WriteSignal<Option<(Option<Uuid>, usize, String, String)>>,
) -> impl IntoView {
    let bg_class = format!("habit-bg-{}", color_index % 5);
    let tooltip = format!("{} ({} - {})", title, start_time, end_time);

    view! {
        <div
            class=format!("calendar-habit-item {}", bg_class)
            on:click=move |ev| {
                ev.stop_propagation();
                set_log_modal_info.set(Some((Some(habit_id), day_idx, start_time.clone(), end_time.clone())));
            }
        >
            <div class="custom-tooltip">{tooltip}</div>
            {move || (!hide_text).then(|| view! { <span class="habit-item-title">{title.clone()}</span> })}
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
    let auth = use_context::<AuthFlowClient>().expect("AuthFlowClient should be provided");

    let stats = LocalResource::new(
        move || {
            let auth = auth.clone();
            async move {
                auth.get::<HabitStats>(&format!("/log/{}/stats", habit_id))
                    .await
                    .ok()
            }
        },
    );

    let time_display = move || {
        stats.get().flatten().map(|s| {
            let hours = s.total_minutes / 60;
            let mins = s.total_minutes % 60;
            if hours > 0 {
                format!("{}h {}m", hours, mins)
            } else {
                format!("{}m", mins)
            }
        }).unwrap_or_else(|| "...".to_string())
    };

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
            <div class="habit-item-header">
                <h3 class="habit-item-title">{title}</h3>
                <span class="habit-item-stats">{time_display}</span>
            </div>
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
pub fn CalendarCell(
    day_idx: usize,
    time: String,
    next_time: String,
    schedules: LocalResource<ScheduleRes>,
    habits: LocalResource<PagedResponse<HabitData>>,
    set_schedule_modal_info: WriteSignal<Option<(Uuid, usize, String, String)>>,
    set_log_modal_info: WriteSignal<Option<(Option<Uuid>, usize, String, String)>>,
) -> impl IntoView {
    let time_inner_for_drop = time.clone();
    let time_inner_for_view = time.clone();
    let time_inner_for_click = time.clone();
    let next_time_inner_for_drop = next_time.clone();
    let next_time_inner_for_view = next_time.clone();
    let next_time_inner_for_click = next_time.clone();

    view! {
        <div class="grid-cell"
            on:dragover=move |ev| ev.prevent_default()
            on:drop=move |ev| {
                ev.prevent_default();
                if let Some(dt) = ev.data_transfer() {
                    if let Ok(habit_id_str) = dt.get_data("text/plain") {
                        if let Ok(habit_id) = Uuid::parse_str(&habit_id_str) {
                            set_schedule_modal_info.set(Some((habit_id, day_idx, time_inner_for_drop.clone(), next_time_inner_for_drop.clone())));
                        }
                    }
                }
            }
            on:click=move |_| {
                if let Some(h_resp) = habits.get_untracked() {
                    if !h_resp.items.is_empty() {
                        set_log_modal_info.set(Some((None, day_idx, time_inner_for_click.clone(), next_time_inner_for_click.clone())));
                    }
                }
            }
        >
            {move || {
                let s_data = schedules.get();
                let h_data = habits.get();
                let time_inner = time_inner_for_view.clone();
                let next_time_inner = next_time_inner_for_view.clone();

                if let (Some(s_resp), Some(h_resp)) = (s_data, h_data) {
                    let filtered_schedules: Vec<_> = s_resp.schedules.iter()
                        .filter(|s| {
                            let s_day_idx = match s.day {
                                DayOfWeek::Monday => 0,
                                DayOfWeek::Tuesday => 1,
                                DayOfWeek::Wednesday => 2,
                                DayOfWeek::Thursday => 3,
                                DayOfWeek::Friday => 4,
                                DayOfWeek::Saturday => 5,
                                DayOfWeek::Sunday => 6,
                            };
                            let s_time_str = s.start_time.format("%H:%M").to_string();
                            let is_in_range = if next_time_inner < time_inner {
                                // Wrap around case (e.g., 20:00 to 00:00 or 00:00 to 06:00)
                                s_time_str >= time_inner || s_time_str < next_time_inner
                            } else {
                                s_time_str >= time_inner && s_time_str < next_time_inner
                            };
                            s_day_idx == day_idx && is_in_range
                        })
                        .collect();

                    logging::log!("Cell {} {}-{} count: {}", day_idx, time_inner, next_time_inner, filtered_schedules.len());
                    
                    let count = filtered_schedules.len();
                    let hide_text = count > 3;

                    filtered_schedules.into_iter()
                        .map(|s| {
                            let habit = h_resp.items.iter().find(|h| h.habit_id == s.habit_id);
                            let color_index = h_resp.items.iter().position(|h| h.habit_id == s.habit_id).unwrap_or(0);
                            let title = habit.map(|h| h.name.clone()).unwrap_or_else(|| "Unknown".to_string());
                            let habit_id = s.habit_id;
                            let start_time = s.start_time.format("%H:%M").to_string();
                            let end_time = s.end_time.format("%H:%M").to_string();

                            view! {
                                <CalendarHabitItem
                                    habit_id=habit_id
                                    title=title
                                    color_index=color_index
                                    hide_text=hide_text
                                    day_idx=day_idx
                                    start_time=start_time
                                    end_time=end_time
                                    set_log_modal_info=set_log_modal_info
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
pub fn NavRail(
    set_show_modal: WriteSignal<bool>,
    set_show_logout_modal: WriteSignal<bool>,
) -> impl IntoView {
    view! {
        <nav class="nav-rail">
            <button class="icon-btn" on:click=move |_| set_show_modal.set(true)>
                <crate::components::icons::IconPlus />
            </button>
            <button class="icon-btn" on:click=move |_| set_show_logout_modal.set(true)>
                <crate::components::icons::IconLogout />
            </button>
        </nav>
    }
}

#[component]
pub fn HabitsSidebar(
    habits: LocalResource<PagedResponse<HabitData>>,
    set_editing_habit: WriteSignal<Option<HabitData>>,
) -> impl IntoView {
    view! {
        <aside class="habits-sidebar">
            <h1 class="habits-title">"My Habits"</h1>
            <Suspense fallback=move || view! { <div class="habits-empty-state">"Loading..."</div> }>
                {move || {
                    let data = habits.get();
                    if let Some(resp) = data {
                        if resp.items.is_empty() {
                            view! {
                                <div class="habits-empty-state">
                                    "You have no habits yet"
                                </div>
                            }.into_any()
                        } else {
                            view! {
                                <div class="habits-list">
                                    {resp.items.iter().enumerate().map(|(i, habit)| {
                                        let habit_clone = habit.clone();
                                        view! {
                                            <HabitItem
                                                habit_id=habit_clone.habit_id
                                                title=habit.name.clone()
                                                description=habit.description.clone().unwrap_or_default()
                                                color_index=i
                                                on_edit=Callback::new(move |_| {
                                                    set_editing_habit.set(Some(habit_clone.clone()));
                                                })
                                            />
                                        }
                                    }).collect_view()}
                                </div>
                            }.into_any()
                        }
                    } else {
                        view! { <div class="habits-empty-state">"Loading..."</div> }.into_any()
                    }
                }}
            </Suspense>
        </aside>
    }
}

#[component]
pub fn CalendarGrid(
    days: Vec<&'static str>,
    times: Vec<&'static str>,
    schedules: LocalResource<ScheduleRes>,
    habits: LocalResource<PagedResponse<HabitData>>,
    set_schedule_modal_info: WriteSignal<Option<(Uuid, usize, String, String)>>,
    set_log_modal_info: WriteSignal<Option<(Option<Uuid>, usize, String, String)>>,
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
                    let next_time = times.get(time_idx + 1).cloned().unwrap_or("06:00");
                    
                    view! {
                        <div class="time-label">{time}</div>
                        { (0..7).map(move |day_idx| {
                            view! {
                                <CalendarCell
                                    day_idx=day_idx
                                    time=time_clone.clone()
                                    next_time=next_time.to_string()
                                    schedules=schedules
                                    habits=habits
                                    set_schedule_modal_info=set_schedule_modal_info
                                    set_log_modal_info=set_log_modal_info
                                />
                            }
                        }).collect_view() }
                    }
                }).collect_view()}
            </div>
        </main>
    }
}
