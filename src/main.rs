use anyhow::Result;
use crate::config::CONFIG;
use crate::db::{DB, UrlMap, Message, Manager};   

mod config;
mod db;

#[tokio::main]
async fn main() -> Result<()> {
    println!(
        "host: {}, port: {}, database.url: {}",
        CONFIG.host, CONFIG.port, CONFIG.database.url
    );

    let db = DB::new().await.unwrap();
    let (db_tx, db_rx) = tokio::sync::mpsc::channel(32);
    tokio::spawn(async move {
        let mut manager = Manager::new(db, db_rx);
        manager.listen().await;
    });

    let (tx, rx) = tokio::sync::oneshot::channel();
    // match db_tx.send(Message::GetUrlMaps { resp: tx }).await {
    //     Ok(_) => {},
    //     Err(e) => eprintln!("Failed to send to database manager: {}", e)
    // }

    // match db_tx.send(Message::GetUrlMap { key: "github".into(), resp: tx }).await {
    //     Ok(_) => {},
    //     Err(e) => eprintln!("Failed to send to database manager: {}", e)
    // }
    
    let url_map = UrlMap::new("linkedin".into(), "https://linkedin.com/danilocordeiro".into());
    match db_tx.send(Message::CreateUrlMap {url_map , resp: tx }).await {
        Ok(_) => {},
        Err(e) => eprintln!("Failed to send to database manager: {}", e)
    }

    let url_maps = rx.await.unwrap();
    match url_maps {
        Ok(ums) => println!("url_maps: {:?}", ums),
        Err(e) => eprintln!("Failed to send to database manager: {}", e)
    }

    Ok(())

}
