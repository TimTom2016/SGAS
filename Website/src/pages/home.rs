use std::time::Duration;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use leptos_toaster::{Toast, ToastId, ToastOptions, Toasts, ToastVariant};

#[component]
pub fn Home() -> impl IntoView {
	provide_meta_context();
	let toast_context = expect_context::<Toasts>();

	let create_toast = move |_| {
		let toast_id = ToastId::new();
		toast_context.toast(
			// This uses the built in toast component that requires the `builtin_toast` feature.
			// You can use your own components here
			view! {
			<Toast
				toast_id
				variant=ToastVariant::Info
				title=view! {"My toast"}.into_view()
			/>
		},
			Some(toast_id),
			Some(ToastOptions {
				dismissible: false,
				duration: Some(Duration::from_secs(100)),
				..Default::default()
			}) // options
		);
	};
	view! {
		<button on:click=create_toast/>

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