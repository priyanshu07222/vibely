use futures_util::stream::SplitSink;
use futures_util::{SinkExt, StreamExt};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::Mutex;
use tokio_tungstenite::WebSocketStream;
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::{accept_async, tungstenite::Result};

type Room =
    Arc<Mutex<HashMap<String, Vec<Arc<Mutex<SplitSink<WebSocketStream<TcpStream>, Message>>>>>>>;

#[tokio::main]
async fn main() -> Result<()> {
    let addr = "127.0.0.1:8080";
    let listener = TcpListener::bind(addr).await?;
    let rooms: Room = Arc::new(Mutex::new(HashMap::new()));
    println!("Server running on {}", addr);

    while let Ok((stream, _)) = listener.accept().await {
        let rooms = rooms.clone();
        tokio::spawn(handle_connection(stream, rooms));
    }

    Ok(())
}

async fn handle_connection(stream: TcpStream, rooms: Room) -> Result<()> {
    let ws_stream = accept_async(stream).await?;
    println!("New WebSocket connection established");

    let (write_half, mut read_half) = ws_stream.split();
    let write_half = Arc::new(Mutex::new(write_half));

    let mut current_room: Option<String> = None;

    while let Some(msg) = read_half.next().await {
        println!("hello1");
        let msg = msg?;
        if msg.is_text() {
            let text = msg.to_string();
            println!("hello1: {text}");
            if text.starts_with("JOIN:") {
                println!("someone joined with msg: {}", text);
                let room_name = text.strip_prefix("JOIN:").unwrap().to_string();
                current_room = Some(room_name.clone());

                let mut rooms_lock = rooms.lock().await;
                rooms_lock
                    .entry(room_name)
                    .or_insert_with(Vec::new)
                    .push(write_half.clone());
            } else if let Some(room) = &current_room {
                println!("hello122");
                let clients = {
                    let rooms_lock = rooms.lock().await;
                    rooms_lock.get(room).cloned()
                };

                if let Some(clients) = clients {
                    let mut disconnected_clients = vec![];
                    for (i, client) in clients.iter().enumerate() {
                        let mut client_lock = client.lock().await;
                        if client_lock
                            .send(Message::Text(text.clone().into()))
                            .await
                            .is_err()
                        {
                            disconnected_clients.push(i);
                        }
                    }

                    // Remove disconnected clients
                    let mut rooms_lock = rooms.lock().await;
                    if let Some(clients) = rooms_lock.get_mut(room) {
                        println!("DISCONNETING USER");
                        for &index in disconnected_clients.iter().rev() {
                            clients.remove(index);
                        }
                    }
                }
            }
        }
    }

    Ok(())
}












// use std::net::SocketAddr;
// use std::{collections::HashMap, sync::Arc};
// use tokio::{
//     net::{TcpListener, TcpStream},
//     sync::Mutex,
// };
// // use tokio_tungstenite::accept_async;
// mod handler;
// mod message;

// type Rooms = Arc<Mutex<HashMap<SocketAddr, (String)>>>;

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error>> {
//     let redis_client = redis::Client::open("redis://127.0.0.1/")?;
//     let peers:Rooms = Arc::new(Mutex::new(HashMap::new()));

//     let listener = TcpListener::bind("127.0.0.1::8080").await?;
//     println!("Server running on ws://127.0.0.1:8080");
//     Ok(())
// }

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
