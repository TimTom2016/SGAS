use std::collections::HashSet;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct User {
	pub id: i64,
	pub username: String,
	pub password: String,
	pub permissions: HashSet<String>,
}

impl Default for User {
	fn default() -> Self {
		let permissions = HashSet::new();

		Self {
			id: -1,
			username: "Guest".into(),
			password: "".into(),
			permissions,
		}
	}
}