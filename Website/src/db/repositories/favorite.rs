use sqlx::mysql::MySqlPool;
use crate::domain::favorite::Favorite;

use super::base::BaseRepository;

#[derive(Clone)]
pub struct FavoriteRepository
{
	pub pool: MySqlPool,
}

impl BaseRepository for FavoriteRepository
{
	type Item = Favorite;
	async fn get_all(&self) -> Result<Vec<Self::Item>,Self::Error> {
		todo!("Invalid");
	}
	async fn get_by_id(&self, id: u64) -> Result<Self::Item,Self::Error> {
		todo!("Not possible")
	}
	async fn create(&self, data: Self::Item) -> Result<u64,Self::Error> {
		let id = sqlx::query!("INSERT INTO favorites (graph, user_id, x, y) VALUES (?, ?,?,?)",data.graph,data.user_id,data.x,data.y)
			.execute(&self.pool)
			.await?
			.last_insert_id();
		return Ok(id);
	}
	async fn update(&self, id: u64, data: Self::Item) -> Result<u64,Self::Error> {
		todo!("Not possible");
	}
	async fn delete(&self, id: u64) -> Result<(),Self::Error>{
        todo!("Not possible");
	}
}

impl FavoriteRepository {
    async fn get_all_by_user_id(&self, id: i64) -> Result<Vec<Favorite>,sqlx::Error> {
        let results = sqlx::query_as("SELECT * FROM favorites WHERE user_id=?")
            .bind(id)
            .fetch_all(&self.pool).await?;
		Ok(results)
    
    }
}