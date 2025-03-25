use actix_web::{test, web, App, http::StatusCode};
use rust_be_template::routes;

#[actix_web::test]
async fn test_index_returns_200() {
    // Create test app
    let app = test::init_service(
        App::new().route("/", web::get().to(routes::index))
    ).await;
    
    // Create test request
    let req = test::TestRequest::get().uri("/").to_request();
    
    // Perform the request
    let resp = test::call_service(&app, req).await;
    
    // Assert
    assert_eq!(resp.status(), StatusCode::OK);
}