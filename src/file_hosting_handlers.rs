use rust_dropbox::client::DBXClient;
use rust_dropbox::{DropboxError, DropboxResult, UploadOptionBuilder};


static TOKEN: &str = "<DROPBOX_TOKEN>";

pub fn download_file_from_dropbox(file_name: &str) -> Result<Vec<u8>, DropboxError> {
    let token = TOKEN;
    let client = DBXClient::new(token);

    client.download(file_name)
}

pub fn upload_file_to_dropbox(file_data: Vec<u8>, file_name: &str) -> DropboxResult<()> {
    let token = TOKEN;
    let client = DBXClient::new(token);
    let upload_options = UploadOptionBuilder::new().disallow_auto_rename().build(); //auto rename

    client.upload(file_data, file_name, upload_options)
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn download_file_response_200() -> Result<(), String> {
        let file_name = "/bartekANG.jpg";
        let content = download_file_from_dropbox(file_name);

        match content.is_ok() {
            true => Ok(()),
            false => Err(String::from(format!("{content:?}"))),
        }
    }

    #[test]
    fn download_file_response_failed() -> Result<(), String> {
        let file_name = "/not_exist.not_exist";

        let content = download_file_from_dropbox(file_name);

        match content.is_err() {
            true => Ok(()),
            false => Err(String::from(
                "File should not exist and response code should be different than 200",
            )),
        }
    }

    #[test]
    fn upload_file_response_200() -> Result<(), String> {
        let data = vec![97u8, 98, 99, 32, 116, 101, 115, 116, 32, 116, 101, 115, 116, 32, 116, 101, 115, 116, 32, 116, 101, 115, 116];
        let file_name = "/example_name.txt";

        let result = upload_file_to_dropbox(data, file_name);

        match result.is_ok() {
            true => Ok(()),
            false => Err(String::from(format!("{result:?}")))
        }
    }
}
