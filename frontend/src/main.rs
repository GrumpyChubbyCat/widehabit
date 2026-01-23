use leptos::prelude::*;
use leptos::{
    IntoView, component,
    view,
};

#[component]
fn LoginPage() -> impl IntoView {
    view! {
        <div class="auth-wrapper">
            <div class="auth-card">
                <h2 class="auth-title">"Hello."</h2>

                <div class="input-section">
                    // Login block
                    <div class="input-group">
                        <input type="text" placeholder="username" id="login-input" />
                        <label class="input-label" for="login-input">"login"</label>
                    </div>
                
                    // Password block
                    <div class="input-group">
                        <input type="password" placeholder="*********" id="password-input" />
                        <label class="input-label" for="password-input">"password"</label>
                    </div>
                </div>

                <button class="sign-in-button">
                    "Sign in"
                </button>
            </div>
        </div>
    }
}

fn main() {
    leptos::mount::mount_to_body(LoginPage);
}
