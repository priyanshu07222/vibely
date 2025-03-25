use futures_util::StreamExt;
use redis::Connection;
use std::{collections::HashMap, net::SocketAddr, sync::Arc};
use tokio::{net::TcpStream, sync::Mutex};
use tokio_tungstenite::{accept_async, tungstenite::Message};

type PeerMap = Arc<Mutex<HashMap<SocketAddr, tokio::sync::mpsc::UnboundedSender<Message>>>>;

pub async fn handle_ws_connection(stream: TcpStream, addr: SocketAddr, peers:PeerMap, mut redis_pub: Connection) -> Result<(), Box<dyn std::error::Error>> {
    let ws_stream = accept_async(stream)
        .await
        .expect("Error during websocket handshake");

    let (mut write, mut read) = ws_stream.split();
    Ok(())
}
