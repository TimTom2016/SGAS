use std::collections::HashSet;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize,FromRow)]
pub struct Sensor {
	pub id: Option<i64>,
	pub username: String,
	pub password: String,
	pub permissions: HashSet<String>,
}

impl Default for Sensor {
	fn default() -> Self {
		let permissions = HashSet::new();

		Self {
            id: -1
			username: "Guest".into(),
			password: "".into(),
			permissions,
		}
	}
}