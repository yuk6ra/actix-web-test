use actix_web::{
    get, post, web, App, HttpResponse, HttpServer, Responder
};

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {name}!")
}

#[post("/echo")]
async fn echo(req: String) -> impl Responder {
    HttpResponse::Ok().body(req)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(greet)
            .service(echo)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}