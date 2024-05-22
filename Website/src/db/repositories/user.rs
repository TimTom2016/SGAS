use std::collections::HashSet;
use sqlx::mysql::MySqlPool;
use crate::shared::user::User;

use super::base::BaseRepository;

#[derive(Clone)]
pub struct UserRepository
{
	pub pool: MySqlPool,
}

#[derive(sqlx::FromRow, Clone)]
pub struct SqlPermissionTokens {
	pub token: String,
}

impl BaseRepository for UserRepository
{
	type Item = User;
	async fn get_all(&self) -> Result<Vec<Self::Item>,Self::Error> {
		todo!("Make get all at user repo");
		let patients = sqlx::query!("SELECT * FROM users")
			.fetch_all(&self.pool)
			.await?;
		let mut results = vec![];
		
		return Ok(results);
	}
	async fn get_by_id(&self, id: u64) -> Result<Self::Item,Self::Error> {
		let user = sqlx::query!("SELECT * FROM users WHERE id = ?", id)
			.fetch_one(&self.pool)
			.await?;
		let sql_user_perms = sqlx::query_as::<_, crate::auth::ssr::SqlPermissionTokens>(
			"SELECT token FROM user_permissions WHERE user_id = ?;",
		)
			.bind(user.id)
			.fetch_all(&self.pool)
			.await
			.ok();

		return Ok(User {
			id: user.id as i64,
			username: user.username,
			password: user.password,
			permissions: if let Some(user_perms) = sql_user_perms {
				user_perms
					.into_iter()
					.map(|x| x.token)
					.collect::<HashSet<String>>()
			} else {
				HashSet::<String>::new()
			},
		});
	}
	async fn create(&self, data: Self::Item) -> Result<u64,Self::Error> {
		let id = sqlx::query("INSERT INTO users (username, password) VALUES (?, ?)")
			.bind(data.username)
			.bind(data.password)
			.execute(&self.pool)
			.await?
			.last_insert_id();
		return Ok(id);
	}
	async fn update(&self, id: u64, data: Self::Item) -> Result<u64,Self::Error> {
		let id = sqlx::query("UPDATE users SET username = ?, password = ? WHERE id = ?")
			.bind(data.username)
			.bind(data.password)
			.bind(id)
			.execute(&self.pool)
			.await?
			.last_insert_id();
		return Ok(id);
	}
	async fn delete(&self, id: u64) -> Result<(),Self::Error>{
		sqlx::query("DELETE FROM users WHERE id = ?")
			.bind(id)
			.execute(&self.pool)
			.await?;
		Ok(())
	}
}

impl UserRepository {
	pub async fn get_by_username(&self, username: String) -> Result<User,sqlx::Error> {
		let user = sqlx::query!("SELECT * FROM users WHERE username = ?", username)
			.fetch_one(&self.pool)
			.await?;
		let sql_user_perms = sqlx::query_as::<_, crate::auth::ssr::SqlPermissionTokens>(
			"SELECT token FROM user_permissions WHERE user_id = ?;",
		)
			.bind(user.id)
			.fetch_all(&self.pool)
			.await
			.ok();

		return Ok(User {
			id: user.id as i64,
			username: user.username,
			password: user.password,
			permissions: if let Some(user_perms) = sql_user_perms {
				user_perms
					.into_iter()
					.map(|x| x.token)
					.collect::<HashSet<String>>()
			} else {
				HashSet::<String>::new()
			},
		});
	}
}