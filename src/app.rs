pub(crate) mod ui;
use crate::keyboard::{IoEvent, MoveDirection};
use log::error;

pub struct App {
    io_tx: tokio::sync::mpsc::Sender<IoEvent>,
    pub grid: [[MoveDirection; 20]; 20],
    pub start: (usize, usize),
    pub end: (usize, usize),
    pub con: bool,
}

impl App {
    pub fn new(io_tx: tokio::sync::mpsc::Sender<IoEvent>) -> App {
        let mut grid = [[MoveDirection::Empty; 20]; 20];
        grid[0][0] = MoveDirection::Right;
        grid[1][0] = MoveDirection::Right;
        grid[2][0] = MoveDirection::Right;
        grid[3][0] = MoveDirection::Right;
        grid[4][0] = MoveDirection::Right;
        grid[5][0] = MoveDirection::Right;
        grid[6][0] = MoveDirection::Right;
        grid[7][0] = MoveDirection::Right;
        grid[8][0] = MoveDirection::Right;
        grid[9][0] = MoveDirection::Right;
        App {
            io_tx,
            grid,
            start: (9, 0),
            end: (0, 0),
            con: true,
        }
    }
    pub async fn dispatch(&mut self, action: IoEvent) {
        // `is_loading` will be set to false again after the async action has finished in io/handler.rs
        //self.is_loading = true;
        if let Err(e) = self.io_tx.send(action).await {
            error!("Error from dispatch {}", e);
        };
    }
}
