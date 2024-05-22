#[cfg(feature = "ssr")]
pub mod ssr {
	use crate::db::repositories::user::UserRepository;

	#[derive(Clone)]
	pub struct Repositories {
		pub user_repo: UserRepository,
	}
}

#[cfg(feature = "ssr")]
pub use ssr::Repositories;