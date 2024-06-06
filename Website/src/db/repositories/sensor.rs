use std::collections::HashSet;
use sqlx::mysql::MySqlPool;
use crate::domain::sensor::Sensor;

use super::base::BaseRepository;

#[derive(Clone)]
pub struct SensorRepository
{
	pub pool: MySqlPool,
}

#[derive(sqlx::FromRow, Clone)]
pub struct SqlPermissionTokens {
	pub token: String,
}

impl BaseRepository for SensorRepository
{
	type Item = Sensor;
	async fn get_all(&self) -> Result<Vec<Self::Item>,Self::Error> {
		todo!("Invalid");
	}
	async fn get_by_id(&self, id: u64) -> Result<Self::Item,Self::Error> {
		let user: Sensor = sqlx::query_as("SELECT * FROM sensor WHERE sensorId = ?")
			.bind(id)
			.fetch_one(&self.pool)
			.await?;
		Ok(user)
	}
	async fn create(&self, data: Self::Item) -> Result<u64,Self::Error> {
		todo!("Invalid");
		let id = sqlx::query("INSERT INTO sensor (name, type, pin, addr) VALUES (?, ?, ?, ?)")
			.bind(data.name)
			.bind(data.sensor_type)
			.bind(data.pin)
			.bind(data.address)
			.execute(&self.pool)
			.await?
			.last_insert_id();
		return Ok(id);
	}
	async fn update(&self, id: u64, data: Self::Item) -> Result<u64,Self::Error> {
		todo!("");
	}
	async fn delete(&self, id: u64) -> Result<(),Self::Error>{
		todo!("");
	}
}