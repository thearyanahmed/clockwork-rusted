use crate::spawn_app;

#[tokio::test]
async fn health_check_works() {
	let app = spawn_app().await;

	let client = reqwest::Client::new();

	let response = client
		.get(format!("{}/api/v1/health_check",&app.address))
		.send()
		.await
		.expect("failed to execute test");

	assert!(response.status().is_success());
}