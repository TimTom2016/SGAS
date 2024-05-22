use leptos::*;
use crate::components::login;
use crate::auth;

#[component]
pub fn Login() -> impl IntoView {
    let action = create_server_action::<auth::Login>();
    view! {
        <div class="vh-100 vw-100 d-flex align-items-center justify-content-center">
            <div class="">
                <login::Login action=action/>
            </div>
        </div>
    }
}
