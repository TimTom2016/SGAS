
use leptos::*;
#[cfg(feature="ssr")]
pub mod ssr {
    pub mod sgas {
        tonic::include_proto!("sgas");
    }
}


#[server]
pub async fn add_new_sensor_pin(name: String,r#type: String, pin: i32) -> Result<(),ServerFnError<String>>
{
	use crate::shared::app_state::AppState;
	use tonic::Request;
	use crate::grpc::ssr::sgas;
	let mut grpc = use_context::<AppState>().unwrap().grpc;
	let request = Request::new(sgas::AddNewSensorMessage {
		name,
		r#type,
		pin: Some(pin),
		addr: None,
	});
	grpc.new_sensor_request(request).await.map_err(|e| ServerFnError::WrappedServerError(e.to_string()))?;
	Ok(())
}

#[server]
pub async fn add_new_sensor_addr(name: String,r#type: String,addr: String) -> Result<(),ServerFnError<String>>
{
	use crate::shared::app_state::AppState;
	use tonic::Request;
	use crate::grpc::ssr::sgas;
	let mut grpc = use_context::<AppState>().unwrap().grpc;
	let request = Request::new(sgas::AddNewSensorMessage {
		name,
		r#type,
		pin: None,
		addr: Some(addr),
	});
	grpc.new_sensor_request(request).await.map_err(|e| ServerFnError::WrappedServerError(e.to_string()))?;
	Ok(())
}

#[server]
pub async fn delete_sensor(id: i32) -> Result<(),ServerFnError<String>>
{
	use crate::shared::app_state::AppState;
	use tonic::Request;
	use crate::grpc::ssr::sgas;
	let mut grpc = use_context::<AppState>().unwrap().grpc;
	let request = Request::new(sgas::DeleteSensorMessage {
		sensor_id: id
	});
	grpc.delete_sensor_request(request).await.map_err(|e| ServerFnError::WrappedServerError(e.to_string()))?;
	Ok(())
}


#[server]
pub async fn get_supported_sensor_types() -> Result<Vec<String>,ServerFnError<String>>
{
	use crate::shared::app_state::AppState;
	use tonic::Request;
	use crate::grpc::ssr::sgas;
	let mut grpc = use_context::<AppState>().unwrap().grpc;
	let request = Request::new(sgas::GetSupportedSensorTypesMessage {
	});
	Ok(grpc.get_supported_sensor_types(request).await.map_err(|e| ServerFnError::WrappedServerError(e.to_string()))?.into_inner().supported_sensor_types)
}




#[server]
pub async fn test_sensor_create() -> Result<(),ServerFnError<String>>
{
	add_new_sensor_pin("Test".to_string(), "GPIO".to_string(), 6).await?;
    Ok(())
} 