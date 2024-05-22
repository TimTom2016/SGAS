use leptos::*;
use leptos_captcha::Captcha;
use leptos_router::*;
use crate::auth;
use crate::pages::main::get_pow;
use leptos::ev::SubmitEvent;


#[component]
pub fn Signup(
	action: Action<auth::Signup, Result<(), ServerFnError>>,
    #[prop(default = "/".to_string().into(),into)]
    next: MaybeSignal<String>,
) -> impl IntoView {
    let is_pending = create_rw_signal(None);

    let on_submit = move |ev: SubmitEvent| {
        if let Ok(mut data) = auth::Signup::from_event(&ev) {
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
	view! {
        <ActionForm action=action on:submit=on_submit class="card">
            <div class="card-body">
                <h3 class="card-title text-center">"Sign Up"</h3>
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
                <div class="mb-3">
                    <label class="form-label" for="password_confirmation">
                        "Confirm Password"
                    </label>
                    <input class="form-control"
                        id="password_confirmation"
                        type="password"
                        name="password_confirmation"
                    />
                </div>
                <div class="mb-3">
                    <input class="form-check-input" type="checkbox" name="remember" id="remember"/>
                    <label class="form-check-label" for="remember">
                        "Remember me?"
                    </label>
                </div>
                <label class="d-none">
                    <input type="text" prop:value=next.clone() name="next"/>
                </label>
                <div class="mb-3">
                    <button type="submit" class=" btn btn-primary d-flex align-self-center">
                        "Sign Up"
                    </button>
                </div>
                <div class="d-none">
                    <Captcha is_pending=is_pending/>
                </div>
                <A href=move || format!("/login?next={}", next.get()) class="text-reset text-decoration-none text-opacity-75"> 
                        <p><small>"Already a Member? " <span class="text-primary">"Press here"</span></small></p>
                </A>
            </div>
        </ActionForm>
    }
}