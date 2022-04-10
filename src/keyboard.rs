//use std::time::Duration;
mod handle;
pub use handle::IoAsyncHandler;
#[derive(Clone, Copy)]
pub enum MoveDirection {
    Up,
    Down,
    Left,
    Right,
    Empty,
    Food,
}
pub enum IoEvent {
    Initialize,
    Move(MoveDirection),
}
