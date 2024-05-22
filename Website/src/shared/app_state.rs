#[cfg(feature = "ssr")]
pub mod ssr {
	use axum::extract::FromRef;
	use leptos::LeptosOptions;
	use leptos_router::RouteListing;
	use crate::db::database::Database;
	use crate::shared::repositories::Repositories;

	#[cfg(feature = "ssr")]
	#[derive(Clone)]
	#[derive(FromRef)]
	pub struct AppState {
		pub options: LeptosOptions,
		pub repos: Repositories,
		pub routes: Vec<RouteListing>,
		pub db: Database,
	}
}

#[cfg(feature = "ssr")]
pub use ssr::AppState;

