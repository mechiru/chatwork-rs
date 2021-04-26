use chatwork::{v2::rooms::files, Client, File};

use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let room_id = std::env::var("CHATWORK_ROOM_ID")?;
    let api_token = std::env::var("CHATWORK_API_TOKEN")?;

    let client = Client::new(api_token);
    let svc = client.v2().rooms().files(room_id.parse()?);

    let file = File::text("fuga", "hoge fuga")?;
    let file_id = svc.create(files::Create::new(file).message("hoge fuga")).await?;
    println!("file id: {:?}", file_id);

    let file = svc.get(file_id.file_id, &files::Get { create_download_url: Some(true) }).await?;
    println!("file: {:?}", file);

    Ok(())
}
