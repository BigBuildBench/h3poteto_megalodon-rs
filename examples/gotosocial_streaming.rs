use megalodon::{generator, streaming::Message, SNS};
use std::env;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let Ok(url) = env::var("GOTOSOCIAL_URL") else {
        println!("Specify GOTOSOCIAL_URL!!");
        return;
    };
    let Ok(token) = env::var("GOTOSOCIAL_ACCESS_TOKEN") else {
        println!("Specify GOTOSOCIAL_ACCESS_TOKEN!!");
        return;
    };

    streaming(url.as_str(), token).await;
}

async fn streaming(url: &str, access_token: String) {
    let client = generator(SNS::Gotosocial, url.to_string(), Some(access_token), None);
    let streaming = client.local_streaming().await;

    streaming
        .listen(Box::new(|message| match message {
            Message::Update(mes) => {
                println!("{:#?}", mes);
            }
            Message::Notification(mes) => {
                println!("{:#?}", mes);
            }
            Message::Conversation(mes) => {
                println!("{:#?}", mes);
            }
            Message::Delete(mes) => {
                println!("message is deleted: {}", mes);
            }
            Message::StatusUpdate(mes) => {
                println!("updated: {:#?}", mes)
            }
            Message::Heartbeat() => {
                println!("heartbeat");
            }
        }))
        .await;
}
