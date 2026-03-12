use crate::api::client::AuthFlowClient;
use crate::compontents::{AuthButton, MainInput, IconPlus, IconSettings, NewHabitModal};
use leptos::task::spawn_local;
use leptos::{IntoView, component, view};
use leptos::{logging, prelude::*};
use shared::model::user::UserAuthReq;

#[component]
pub fn LoginPage() -> impl IntoView {
    let auth = use_context::<AuthFlowClient>().expect("AuthFlowClient should be provided in context");

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
    let (refresh_trigger, set_refresh_trigger) = signal(());

    let auth = use_context::<AuthFlowClient>().expect("AuthFlowClient should be provided in context");

    let habits = LocalResource::new(move || {
        let auth = auth.clone();
        refresh_trigger.get();
        async move {
            let res: Result<shared::model::PagedResponse<shared::model::habit::HabitData>, String> =
                auth.get("/habit?page=1&limit=7").await;
            res.unwrap_or_else(|_| shared::model::PagedResponse {
                items: vec![],
                total_count: 0,
                page: 1,
                page_size: 100,
            })
        }
    });

    view! {
        <div class="habits-container">
            {move || if show_modal.get() {
                Some(view! {
                    <NewHabitModal set_show_modal=set_show_modal set_refresh_trigger=set_refresh_trigger />
                })
            } else {
                None
            }}
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
                                        {resp.items.into_iter().enumerate().map(|(i, habit)| {
                                            view! {
                                                <crate::compontents::HabitItem
                                                    title=habit.name
                                                    description=habit.description.unwrap_or_default()
                                                    color_index=i
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
            <main class="calendar-view">
                <div class="calendar-grid">
                    // Day headers
                    <div></div> // Spacer for the time column
                    {days.into_iter().map(|day| view! {
                        <div class="day-header">{day}</div>
                    }).collect_view()}

                    // Time rows and cells
                    {times.into_iter().map(|time| view! {
                        <div class="time-label">{time}</div>
                        { (0..7).map(|_| view! {
                            <div class="grid-cell"></div>
                        }).collect_view() }
                    }).collect_view()}
                </div>
            </main>
        </div>
    }
}