use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use leptos_toaster::{Toaster, ToasterPosition};
use leptos_use::{signal_throttled, use_css_var, use_preferred_dark};
use crate::pages::home::Home;
use crate::pages::login::Login;
use crate::pages::signup::Signup;
use crate::pages::database::Database;
use thaw::*;
#[component]
pub fn App() -> impl IntoView {
	provide_meta_context();
	let is_dark_preferred = signal_throttled(use_preferred_dark(), 1000.0);
	let theme= create_rw_signal(Theme::dark());
	
	create_effect(move |_| 
	{
		if is_dark_preferred.get() {
			theme.set(Theme::dark());
			document().body().unwrap().set_attribute("data-bs-theme", "dark");
		} else {
			theme.set(Theme::light());
			document().body().unwrap().set_attribute("data-bs-theme", "light");
		};

	});
	view! {
        <Stylesheet href="https://cdn.jsdelivr.net/npm/bootstrap@5.3.2/dist/css/bootstrap.min.css"/>
        <Stylesheet id="leptos" href="/pkg/sgas.css"/>
		<link rel="preconnect" href="https://fonts.googleapis.com"/>
		<link rel="preconnect" href="https://fonts.gstatic.com" crossorigin/>
		<Stylesheet href="https://fonts.googleapis.com/css2?family=Playwrite+GB+J:ital,wght@0,100..400;1,100..400&display=swap"/>
        // <script src="https://cdn.jsdelivr.net/npm/@floating-ui/core@1.6.0"></script>
        // <script src="https://cdn.jsdelivr.net/npm/@floating-ui/dom@1.6.1"></script>
        <script src="https://cdn.jsdelivr.net/npm/bootstrap@5.3.2/dist/js/bootstrap.bundle.min.js" integrity="sha384-C6RzsynM9kWDrMNeT87bh95OGNyZPhcTNXj1NW7RuBCsyN/o0jlpcV8Qyq46cDfL" crossorigin="anonymous"></script>
        <script src="https://cdn.jsdelivr.net/npm/echarts@5.4.2/dist/echarts.min.js"></script>
        <script src="https://cdn.jsdelivr.net/npm/echarts-gl@2.0.9/dist/echarts-gl.min.js"></script>
        <Router>
			<ThemeProvider theme=theme>
				<main class="min-vh-100 d-flex  gradient-back">
					//<Toaster position=ToasterPosition::BottomCenter>
						<Routes>
							<Route path="/" view=Home/>
							<Route path="database" view=Database/>
							<Route path="login" view=Login/>
							<Route path="signup" view=Signup/>
						</Routes>
					//</Toaster>
				</main>
			</ThemeProvider>
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