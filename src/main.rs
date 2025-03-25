use std::net::SocketAddr;
use std::{collections::HashMap, sync::Arc};
use tokio::{
    net::{TcpListener, TcpStream},
    sync::Mutex,
};
// use tokio_tungstenite::accept_async;
mod handler;
mod message;

// type Rooms = Arc<Mutex<HashMap<SocketAddr, (String)>>>;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let redis_client = redis::Client::open("redis://127.0.0.1/")?;
    let peers = Arc::new(Mutex::new(HashMap::new()));

    let listener = TcpListener::bind("127.0.0.1::8080").await?;
    println!("Server running on ws://127.0.0.1:8080");
    Ok(())
}

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error>> {
//     let client = redis::Client::open("redis:://127.0.0.1/")?;

//     let mut pub_con = client.get_connection()?;
//     let mut sub_con = client.get_connection()?;

//     let mut pub_sub = sub_con.as_pubsub();

//     let channel = String::from("room");

//     pub_sub.subscribe(&channel);

//     pub_con.publish(&channel, String::from("ki gal hai mittar"))?;

//     loop {
//         let msg = pub_sub.get_message()?;
//         let payload:String = msg.get_payload()?;
//         let channel: String = msg.get_channel()?;

//         println!("Msg Received {} from {}", payload, channel);
//     }

//     let listener = TcpListener::bind("127.0.0.1:8080").await?;
//     println!("Listening on port 8080");
//     while let Ok((stream, socketaddr)) = listener.accept().await {
//         tokio::spawn(handle_connection(stream));
//     }
//     Ok(())
// }

// async fn handle_connection(stream: TcpStream) {
//     let ws_stream = accept_async(stream)
//         .await
//         .expect("Error during websocket handshake");
//     println!("New WebSocket connection: {:?}", ws_stream);

//     let (mut write, mut read) = ws_stream.split();

//     while let Some(msg) = read.next().await {
//         match msg {
//             Ok(msg) => {
//                 println!("Received a message: {:?}", msg);
//                 write.send(msg).await.expect("Error sending message");
//             }
//             Err(e) => {
//                 println!("Error receiving message: {:?}", e);
//                 break;
//             }
//         }
//     }
// }
