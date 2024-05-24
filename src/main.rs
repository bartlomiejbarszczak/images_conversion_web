mod conversion;
mod file_hosting_handlers;
mod handlers;
mod database_handlers;

use actix_web::{App, HttpServer, web};
use handlers::{greet, send_image, show_result};
use database_handlers::{db_init};


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let client = match db_init().await {
        Ok(client) => client,
        Err(error) => panic!("{}", error.to_string())
    };

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(client.clone()))
            .service(greet)
            .service(show_result)
            .service(send_image)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
