use crate::api::client::AuthFlowClient;
use crate::components::icons::{IconPlus, IconSettings};
use crate::components::modals::{EditHabitModal, NewHabitModal, ScheduleHabitModal};
use crate::components::{AuthButton, CalendarGrid, HabitItem, MainInput};
use leptos::task::spawn_local;
use leptos::{component, view, IntoView};
use leptos::{logging, prelude::*};
use shared::model::habit::HabitData;
use shared::model::schedule::ScheduleRes;
use shared::model::user::UserAuthReq;
use shared::model::PagedResponse;
use uuid::Uuid;

#[component]
pub fn LoginPage() -> impl IntoView {
    let auth =
        use_context::<AuthFlowClient>().expect("AuthFlowClient should be provided in context");

    let (username, set_username) = signal(String::new());
    let (password, set_password) = signal(String::new());
    let (is_loading, _set_is_loading) = signal(false);
    let (has_error, _set_has_error) = signal(false);

    let on_sign_in = move |_| {
        let navigate = leptos_router::hooks::use_navigate();

        let auth = auth.clone();
        _set_is_loading.set(true);
        _set_has_error.set(false);

        spawn_local(async move {
            let req = UserAuthReq {
                username: username.get(),
                password: password.get(),
            };

            match auth.login("/auth/login", &req).await {
                Ok(_) => {
                    navigate("/", Default::default());
                }
                Err(e) => {
                    logging::error!("Login error: {}", e);
                    _set_has_error.set(true);
                }
            }
            _set_is_loading.set(false);
        });
    };

    view! {
        <div class="auth-wrapper">
            <div class="auth-card">
                <h2 class="auth-title">"Hello."</h2>

                <div class="input-section">
                    // Login block
                    <MainInput
                        label="login"
                        placeholder="username"
                        input_type="text"
                        value=username
                        set_value=set_username
                        has_error=has_error.into()
                    />
                    // Password block
                    <MainInput
                        label="password"
                        placeholder="*********"
                        input_type="password"
                        value=password
                        set_value=set_password
                        has_error=has_error.into()
                    />
                </div>

                {move || has_error.get().then(|| view! { <div class="error-message">"Invalid username or password"</div> })}

                <AuthButton
                    text="Sign in"
                    loading_text="Wait..."
                    is_loading=is_loading.into()
                    on_click=Callback::new(on_sign_in)
                />
            </div>
        </div>
    }
}

#[component]
pub fn HabitsPage() -> impl IntoView {
    let days = vec!["MON", "TUE", "WED", "THU", "FRI", "SAT", "SUN"];
    let times = vec!["06:00", "10:00", "14:00", "18:00", "20:00", "00:00"];

    let (show_modal, set_show_modal) = signal(false);
    let (editing_habit, set_editing_habit) = signal::<Option<HabitData>>(None);
    let (schedule_modal_info, set_schedule_modal_info) =
        signal::<Option<(Uuid, usize, String, String)>>(None);
    let (refresh_trigger, set_refresh_trigger) = signal(());

    let auth =
        use_context::<AuthFlowClient>().expect("AuthFlowClient should be provided in context");

    let schedules = {
        let auth = auth.clone();
        LocalResource::new(move || {
            let auth = auth.clone();
            refresh_trigger.get();
            async move {
                let res: Result<ScheduleRes, String> = auth.get("/schedule").await;
                res.unwrap_or_else(|_| ScheduleRes { schedules: vec![] })
            }
        })
    };

    let habits = {
        let auth = auth.clone();
        LocalResource::new(move || {
            let auth = auth.clone();
            refresh_trigger.get();
            async move {
                let res: Result<PagedResponse<HabitData>, String> =
                    auth.get("/habit?page=1&limit=7").await;
                res.unwrap_or_else(|_| PagedResponse {
                    items: vec![],
                    total_count: 0,
                    page: 1,
                    page_size: 100,
                })
            }
        })
    };

    view! {
        <div class="habits-container">
            <Suspense>
                {move || if show_modal.get() {
                    Some(view! {
                        <NewHabitModal set_show_modal=set_show_modal set_refresh_trigger=set_refresh_trigger />
                    })
                } else {
                    None
                }}
            </Suspense>
            <Suspense>
                {move || if let Some(habit) = editing_habit.get() {
                    Some(view! {
                        <EditHabitModal
                            habit=habit
                            on_close=Callback::new(move |_: ()| set_editing_habit.set(None))
                            set_refresh_trigger=set_refresh_trigger
                        />
                    })
                } else {
                    None
                }}
            </Suspense>
            // Left navigation rail
            <nav class="nav-rail">
                <button class="icon-btn" on:click=move |_| set_show_modal.set(true)>
                    <IconPlus />
                </button>
                <button class="icon-btn">
                    <IconSettings />
                </button>
            </nav>

            // Habits-list
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

            // Main grid
            <Suspense fallback=move || view! { <div class="calendar-view">"Loading calendar..."</div> }>
                <CalendarGrid
                    days=days
                    times=times
                    schedules=schedules
                    habits=habits
                    set_schedule_modal_info=set_schedule_modal_info
                />
            </Suspense>

            <Suspense>
                {move || if let Some((habit_id, day_idx, start_time, end_time)) = schedule_modal_info.get() {
                    view! {
                        <ScheduleHabitModal
                            habit_id=habit_id
                            day_idx=day_idx
                            start_time_str=start_time
                            end_time_str=end_time
                            on_close=Callback::new(move |_| set_schedule_modal_info.set(None))
                            set_refresh_trigger=set_refresh_trigger
                        />
                    }.into_any()
                } else {
                    view! { <span/> }.into_any()
                }}
            </Suspense>
        </div>
    }
}
