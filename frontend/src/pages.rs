use crate::api::client::AuthFlowClient;
use crate::components::modals::{EditHabitModal, LogHabitModal, NewHabitModal, ScheduleHabitModal, LogoutModal};
use crate::components::{AuthButton, AuthTextLink, CalendarGrid, MainInput, NavRail, HabitsSidebar};
use leptos::task::spawn_local;
use leptos::{component, view, IntoView};
use leptos::{logging, prelude::*};
use shared::model::habit::HabitData;
use shared::model::schedule::ScheduleRes;
use shared::model::user::{UserAuthReq, UserRegistrationReq};
use shared::model::PagedResponse;
use uuid::Uuid;

#[component]
pub fn LoginPage() -> impl IntoView {
    let auth = use_context::<AuthFlowClient>();
    let navigate = leptos_router::hooks::use_navigate();
    
    if auth.is_none() {
        logging::error!("LoginPage: AuthFlowClient is MISSING from context!");
    }

    let auth = auth.expect("AuthFlowClient should be provided in context");
    let navigate_to_registration = navigate.clone();

    let (username, set_username) = signal(String::new());
    let (password, set_password) = signal(String::new());
    let (is_loading, _set_is_loading) = signal(false);
    let (has_error, _set_has_error) = signal(false);

    let on_sign_in = move |_| {
        logging::log!("Sign in button clicked");
        
        let navigate = navigate.clone();

        let auth = auth.clone();
        _set_is_loading.set(true);
        _set_has_error.set(false);

        let username_val = username.get_untracked();
        let password_val = password.get_untracked();

        spawn_local(async move {
            let req = UserAuthReq {
                username: username_val,
                password: password_val,
            };

            logging::log!("Calling auth.login...");
            let res = auth.login("/auth/login", &req).await;
            logging::log!("auth.login returned");

            match res {
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

                <AuthTextLink
                    text="Create account"
                    on_click=Callback::new(move |_| navigate_to_registration("/registration", Default::default()))
                />
            </div>
        </div>
    }
}

#[component]
pub fn RegisterPage() -> impl IntoView {
    let auth = use_context::<AuthFlowClient>();
    let navigate = leptos_router::hooks::use_navigate();

    if auth.is_none() {
        logging::error!("RegisterPage: AuthFlowClient is MISSING from context!");
    }

    let auth = auth.expect("AuthFlowClient should be provided in context");
    let navigate_to_login = navigate.clone();

    let (email, set_email) = signal(String::new());
    let (username, set_username) = signal(String::new());
    let (password, set_password) = signal(String::new());
    let (password_confirm, set_password_confirm) = signal(String::new());
    let (is_loading, set_is_loading) = signal(false);
    let (has_error, set_has_error) = signal(false);
    let (error_message, set_error_message) = signal(String::new());

    let on_register = move |_| {
        let auth = auth.clone();
        let navigate = navigate.clone();

        set_is_loading.set(true);
        set_has_error.set(false);
        set_error_message.set(String::new());

        let email_val = email.get_untracked();
        let username_val = username.get_untracked();
        let password_val = password.get_untracked();
        let password_confirm_val = password_confirm.get_untracked();

        if password_val != password_confirm_val {
            set_is_loading.set(false);
            set_has_error.set(true);
            set_error_message.set("Passwords do not match".to_string());
            return;
        }

        spawn_local(async move {
            let req = UserRegistrationReq {
                email: email_val,
                username: username_val,
                password: password_val,
            };

            match auth.register("/auth/registration", &req).await {
                Ok(_) => navigate("/login", Default::default()),
                Err(err) => {
                    logging::error!("Registration error: {}", err);
                    set_has_error.set(true);
                    set_error_message.set("Failed to create account".to_string());
                }
            }

            set_is_loading.set(false);
        });
    };

    view! {
        <div class="auth-wrapper">
            <div class="auth-card">
                <h2 class="auth-title">"Welcome!"</h2>

                <div class="input-section">
                    <MainInput
                        label="email"
                        placeholder="name@example.com"
                        input_type="email"
                        value=email
                        set_value=set_email
                        has_error=has_error.into()
                    />
                    <MainInput
                        label="login"
                        placeholder="username"
                        input_type="text"
                        value=username
                        set_value=set_username
                        has_error=has_error.into()
                    />
                    <MainInput
                        label="password"
                        placeholder="*********"
                        input_type="password"
                        value=password
                        set_value=set_password
                        has_error=has_error.into()
                    />
                    <MainInput
                        label="confirm password"
                        placeholder="*********"
                        input_type="password"
                        value=password_confirm
                        set_value=set_password_confirm
                        has_error=has_error.into()
                    />
                </div>

                {move || has_error.get().then(|| view! { <div class="error-message">{error_message.get()}</div> })}

                <AuthButton
                    text="Create account"
                    loading_text="Wait..."
                    is_loading=is_loading.into()
                    on_click=Callback::new(on_register)
                />

                <AuthTextLink
                    text="Back to sign in"
                    on_click=Callback::new(move |_| navigate_to_login("/login", Default::default()))
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
    let (log_modal_info, set_log_modal_info) =
        signal::<Option<(Option<Uuid>, usize, String, String)>>(None);
    let (show_logout_modal, set_show_logout_modal) = signal(false);
    let (refresh_trigger, set_refresh_trigger) = signal(());

    let auth =
        use_context::<AuthFlowClient>().expect("AuthFlowClient should be provided in context");

    let navigate = leptos_router::hooks::use_navigate();

    let on_logout = {
        let auth = auth.clone();
        let navigate = navigate.clone();
        move |_: ()| {
            auth.logout();
            set_show_logout_modal.set(false);
            navigate("/login", Default::default());
        }
    };

    let on_logout_cb = Callback::new(on_logout);

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
            
            <NavRail set_show_modal=set_show_modal set_show_logout_modal=set_show_logout_modal />

            <HabitsSidebar habits=habits set_editing_habit=set_editing_habit />

            // Main grid
            <Suspense fallback=move || view! { <div class="calendar-view">"Loading calendar..."</div> }>
                <CalendarGrid
                    days=days
                    times=times
                    schedules=schedules
                    habits=habits
                    set_schedule_modal_info=set_schedule_modal_info
                    set_log_modal_info=set_log_modal_info
                />
            </Suspense>

            <Suspense>
                {move || if let Some((habit_id, day_idx, start_time, end_time)) = log_modal_info.get() {
                    view! {
                        <LogHabitModal
                            habit_id=habit_id
                            day_idx=day_idx
                            start_time_str=start_time
                            end_time_str=end_time
                            habits=habits
                            on_close=Callback::new(move |_| set_log_modal_info.set(None))
                            set_refresh_trigger=set_refresh_trigger
                        />
                    }.into_any()
                } else {
                    view! { <span/> }.into_any()
                }}
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

            <Suspense>
                {move || if show_logout_modal.get() {
                    view! {
                        <LogoutModal
                            on_cancel=Callback::new(move |_| set_show_logout_modal.set(false))
                            on_confirm=on_logout_cb
                        />
                    }.into_any()
                } else {
                    view! { <span/> }.into_any()
                }}
            </Suspense>
        </div>
    }
}
