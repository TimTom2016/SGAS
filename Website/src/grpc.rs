
use leptos::*;
#[cfg(feature="ssr")]
pub mod ssr {
    use sgas::DoneMessage;
    pub mod sgas {
        tonic::include_proto!("sgas");
    }
}


#[server]
pub async fn add_new_sensor(name: String,r#type: String, pin: i32, addr: String) -> Result<(),ServerFnError<String>>
{
	use crate::shared::app_state::AppState;
	use tonic::Request;
	use crate::grpc::ssr::sgas;
	let mut grpc = use_context::<AppState>().unwrap().grpc;
	let request = Request::new(sgas::AddNewSensorMessage {
		name,
		r#type,
		pin,
		addr,
	});
	grpc.new_sensor_request(request).await.map_err(|e| ServerFnError::WrappedServerError(e.to_string()))?;
	Ok(())
}
#[server]
pub async fn test_sensor_create() -> Result<(),ServerFnError<String>>
{
	add_new_sensor("Test".to_string(), "GPIO".to_string(), 6, "".to_string()).await?;
    Ok(())
} 