use std::collections::HashMap;
use std::time::Duration;
use itertools::Itertools;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use leptos_toaster::{Toast, ToastId, ToastOptions, Toasts, ToastVariant,Theme};
use crate::components::header::Header;
use crate::components::favorite::{Favorite,AddFavorite,EmptyFavorite};
use crate::components::not_logged_in_home::NotLoggedInHomePage;
#[component]
pub fn Home() -> impl IntoView {
	provide_meta_context();
	//let toast_context = expect_context::<Toasts>();
	let result = create_resource(|| {}, |_| async move {
		get_favorites().await
	});
	// let create_toast = move |_| {
	// 	spawn_local(async move {
	// 		use crate::grpc::*;
	// 		test_sensor_create().await.unwrap();
	// 	});
	// 	let toast_id = ToastId::new();
	// 	toast_context.toast(
	// 		// This uses the built in toast component that requires the `builtin_toast` feature.
	// 		// You can use your own components here
	// 		view! {
	// 		<Toast
	// 			theme=Theme::Light
	// 			toast_id
	// 			variant=ToastVariant::Info
	// 			title=view! {"Worked"}.into_view()
	// 		/>
	// 	},
	// 		Some(toast_id),
	// 		Some(ToastOptions {
	// 			dismissible: true,
	// 			duration: Some(Duration::from_secs(10)),
	// 			..Default::default()
	// 		}) // options
	// 	);
	// };
	let reload = store_value(move || result.refetch());
	view! {
		<div class="d-flex flex-column flex-fill">
			<Header/>
			<Suspense>
				<Show
				when=move || result.get().is_some()>
				<Show
				when=move || result.get().unwrap().is_ok()
				fallback=move || view!{
					<NotLoggedInHomePage/>
				}>
					<For
					each=move || result.get().unwrap().unwrap_or_default().clone().into_iter().enumerate()
					key=move |data| data.0.clone()
					let:y>
						<div class="d-flex flex-column m-4">
							<div class="d-flex flew-row justify-content-between flex-wrap">
								<For
								each=move || y.1.clone().into_iter().enumerate()
								key=move |data| data.0.clone()
								let:x>
									{ move || if x.1 >= 0 {
										view!{<Favorite id=x.1 as u32 x=x.0 as u32 y=y.0 as u32 reload=reload.clone()/>}
									} else {
										view!{
											<EmptyFavorite y=y.0 as u32 x=x.0 as u32 reload=reload.clone()/>
										}
									}}
									
								</For>
							</div>
						</div>
					</For>
				</Show>
				
				</Show>
			</Suspense>
		</div>
    }
}

fn hash_map_to_vec(hash_map: &HashMap<(i32, i32), i32>) -> Vec<Vec<i32>> {
    // Determine the bounds for the vector dimensions
    let (min_y, max_y, min_x, max_x) = hash_map.keys().fold(
        (i32::MAX, i32::MIN, i32::MAX, i32::MIN),
        |(min_y, max_y, min_x, max_x), &(y, x)| {
            (
                min_y.min(y),
                max_y.max(y),
                min_x.min(x),
                max_x.max(x),
            )
        },
    );

    // Calculate dimensions
    let height = (max_y - min_y + 1 + 1).max(1) as usize;
    let width = (max_x - min_x + 1).max(3) as usize;

    // Initialize a 2D vector with default values (e.g., 0)
    let mut vec2d = vec![vec![-1; width]; height];

    // Populate the 2D vector with values from the hash map
    for (&(y, x), &value) in hash_map {
        let row = (y) as usize;
        let col = (x) as usize;
        vec2d[row][col] = value;
    }
    vec2d
}


#[server]
pub async fn get_favorites() -> Result<Vec<Vec<i32>>,ServerFnError> {
    use crate::shared::app_state::AppState;
	use crate::auth::get_user;
    use futures::stream::BoxStream;
    use futures::{Stream, TryStream, TryStreamExt};
    let user = get_user().await?;
	if user.is_none() {
		return Err(ServerFnError::ServerError("Unauthorized".to_string()));
	}
	let user = user.unwrap();
    let db = expect_context::<AppState>().db;
    let mut favorites_hash: HashMap<(i32,i32),i32> = HashMap::new();
	let mut stream = sqlx::query!("SELECT * FROM favorites WHERE user_id=?",user.id).fetch(db.get_pool());
    while let Some(value) = stream.try_next().await? {
        favorites_hash.insert((value.y,value.x), value.graph);
    };
	if favorites_hash.is_empty() {
		return Ok(vec![vec![-1;3]]);
	}
    Ok(hash_map_to_vec(&favorites_hash))
}