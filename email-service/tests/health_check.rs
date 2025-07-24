use email_service::run;

#[tokio::test]
async fn health_check_works() {
    
    spawn_app();
    let client = reqwest::Client::new();
    let response = client.get("http://127.0.0.1:8000/health_check")
        .send()
        .await
        .expect("Failed to send request");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

fn spawn_app() {
    let server = email_service::run().expect("failed to start server");
    tokio::spawn(server);
}
