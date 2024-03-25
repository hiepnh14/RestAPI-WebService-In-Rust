use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct OperationRequest {
    x: f32,
    y: f32,
}

#[derive(Serialize)]
struct OperationResponse {
    result: f32,
}

async fn sum(req: web::Query<OperationRequest>) -> impl Responder {
    let result = req.x + req.y;
    HttpResponse::Ok().json(OperationResponse { result })
}

async fn multiply(req: web::Query<OperationRequest>) -> impl Responder {
    let result = req.x * req.y;
    HttpResponse::Ok().json(OperationResponse { result })
}

async fn substract(req: web::Query<OperationRequest>) -> impl Responder {
    let result = req.x - req.y;
    HttpResponse::Ok().json(OperationResponse { result })
}

async fn divide(req: web::Query<OperationRequest>) -> impl Responder {
    if req.y == 0.0 {
        return HttpResponse::BadRequest().body("Cannot divide by zero");
    }
    
    let result = req.x / req.y ;
    HttpResponse::Ok().json(OperationResponse { result })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/sum", web::get().to(sum))
            .route("/multiply", web::get().to(multiply))
            .route("/substract", web::get().to(substract))
            .route("/divide", web::get().to(divide))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
