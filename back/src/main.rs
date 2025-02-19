use actix_web::{web, App, HttpServer, Responder, HttpResponse};

// Handler for the root route
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello, World!")
}

// The #[actix_web::main] macro sets up the async runtime for the main function
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))  // Define the route and associate it with the handler
    })
    .bind("127.0.0.1:8080")?  // Bind to the address and port
    .run()  // Run the server
    .await
}
