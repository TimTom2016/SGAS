use sqlx::mysql::MySqlPool;
#[derive(Debug,Clone)]
pub struct Database {
    connection_string: String,
    connection_pool: MySqlPool,
}

impl Database {
    pub async fn new(connection_pool: MySqlPool) -> Self
    {
        Self{connection_pool, connection_string: String::from("")}
    }
    pub async fn connect(connection_string: &String) -> Result<Self, sqlx::Error> {
        let pool = MySqlPool::connect(connection_string).await?;
        if pool.is_closed() {
            return Err(sqlx::Error::PoolClosed);
        }
        Ok(Self {
            connection_string: connection_string.to_string(),
            connection_pool: pool,
        })
    }
    pub fn get_pool(&self) -> &MySqlPool {
        &self.connection_pool
    }
}
