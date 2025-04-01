use std::{collections::HashMap, sync::Arc};

use redis::{Client, Connection, RedisError};
use tokio::sync::Mutex;
use tokio_tungstenite::tungstenite::Message;

type PeerMap = Arc<Mutex<HashMap<std::net::SocketAddr, tokio::sync::mpsc::UnboundedSender<Message>>>>;

pub async fn setup_redis(client: Client, peers: PeerMap ) -> Result<Connection, RedisError> {
    let redis_con = client.get_connection()?;

    Ok(redis_con)
}

async fn handle_redis_subscription(redis_con: Connection, peers: PeerMap) {}

pub async fn publish_message(redis_pub: Connection, room: &str, message: String) {}