use chatwork::{v2::rooms::messages, Client};

use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let room_id = std::env::var("CHATWORK_ROOM_ID")?;
    let api_token = std::env::var("CHATWORK_API_TOKEN")?;

    let client = Client::new(api_token);
    let svc = client.v2().rooms().messages(room_id.parse()?);

    let message_id = svc.create(&messages::Create { body: "hello", ..Default::default() }).await?;
    println!("message_id: {:?}", message_id);

    let message = svc.get(message_id.message_id).await?;
    println!("message: {:?}", message);

    Ok(())
}
