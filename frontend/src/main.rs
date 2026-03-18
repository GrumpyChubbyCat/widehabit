use frontend::api::client::AuthFlowClient;
use frontend::pages::{HabitsPage, LoginPage, RegisterPage};
use leptos::{IntoView, component, view};
use leptos::prelude::*;
use leptos_router::{components::*, path};

#[component]
fn WideApp() -> impl IntoView {
    let auth_client = AuthFlowClient::new();

    provide_context(auth_client.clone());

    view! {
        <Router>
            <main>
                <Routes fallback=|| view! { "Page not found." }>
                    <Route path=path!("/login") view=LoginPage />
                    <Route path=path!("/registration") view=RegisterPage />
                    <ProtectedRoute 
                        path=path!("/") 
                        condition=move || Some(auth_client.is_authenticated()) 
                        redirect_path=|| "/login" 
                        view=HabitsPage 
                    />
                </Routes>
            </main>
        </Router>
    }
}

fn main() {
    leptos::mount::mount_to_body(WideApp);
}
