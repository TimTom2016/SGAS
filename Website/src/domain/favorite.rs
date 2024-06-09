use serde::{Deserialize, Serialize};
use sqlx::{FromRow};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize,FromRow)]
pub struct Favorite {
	pub graph: i64,
    pub user_id: i64,
    pub x: i64,
    pub y: i64,
}