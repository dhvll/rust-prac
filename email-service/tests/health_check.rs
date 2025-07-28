use std::{ net::TcpListener};

#[tokio::test]
async fn health_check_works() {
    
    let address = spawn_app();
    let client = reqwest::Client::new();
    let response = client.get(&format!("{}/health_check", address))
        .send()
        .await
        .expect("Failed to send request");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port  = listener.local_addr().unwrap().port();
    let server = email_service::run(listener).expect("failed to bind address");
    let _ = tokio::spawn(server);
    format!("http://127.0.0.1:{}", port)
}

#[tokio::test]
async fn subscribe_returns_200_for_valid_form_data() {
    let app_address = spawn_app();
    let client = reqwest::Client::new();

    let body = "name=john%20doe&email=john%40doe.com";
    let respoonse = client.post(&format!("{}/subscriptions", &app_address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to send request");

    assert_eq!(200, respoonse.status().as_u16());
}

#[tokio::test]
async fn subscribe_returns_400_for_missing_form_data() {
    let app_address = spawn_app();
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=john%20doe", "missing the email"),
        ("email=john%40doe.com", "missing the name"),
        ("", "missing both name and email"),
    ];
    for (invalid_body, error_message) in test_cases {
        let response = client.post(&format!("{}/subscriptions", &app_address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to send request");

        assert_eq!(400, response.status().as_u16(), "The API did not return 400 Bad Request when the payload was {}", error_message);
        
    }
}