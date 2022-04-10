use startup::start_ui;
use std::{error::Error, sync::Arc};
mod app;
mod keyboard;
use app::*;
use keyboard::IoEvent;
mod startup;
use crate::keyboard::IoAsyncHandler;
use log::LevelFilter;
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // setup terminal
    let (sync_io_tx, mut sync_io_rx) = tokio::sync::mpsc::channel::<IoEvent>(100);
    tui_logger::init_logger(LevelFilter::Debug).unwrap();
    tui_logger::set_default_level(log::LevelFilter::Debug);
    // create app and run it
    //let app = App::new(sync_io_tx);
    let app = Arc::new(tokio::sync::Mutex::new(App::new(sync_io_tx.clone())));
    let app_ui = Arc::clone(&app);

    tokio::spawn(async move {
        let mut handler = IoAsyncHandler::new(app);
        while let Some(io_event) = sync_io_rx.recv().await {
            handler.handle_io_event(io_event).await;
        }
    });
    start_ui(&app_ui).await?;
    // restore terminal

    Ok(())
}
