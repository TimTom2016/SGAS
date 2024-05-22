pub trait BaseRepository {
    type Item;
    type Error = sqlx::Error;
    async fn get_all(&self) -> Result<Vec<Self::Item>,Self::Error>;
    async fn get_by_id(&self, id: u64) -> Result<Self::Item,Self::Error>;
    async fn create(&self, data: Self::Item) -> Result<u64,Self::Error>;
    async fn update(&self, id: u64, data: Self::Item) -> Result<u64,Self::Error>;
    async fn delete(&self, id: u64) -> Result<(),Self::Error>;
}
