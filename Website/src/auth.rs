use leptos::*;
use crate::shared::user::User;
#[cfg(feature = "ssr")]
pub mod ssr {
	pub use axum_session_sqlx::SessionMySqlPool;
	pub use crate::shared::user::User;
	pub use axum_session_auth::{
		Authentication, HasPermission,
	};
	pub use sqlx::MySqlPool;
	use leptos::{ServerFnError, use_context};
	use crate::shared::app_state::AppState;

	pub fn Pool() -> Result<MySqlPool, ServerFnError> {
		use_context::<AppState>().map(|x| x.db.get_pool().clone())
			.ok_or_else(|| ServerFnError::ServerError("Pool missing.".into()))
	}

	pub fn auth() -> Result<AuthSession, ServerFnError> {
		use_context::<AuthSession>().ok_or_else(|| {
			ServerFnError::ServerError("Auth session missing.".into())
		})
	}
	pub type AuthSession = axum_session_auth::AuthSession<
		User,
		i64,
		SessionMySqlPool,
		MySqlPool,
	>;

	#[derive(sqlx::FromRow, Clone)]
	pub struct SqlPermissionTokens {
		pub token: String,
	}
	#[async_trait::async_trait]
	impl Authentication<User, i64, MySqlPool> for User {
		async fn load_user(
			userid: i64,
			pool: Option<&MySqlPool>,
		) -> Result<User, anyhow::Error> {
			use crate::db::repositories::user::UserRepository;
			use crate::db::repositories::base::BaseRepository;
			let repo = UserRepository {
				pool: pool.unwrap().clone()
			};

			let user: User = repo.get_by_id(userid as u64).await?;
			return Ok(user);
		}

		fn is_authenticated(&self) -> bool {
			true
		}

		fn is_active(&self) -> bool {
			true
		}

		fn is_anonymous(&self) -> bool {
			false
		}
	}
	#[async_trait::async_trait]
	impl HasPermission<MySqlPool> for User {
		async fn has(&self, perm: &str, _pool: &Option<&MySqlPool>) -> bool {
			self.permissions.contains(perm)
		}
	}
}


#[server]
pub async fn get_user() -> Result<Option<User>, ServerFnError> {
	use self::ssr::*;
	let auth = auth()?;

	Ok(auth.current_user)
}

#[server(Login, "/api")]
pub async fn login(
	pow: String,
	username: String,
	password: String,
	remember: Option<String>,
	next: Option<String>,
) -> Result<(), ServerFnError> {
	use self::ssr::*;
	use crate::shared::app_state::AppState;
	use password_auth::verify_password;
	use leptos_captcha::spow::pow::Pow;
	Pow::validate(&pow)?;
	let repos = match use_context::<AppState>() {
		Some(repos) => repos.repos,
		None => return Err(ServerFnError::ServerError("No Repositories".into())),
	};
	let auth = auth()?;

	let user: User = repos.user_repo.get_by_username(username).await?;

	match verify_password(password, &user.password) {
		Ok(()) => {
			auth.login_user(user.id);
			auth.remember_user(remember.is_some());
			leptos_axum::redirect(next.unwrap_or("/".to_string()).as_str());
			Ok(())
		}
		Err(error) => Err(ServerFnError::ServerError(
			error.to_string(),
		)),
	}
}

#[server(Signup, "/api")]
pub async fn signup(
	pow: String,
	username: String,
	password: String,
	password_confirmation: String,
	remember: Option<String>,
	next: Option<String>,
) -> Result<(), ServerFnError> {
	use self::ssr::*;
	use crate::shared::app_state::AppState;
	use password_auth::generate_hash;
	use leptos_captcha::spow::pow::Pow;
	Pow::validate(&pow)?;
	let pool = Pool()?;
	let auth = auth()?;
	let repos = match use_context::<AppState>() {
		Some(repos) => repos.repos,
		None => return Err(ServerFnError::ServerError("No Repositories".into())),
	};

	if password != password_confirmation {
		return Err(ServerFnError::ServerError(
			"Passwords did not match.".to_string(),
		));
	}

	let password_hashed = generate_hash(password);

	sqlx::query("INSERT INTO users (username, password) VALUES (?,?)")
		.bind(username.clone())
		.bind(password_hashed)
		.execute(&pool)
		.await?;

	let user: User = repos.user_repo.get_by_username(username).await?;

	auth.login_user(user.id);
	auth.remember_user(remember.is_some());

	leptos_axum::redirect(next.unwrap_or("/".to_string()).as_str());

	Ok(())
}

#[server(Logout, "/api")]
pub async fn logout() -> Result<(), ServerFnError> {
	use self::ssr::*;

	let auth = auth()?;

	auth.logout_user();
	leptos_axum::redirect("/");

	Ok(())
}