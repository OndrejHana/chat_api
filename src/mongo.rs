use crate::model::*;

use dotenv::dotenv;
use std::env;

use mongodb::{options::ClientOptions, Client, Collection, error::Result, bson::{DateTime, oid::ObjectId }};

const DB_NAME: &str = "TextApp";
const USER_COL: &str = "User";
const CHATS_COL: &str = "Chats";

#[derive(Clone)]
pub struct MongoCon {
    pub client: Client,
}

impl MongoCon {
    /// Creates new MongoCon struct instance, which includes client for using mongodb
    pub async fn new() -> mongodb::error::Result<MongoCon> {
        dotenv().ok();

        let client_options =
            ClientOptions::parse(env::var("CONN_STRING").expect("CONN_STRING could not be found"))
                .await?;
        let client: Client = Client::with_options(client_options)?;

        Ok(MongoCon { client })
    }
    
    /// fetches all users from database as Vec<Uzivatel>
    pub async fn get_users(&self) -> Result<Vec<User>> {
        let uzivatel_col: Collection<User> = self.client.database(DB_NAME).collection(USER_COL);
        let mut uzivatele_cursor: mongodb::Cursor<User> = uzivatel_col.find(None, None).await?;
        
        let mut uzivatele: Vec<User> = Vec::new();
        while uzivatele_cursor.advance().await? {
            let uz = uzivatele_cursor.deserialize_current()?;
            uzivatele.push(uz);
        }

        Ok(uzivatele)
    }

    /// takes username and creates new user in the database with current datetime and empty chat
    /// pool
    pub async fn create_user(&self, username: String) -> Result<()> {
        let user_col: Collection<User> = self.client.database(DB_NAME).collection(USER_COL);
        
        let new_user = User {
            _id: ObjectId::new(),
            username,
            created: DateTime::now(),
            chats: Vec::new(),
        };

        user_col.insert_one(new_user, None).await?;

        Ok(())
    }
}










