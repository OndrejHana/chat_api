use crate::MongoCon;
use serde::{ Serialize, Deserialize };
use mongodb::bson::{ DateTime, oid::ObjectId };

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct User {
    pub _id: ObjectId,
    pub username: String,
    pub created: DateTime, 
    pub chats: Vec<Chat>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Chat {
    pub _id: ObjectId,
    pub user1: User,
    pub user2: User,
    pub messages: Vec<Message>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Message {
    pub _id: ObjectId,
    pub sender: User,
    pub receiver: User,
    pub content: String,
    pub time_sent: DateTime,
}

pub struct State {
    pub mongo_con: MongoCon
}
