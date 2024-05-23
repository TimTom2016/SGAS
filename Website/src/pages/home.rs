use std::time::Duration;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use leptos_toaster::{Toast, ToastId, ToastOptions, Toasts, ToastVariant,Theme};
use crate::components::header::Header;
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
				theme=Theme::Dark
				toast_id
				variant=ToastVariant::Info
				title=view! {"My toast"}.into_view()
			/>
		},
			Some(toast_id),
			Some(ToastOptions {
				dismissible: true,
				duration: Some(Duration::from_secs(10)),
				..Default::default()
			}) // options
		);
	};
	view! {
		<div class="d-flex"> 
		<Header/>
		<button class="btn btn-primary align-self-start d-flex flex-column" on:click=create_toast/>
		</div>
    }
}
