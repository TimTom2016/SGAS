use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use crate::shared::graph_types::GraphTypes;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize,FromRow)]
pub struct Graph {
	pub id: Option<i64>,
    pub name: String,
    pub sensor_id: i64,
    pub graph_type: GraphTypes, 
}