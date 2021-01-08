use actix_rt::spawn;
use actix_web::client;

use std::net::TcpListener;

use zero2prod::run;

#[actix_rt::test]
async fn health_check_works() {

    spawn_app();

    // let client = reqwest::Client::new();

    // let response = client
    //         .get("http://127.0.0.1:8000/healt_check")
    //         .send()
    //         .await
    //         .expect("Failed to execute request");

    // // Assert
    // assert!(response.status().is_success());
    // assert_eq!(Some(0), response.content_length());

}

fn spawn_app() -> String {

    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");

    let port = listener.local_addr().unwrap().port();

    let server = zero2prod::run(listener).expect("Failed to bind address");
    let _ = tokio::spawn(server);

    format!("http://127.0.0.1:{}", port)
}