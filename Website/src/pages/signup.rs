use leptos::*;
use leptos_router::*;
use crate::auth;
use crate::components::signup;

#[component]
pub fn Signup() -> impl IntoView {
    let signup = create_server_action::<auth::Signup>();
    let next = move || use_query_map().with(|q| q.get("next").map(|x| x.to_string()).unwrap_or("/".to_string()));
    view! {
        <div class="vh-100 vw-100 d-flex align-items-center justify-content-center">
            <div class="">
                <signup::Signup action=signup next=next()/>
            </div>
        </div>
    }
}





