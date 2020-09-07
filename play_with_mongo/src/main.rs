use futures::stream::StreamExt;
use mongodb::{
    bson::{doc, Bson},
    error::Error,
    options::ClientOptions,
    Client,
};
use tokio;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let should_insert = false;

    if should_insert {
        insert_dinos().await
    } else {
        read_dinos().await
    }
}

async fn insert_dinos() -> Result<(), Error> {
    let client_options = ClientOptions::parse("mongodb://localhost:27017").await?;
    let client = Client::with_options(client_options)?;

    let db = client.database("things");

    let collection = db.collection("dinos");

    let docs = vec![
        doc! { "name": "Tyrannosaurus rex", "carnivore": true },
        doc! { "name": "Triceratops", "carnivore": false },
    ];

    collection.insert_many(docs, None).await?;

    println!("Inserted dinos!");

    Ok(())
}

async fn read_dinos() -> Result<(), Error> {
    let client_options = ClientOptions::parse("mongodb://localhost:27017").await?;
    let client = Client::with_options(client_options)?;

    let db = client.database("things");

    let collection = db.collection("dinos");

    let mut dinos_cursor = collection.find(None, None).await?;

    while let Some(result) = dinos_cursor.next().await {
        match result {
            Ok(document) => {
                if let Some(name) = document.get("name").and_then(Bson::as_str) {
                    println!("name: {}", name);
                } else {
                    println!("no name found");
                }
            }
            Err(e) => return Err(e.into()),
        }
    }

    Ok(())
}
