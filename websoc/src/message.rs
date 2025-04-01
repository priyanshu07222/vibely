use serde::{Serialize, Deserialize};

pub enum ClientMessage{
    Join,
    Upvote,
    Message,
    Leave,
    NextSong,
}