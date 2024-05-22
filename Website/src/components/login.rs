use leptos::*;
use leptos_captcha::Captcha;
use leptos_router::*;
use crate::auth;
use leptos::ev::SubmitEvent;
use crate::pages::main::get_pow;
#[component]
pub fn Login(
	action: Action<auth::Login, Result<(), ServerFnError>>,
) -> impl IntoView {
    let is_pending = create_rw_signal(None);

    let on_submit = move |ev: SubmitEvent| {
        if let Ok(mut data) = auth::Login::from_event(&ev) {
            ev.prevent_default();
            action.set_pending(true);

            // Currently, the Captcha validation is running thread local.
            // This means a too high difficulty will block the thread.
            // The default of 20 is reasonable for a release build, but
            // way too high for development.
            //
            // The validation might me improved in the future by using
            // a web worker for this purpose, but this is not yet implemented.
            leptos_captcha::pow_dispatch(get_pow, is_pending, move |pow| {
                data.pow = pow.unwrap();
                action.dispatch(data);
            })
        }
    };
    let query = use_query_map();
    let next: String = query.with(|q| q.get("next").map(|x| x.to_string()).unwrap_or("/".to_string()));
    view! {
        <ActionForm action=action on:submit=on_submit class="card">
            <div class="card-body">
                <h3 class="card-title text-center">"Login"</h3>
                <div class="mb-3">
                    <label class="form-label" for="username">
                        "Username"
                    </label>
                    <input class="form-control"
                        id="username"
                        type="text"
                        maxlength="32"
                        name="username"
                    />
                </div>
                <div class="mb-3">
                    <label class="form-label" for="password">
                        "Password"
                    </label>
                    <input class="form-control"
                        id="password"
                        type="password"
                        name="password"
                    />
                </div>
                <label class="d-none">
                    <input type="text" prop:value=next.clone() name="next"/>
                </label>
                <div class="mb-3">
                    <button type="submit" class=" btn btn-primary d-flex align-self-center">
                        "Login"
                    </button>
                </div>
                <div class="d-none">
                    <Captcha is_pending=is_pending/>
                </div>
                <A href=move || format!("/signup?next={}",next) class="text-reset text-decoration-none text-opacity-75"> 
                        <p><small>"Don't have an Account? " <span class="text-primary">"Press here"</span></small></p>
                </A>
            </div>
        </ActionForm>
    }
}