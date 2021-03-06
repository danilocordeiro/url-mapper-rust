use anyhow::Result;
use crate::config::CONFIG;
use crate::db::{DB, UrlMap, Message, Manager};   
use tracing::{info, error, subscriber::set_global_default};
use tracing_subscriber::FmtSubscriber;
use server::Server;
use std::process;

mod config;
mod db;
mod server;

#[tokio::main]
async fn main() -> Result<()> {
    let subscriber = FmtSubscriber::new();
    set_global_default(subscriber)?;

    info!(
        "host: {}, port: {}, database.url: {}",
        CONFIG.host, CONFIG.port, CONFIG.database.url
    );

    let db = DB::new().await.unwrap();
    let (db_tx, db_rx) = tokio::sync::mpsc::channel(32);
    tokio::spawn(async move {
        let mut manager = Manager::new(db, db_rx);
        manager.listen().await;
    });

    tokio::spawn(async move {
        use tokio::signal::unix::{signal, SignalKind};
        let mut hup = signal(SignalKind::hangup()).unwrap();
        let mut int = signal(SignalKind::interrupt()).unwrap();
        let mut quit = signal(SignalKind::quit()).unwrap();
        let mut term = signal(SignalKind::terminate()).unwrap();
    
        tokio::select! {
            _ = hup.recv() => tracing::info!("Recieved SIGHUP!"),
            _ = int.recv() => tracing::info!("Recieved SIGINT!"),
            _ = quit.recv() => tracing::info!("Recieved SIGQUIT!"),
            _ = term.recv() => tracing::info!("Recieved SIGTERM!"),
        }

        tracing::info!("Good bye from Url Mapper in Rust");
        process::exit(0);
    });

    Server::new(db_tx).listen().await?;

    Ok(())

}
