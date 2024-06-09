use std::time::Duration;
use itertools::Itertools;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use leptos_toaster::{Toast, ToastId, ToastOptions, Toasts, ToastVariant,Theme};
use crate::components::header::Header;
use crate::components::favorite::{Favorite,AddFavorite};
#[component]
pub fn Home() -> impl IntoView {
	provide_meta_context();
	let toast_context = expect_context::<Toasts>();

	let create_toast = move |_| {
		spawn_local(async move {
			use crate::grpc::*;
			test_sensor_create().await.unwrap();
		});
		let toast_id = ToastId::new();
		toast_context.toast(
			// This uses the built in toast component that requires the `builtin_toast` feature.
			// You can use your own components here
			view! {
			<Toast
				theme=Theme::Light
				toast_id
				variant=ToastVariant::Info
				title=view! {"Worked"}.into_view()
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
	let graphs = (0..3).collect_vec();
	let values = (0..100).collect_vec().iter().map(|x| graphs.clone()).collect_vec();
	
	view! {
		<div class="d-flex flex-column"> 
			<Header/>
			<For
			each=move || values.clone()
			key=move |data| data.clone()
			let:data>
				<div class="d-flex flex-column m-4">
					<div class="d-flex flew-row justify-content-between flex-wrap">
						<For
						each=move || data.clone()
						key=move |data| data.clone()
						let:data>
							<Favorite/>
						</For>
					</div>
				</div>
			</For>
			<AddFavorite/>
			<button class="btn btn-primary align-self-start d-flex" on:click=create_toast/>
		</div>
    }
}