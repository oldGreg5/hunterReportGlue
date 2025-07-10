use actix_files::Files;
use actix_web::{App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Serving at: http://localhost:8081");
    HttpServer::new(|| {
        App::new()
            .service(Files::new("/", "../static").index_file("index.html"))
    })
    .bind(("127.0.0.1", 8081))?
    .run()
    .await
}
