use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use leptos_toaster::{Toaster, ToasterPosition};
use leptos_use::{signal_throttled, use_preferred_dark};
use crate::pages::home::Home;
use crate::pages::login::Login;
use crate::pages::signup::Signup;

#[component]
pub fn App() -> impl IntoView {
	provide_meta_context();

	let is_dark_preferred = signal_throttled(use_preferred_dark(), 60000.0);
	create_effect(move |_| 
	{
		if is_dark_preferred.get() {
			document().body().unwrap().set_attribute("data-bs-theme", "dark")
		} else {
			document().body().unwrap().set_attribute("data-bs-theme", "light")
		}
	});
	view! {
        <Stylesheet href="https://cdn.jsdelivr.net/npm/bootstrap@5.3.2/dist/css/bootstrap.min.css"/>
        <Stylesheet id="leptos" href="/pkg/Website.css"/>
        // <script src="https://cdn.jsdelivr.net/npm/@floating-ui/core@1.6.0"></script>
        // <script src="https://cdn.jsdelivr.net/npm/@floating-ui/dom@1.6.1"></script>
        <script src="https://cdn.jsdelivr.net/npm/bootstrap@5.3.2/dist/js/bootstrap.bundle.min.js" integrity="sha384-C6RzsynM9kWDrMNeT87bh95OGNyZPhcTNXj1NW7RuBCsyN/o0jlpcV8Qyq46cDfL" crossorigin="anonymous"></script>
        // <script src="https://cdn.jsdelivr.net/npm/echarts@5.4.2/dist/echarts.min.js"></script>
        // <script src="https://cdn.jsdelivr.net/npm/echarts-gl@2.0.9/dist/echarts-gl.min.js"></script>
        <Router>
            <main class="vh-100 vw-100 d-flex">
				<Toaster position=ToasterPosition::BottomCenter>
	                <Routes>
						<Route path="/" view=Home/>
						<Route path="login" view=Login/>
						<Route path="signup" view=Signup/>
	                </Routes>
				</Toaster>
            </main>
        </Router>
    }
}

#[server]
pub async fn get_pow() -> Result<String, ServerFnError> {
	use leptos_captcha::spow::pow::Pow;

	// I highly suggest, that you create a global static variable in your app
	// as an indicator if you are in DEV / DEBUG mode, or something like that.
	// You could pull it from the context, or where ever it makes sense for you.
	// In debug mode, the speed of the verification in the UI is a low slower and
	// you should just use the lowest difficulty of `10` during development.
	const DEV_MODE: bool = true;

	if DEV_MODE {
		Ok(Pow::with_difficulty(10, 10)?.to_string())
	} else {
		Ok(Pow::new(10)?.to_string())
	}
}