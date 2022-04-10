use std::sync::Arc;
use std::time::Duration;

use anyhow::Result;
use log::{error, info};

use super::{IoEvent, MoveDirection};
use crate::app::App;
use rand::Rng;
/// In the IO thread, we handle IO event without blocking the UI thread
pub struct IoAsyncHandler {
    app: Arc<tokio::sync::Mutex<App>>,
}

impl IoAsyncHandler {
    pub fn new(app: Arc<tokio::sync::Mutex<App>>) -> Self {
        Self { app }
    }

    /// We could be async here
    pub async fn handle_io_event(&mut self, io_event: IoEvent) {
        let result = match io_event {
            IoEvent::Initialize => self.do_initialize().await,
            IoEvent::Move(direction) => self.do_move(direction).await,
        };

        if let Err(err) = result {
            error!("Oops, something wrong happen: {:?}", err);
        }

        //let mut app = self.app.lock().await;
        //app.loaded();
    }

    /// We use dummy implementation here, just wait 1s
    async fn do_initialize(&mut self) -> Result<()> {
        info!("🚀 Initialize the application");
        //let mut app = self.app.lock().await;
        tokio::time::sleep(Duration::from_nanos(1000)).await;
        //app.initialized(); // we could update the app state
        info!("👍 Application initialized");

        Ok(())
    }
    async fn do_move(&mut self, direction: MoveDirection) -> Result<()> {
        let mut app = self.app.lock().await;
        if !app.hasfood {
            loop {
                let selectx: usize = rand::thread_rng().gen_range(0..20);
                let selecty: usize = rand::thread_rng().gen_range(0..20);
                if let MoveDirection::Empty = app.grid[selectx][selecty] {
                    app.grid[selectx][selecty] = MoveDirection::Food;
                    app.hasfood = true;
                    break;
                }
            }
        }
        let (mut startx, mut starty) = app.start;
        let (endx, endy) = app.end;
        let mut direction = direction;
        //if let MoveDirection::Empty = direction {
        //    direction = app.grid[startx][starty];
        //}
        match (direction, app.grid[startx][starty]) {
            (MoveDirection::Up, MoveDirection::Down)
            | (MoveDirection::Down, MoveDirection::Up)
            | (MoveDirection::Left, MoveDirection::Right)
            | (MoveDirection::Right, MoveDirection::Left)
            | (MoveDirection::Empty, _) => {
                direction = app.grid[startx][starty];
            }
            _ => {}
        }
        match direction {
            MoveDirection::Up => {
                if starty == 0 {
                    starty = 19;
                } else {
                    starty -= 1;
                }
            }
            MoveDirection::Down => {
                if starty == 19 {
                    starty = 0;
                } else {
                    starty += 1;
                }
            }
            MoveDirection::Left => {
                if startx == 0 {
                    startx = 19;
                } else {
                    startx -= 1;
                }
            }
            MoveDirection::Right => {
                if startx == 19 {
                    startx = 0;
                } else {
                    startx += 1;
                }
            }
            MoveDirection::Empty | MoveDirection::Food => {}
        }
        if let MoveDirection::Empty = app.grid[startx][starty] {
            app.grid[startx][starty] = direction;
        } else if let MoveDirection::Food = app.grid[startx][starty] {
            app.grid[startx][starty] = direction;
            match direction {
                MoveDirection::Up => {
                    if starty == 0 {
                        starty = 19;
                    } else {
                        starty -= 1;
                    }
                }
                MoveDirection::Down => {
                    if starty == 19 {
                        starty = 0;
                    } else {
                        starty += 1;
                    }
                }
                MoveDirection::Left => {
                    if startx == 0 {
                        startx = 19;
                    } else {
                        startx -= 1;
                    }
                }
                MoveDirection::Right => {
                    if startx == 19 {
                        startx = 0;
                    } else {
                        startx += 1;
                    }
                }
                MoveDirection::Empty | MoveDirection::Food => {}
            }
            app.hasfood = false;
            if let MoveDirection::Empty = app.grid[startx][starty] {
                app.grid[startx][starty] = direction;
            } else {
                app.con = false;
                return Ok(());
            }
        } else {
            app.con = false;
            return Ok(());
        }
        let (nstartx, nstarty) = (startx, starty);
        loop {
            tokio::time::sleep(Duration::from_nanos(10)).await;
            match direction {
                MoveDirection::Up => {
                    let (bstartx, bstarty) = (startx, starty);
                    if starty == 19 {
                        starty = 0;
                    } else {
                        starty += 1;
                    }
                    let (localx, mut localy) = (startx, starty);
                    if localy == 0 {
                        localy = 19;
                    } else {
                        localy -= 1;
                    }
                    info!("⏰Up");
                    direction = app.grid[startx][starty];
                    app.grid[localx][localy] = MoveDirection::Up;
                    app.grid[startx][starty] = MoveDirection::Empty;
                    if (startx, starty) == (endx, endy) {
                        app.end = (bstartx, bstarty);
                        break;
                    }
                }
                MoveDirection::Down => {
                    let (bstartx, bstarty) = (startx, starty);
                    info!("⏰{},{} Down!", startx, starty);
                    if starty == 0 {
                        starty = 19;
                    } else {
                        starty -= 1;
                    }
                    let (localx, mut localy) = (startx, starty);
                    if localy == 19 {
                        localy = 0;
                    } else {
                        localy += 1;
                    }
                    info!("⏰{},{} Down!", startx, starty);
                    direction = app.grid[startx][starty];
                    app.grid[localx][localy] = MoveDirection::Down;
                    app.grid[startx][starty] = MoveDirection::Empty;
                    if (startx, starty) == (endx, endy) {
                        app.end = (bstartx, bstarty);
                        break;
                    }
                }
                MoveDirection::Left => {
                    let (bstartx, bstarty) = (startx, starty);
                    if startx == 19 {
                        startx = 0;
                    } else {
                        startx += 1;
                    }

                    let (mut localx, localy) = (startx, starty);
                    if localx == 0 {
                        localx = 19;
                    } else {
                        localx -= 1;
                    }
                    info!("⏰Move Left");
                    direction = app.grid[startx][starty];
                    app.grid[localx][localy] = MoveDirection::Left;
                    app.grid[startx][starty] = MoveDirection::Empty;
                    if (startx, starty) == (endx, endy) {
                        app.end = (bstartx, bstarty);
                        break;
                    }
                }
                MoveDirection::Right => {
                    let (bstartx, bstarty) = (startx, starty);
                    if startx == 0 {
                        startx = 19;
                    } else {
                        startx -= 1;
                    }
                    let (mut localx, localy) = (startx, starty);
                    if localx == 19 {
                        localx = 0;
                    } else {
                        localx += 1;
                    }
                    info!("⏰ Right!");
                    direction = app.grid[startx][starty];
                    app.grid[localx][localy] = MoveDirection::Right;
                    app.grid[startx][starty] = MoveDirection::Empty;
                    if (startx, starty) == (endx, endy) {
                        app.end = (bstartx, bstarty);
                        break;
                    }
                }
                MoveDirection::Empty | MoveDirection::Food => {
                    info!("⏰Empty");
                    break;
                }
            }
        }
        app.start = (nstartx, nstarty);
        Ok(())
    }
    //async fn do_sleep(&mut self, duration: Duration) -> Result<()> {
    //    info!("😴 Go sleeping for {:?}...", duration);
    //    tokio::time::sleep(duration).await;
    //    info!("⏰ Wake up !");
    //    // Notify the app for having slept
    //    //let mut app = self.app.lock().await;
    //    //app.slept();

    //    Ok(())
    //}
}
