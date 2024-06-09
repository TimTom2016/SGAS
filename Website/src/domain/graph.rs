use serde::{Deserialize, Serialize};
use sqlx::{prelude::Type, FromRow};

#[derive(Type,Clone,Serialize,Deserialize,Debug,PartialEq,Eq)]
#[repr(i32)]
pub enum GraphTypes {
    BasicLine,
    SmoothedLine,
    BasicArea,
    BasicBar,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize,FromRow)]
pub struct Graph {
	pub id: Option<i64>,
    pub sensor_id: i64,
    pub graph_type: GraphTypes, 
}