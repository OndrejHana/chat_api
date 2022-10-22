use crate::model::Uzivatel;

use dotenv::dotenv;
use std::env;

use mongodb::{options::ClientOptions, Client, Collection };

const DB_NAME: &str = "Zpravy";
const COL_NAME: &str = "uzivatel";

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
    pub async fn get_users(&self) -> mongodb::error::Result<Vec<Uzivatel>> {
        let uzivatel_col: Collection<Uzivatel> = self.client.database(DB_NAME).collection(COL_NAME);
        let mut uzivatele_cursor: mongodb::Cursor<Uzivatel> = uzivatel_col.find(None, None).await?;
        
        let mut uzivatele: Vec<Uzivatel> = Vec::new();
        while uzivatele_cursor.advance().await? {
            let uz: Uzivatel = uzivatele_cursor.deserialize_current()?;
            uzivatele.push(uz);
        }

        Ok(uzivatele)
    }
}
