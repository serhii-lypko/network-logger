use futures::stream::{StreamExt, TryStreamExt};

use mongodb::{
    bson::{doc, Document},
    options::{ClientOptions, ServerApi, ServerApiVersion},
    Client, Collection,
};

use log::{debug, error, info};

use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::NetworkLogRecord;

// TODO: it's important to make sure the cloud DB storage won't be exceeded too fast -> write metrics

#[derive(Debug, Serialize, Deserialize)]
struct Book {
    // id ?
    title: String,
    author: String,
}

#[derive(Debug)]
pub struct DbClient {
    client: Client,
}

impl DbClient {
    pub async fn new() -> Result<Self> {
        // TODO: env var
        let uri = "mongodb://admin:ElaAbCQkZQbv9gpv@localhost:27017";

        let options = ClientOptions::parse_async(uri).await?;
        let client = Client::with_options(options)?;

        Ok(DbClient { client })
    }

    pub async fn read(&self) -> Result<()> {
        // client
        //     .database("test")
        //     .run_command(doc! { "ping": 1 }, None)
        //     .await?;

        // let db = client.database("test");
        // let collection = db.collection::<Document>("books");

        // let collection: Collection<Book> = client.database("test").collection("books");
        // let cursor = collection.find(None, None).await?;
        // let v: Vec<Book> = cursor.try_collect().await?;
        // println!("{:?}", v);

        // while cursor.advance().await? {
        //     println!("{:?}", cursor.deserialize_current()?);
        // }

        // -------- -------- -------- -------- -------- --------

        todo!()
    }

    pub async fn write(&self, network_log_record: NetworkLogRecord) -> Result<()> {
        // let docs = vec![
        //     doc! { "title": "1984", "author": "George Orwell" },
        //     doc! { "title": "Animal Farm", "author": "George Orwell" },
        //     doc! { "title": "The Great Gatsby", "author": "F. Scott Fitzgerald" },
        // ];

        // collection.insert_many(docs, None).await?;

        // let db = client.database("books");

        // for collection_name in db.list_collection_names(None).await? {
        //     println!("{}", collection_name);
        // }

        Ok(())
    }
}
