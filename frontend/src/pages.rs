use crate::api::client::AuthFlowClient;
use crate::compontents::{AuthButton, MainInput};
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