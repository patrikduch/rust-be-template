use actix_web::{HttpRequest, HttpResponse, Responder};

pub async fn get_ip(req: HttpRequest) -> impl Responder {
    let ip_address = req
        .headers()
        .get("X-Forwarded-For")
        .and_then(|val| val.to_str().ok())
        .map(|s| s.split(',').next().unwrap_or("").trim().to_string())
        .or_else(|| req.headers().get("X-Real-IP").and_then(|val| val.to_str().ok()).map(String::from))
        .or_else(|| req.peer_addr().map(|addr| addr.ip().to_string()))
        .unwrap_or_else(|| "Unknown".to_string());
    
    println!("📡 Received request to /ip, returning IP: {}", ip_address);
    HttpResponse::Ok().body(format!("Your IP address: {}", ip_address))
}