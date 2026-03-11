use frontend::api::client::AuthFlowClient;
use frontend::pages::LoginPage;
use leptos::{IntoView, component, view};
use leptos::prelude::*;
use leptos_router::{components::*, path};

#[component]
fn WideApp() -> impl IntoView {
    let auth_client = AuthFlowClient::new();

    provide_context(auth_client);

    view! {
        <Router>
            <main>
                <Routes fallback=|| view! { "Page now found." }>
                    <Route path=path!("/") view=LoginPage />
                </Routes>
            </main>
        </Router>
    }
}

fn main() {
    leptos::mount::mount_to_body(WideApp);
}
