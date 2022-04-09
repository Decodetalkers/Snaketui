use std::time::Duration;
mod handle;
pub use handle::IoAsyncHandler;
#[allow(dead_code)]
pub enum IoEvent {
    Initialize,
    Sleep(Duration),
}
