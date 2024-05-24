use actix_web::web;
use mongodb::{Client, Collection, bson::doc};
use serde::{Deserialize, Serialize};



static DB_NAME: &str = "ImagesConversion";
static COLL_NAME: &str = "images";
static URI: &str = "mongodb+srv://bbarszczak35:<PASSWORD>@imagesconversion.kjlcdrs.mongodb.net/?retryWrites=true&w=majority&appName=ImagesConversion";



#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
struct DBImageName {
    id: u32,
    raw_name: String,
    converted_name: String,
}


pub async fn db_init() -> mongodb::error::Result<Client> {
    let password = match std::env::var("ImagesConversionWeb_PASSWORD") {
        Ok(var) => var,
        Err(error) => return Err(mongodb::error::Error::custom(error.to_string()))
    };

    let uri = URI.replace("<PASSWORD>", &password);

    let client = Client::with_uri_str(uri).await;

    client
}


pub async fn get_name_from_db(unique_id: u32, client: &web::Data<Client>) -> Result<String, String>{
    let collection: Collection<DBImageName> = client.database(DB_NAME).collection(COLL_NAME);

    match collection.find_one(doc! {"id": unique_id}, None).await {
        Ok(Some(record)) => Ok(record.raw_name),
        Ok(None) => Err("such id does not exist".to_string()),
        Err(error) => Err(error.to_string())
    }
}


pub async fn update_document_in_db(unique_id: u32, client: &web::Data<Client>, new_name: &String) -> Result<(), String> {
    let collection: Collection<DBImageName> = client.database(DB_NAME).collection(COLL_NAME);
    let query = doc! {"id": unique_id};
    let update = doc! {"$set": {"converted_name": new_name}};

    let result = collection.update_one(query, update, None).await;

    match result {
        Ok(_) => Ok(()),
        Err(error_message) => Err(error_message.to_string())
    }
}



#[cfg(test)]
mod tests {

    #[test]
    fn check_env_var() -> Result<(), String> {
        match std::env::var("ImagesConversionWeb_PASSWORD") {
            Ok(_) => Ok(()),
            Err(error) => Err(error.to_string())
        }
    }
}
