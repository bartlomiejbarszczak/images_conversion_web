use crate::conversion::{do_conversion, ConversionMode};
use crate::file_hosting_handlers::{download_file_from_dropbox, upload_file_to_dropbox};
use crate::database_handlers::{get_name_from_db, update_document_in_db};
use actix_web::{get, post, web, HttpResponse, Responder};
use mongodb::Client;
use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct AuthRequest {
    id: u32,
    pub(crate) conversion_type: ConversionMode,
}


// ---------------------------------------------------------
//                          GET
// ---------------------------------------------------------
#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    let end_string = format!("Hello {name}!\t");

    HttpResponse::Ok().body(end_string)
}

#[get("/show_result/{image_id}")]
async fn show_result(path: web::Path<String>, client: web::Data<Client>) -> impl Responder {
    let unique_id = match path.into_inner().parse::<u32>() {
        Ok(id) => id,
        Err(error) => return HttpResponse::InternalServerError().body(error.to_string())
    };

    let name = match get_name_from_db(unique_id, &client).await {
        Ok(name) => name,
        Err(error) => return HttpResponse::InternalServerError().body(error.to_string())
    };

    let photo_data = match download_file_from_dropbox(format!("/{}", name).as_str()) {
        Ok(data) => data,
        Err(error) => return HttpResponse::InternalServerError().body(format!("{error:?}"))
    };

    HttpResponse::Ok().json(photo_data)
}


// ---------------------------------------------------------
//                          POST
// ---------------------------------------------------------

#[post("/send_image")]
async fn send_image(query_data: web::Query<AuthRequest>, client: web::Data<Client>) -> impl Responder {
    let auth = query_data.into_inner();
    let unique_id = auth.id;
    let conversion_mode = auth.conversion_type;

    let image_name = match get_name_from_db(unique_id, &client).await {
        Ok(name) => name,
        Err(error_message) => return HttpResponse::InternalServerError().body(format!("{error_message}"))
    };

    let image_name_pre = format!("/{image_name}");

    let image_data = match download_file_from_dropbox(&image_name_pre) {
        Ok(data) => data,
        Err(error_message) => return HttpResponse::InternalServerError().body(format!("{error_message:?}"))
    };

    let image_buffer = match do_conversion(image_data, conversion_mode) {
        Ok(image) => image,
        Err(error_message) => return HttpResponse::InternalServerError().body(format!("{error_message:?}"))
    };

    let image_name_post = format!("converted_{image_name}");

    match update_document_in_db(unique_id, &client, &image_name_post).await {
        Ok(()) => (),
        Err(error_message) => return HttpResponse::InternalServerError().body(format!("Cannot update name in database\n{error_message:?}"))
    };

    let image_name_post = format!("/{image_name_post}");

    return match upload_file_to_dropbox(image_buffer, &image_name_post) {
        Ok(()) => HttpResponse::Ok().json(unique_id.to_string()),
        Err(error_message) => HttpResponse::InternalServerError().body(format!("Internal problem with dropbox...\n{error_message:?}"))
    };
}

