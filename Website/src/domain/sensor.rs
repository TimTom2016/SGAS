use serde::{Deserialize, Serialize};
use sqlx::FromRow;
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize,FromRow)]
pub struct Sensor {
	#[sqlx(rename="sensorId")]
	pub id: Option<i64>,
	pub name: String,
	#[sqlx(rename="type")]
	pub sensor_type: String,
	pub pin: Option<u8>,
	#[sqlx(rename="addr")]
	pub address: Option<String>,
}

impl Sensor {
	pub fn new_pin(name: String, sensor_type: String, pin: u8) -> Self {
		Self {
			id: None,
			name,
			sensor_type,
			pin: Some(pin),
			address: None,
		}
	}
	pub fn new_addr(name: String, sensor_type: String, address: String) -> Self {
		Self {
			id: None,
			name,
			sensor_type,
			pin: None,
			address: Some(address),
		}
	}
}