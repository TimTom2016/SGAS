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
