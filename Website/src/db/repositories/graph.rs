use sqlx::mysql::MySqlPool;
use crate::domain::graph::Graph;

use super::base::BaseRepository;

#[derive(Clone)]
pub struct GraphRepository
{
	pub pool: MySqlPool,
}

impl BaseRepository for GraphRepository
{
	type Item = Graph;
	async fn get_all(&self) -> Result<Vec<Self::Item>,Self::Error> {
		todo!("Invalid");
	}
	async fn get_by_id(&self, id: u64) -> Result<Self::Item,Self::Error> {
		let user: Self::Item = sqlx::query_as("SELECT * FROM graph WHERE id = ?")
			.bind(id)
			.fetch_one(&self.pool)
			.await?;
		Ok(user)
	}
	async fn create(&self, data: Self::Item) -> Result<u64,Self::Error> {
		let id = sqlx::query!("INSERT INTO graph (sensor_id, graph_type) VALUES (?, ?)",data.sensor_id,data.graph_type)
			.execute(&self.pool)
			.await?
			.last_insert_id();
		return Ok(id);
	}
	async fn update(&self, id: u64, data: Self::Item) -> Result<u64,Self::Error> {
		let id = sqlx::query!("UPDATE graph SET sensor_id=? ,graph_type=? WHERE id=?",data.sensor_id,data.graph_type,id)
			.execute(&self.pool)
			.await?
			.last_insert_id();
		return Ok(id);
	}
	async fn delete(&self, id: u64) -> Result<(),Self::Error>{
		sqlx::query!("DELETE FROM graph WHERE id=?",id)
			.execute(&self.pool)
			.await?;
		Ok(())
	}
}