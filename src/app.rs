pub(crate) mod ui;
use log::error;
use crate::keyboard::IoEvent;
#[derive(Clone)]
pub struct SinSignal {
    x: f64,
    interval: f64,
    period: f64,
    scale: f64,
}

impl SinSignal {
    pub fn new(interval: f64, period: f64, scale: f64) -> SinSignal {
        SinSignal {
            x: 0.0,
            interval,
            period,
            scale,
        }
    }
}

impl Iterator for SinSignal {
    type Item = (f64, f64);
    fn next(&mut self) -> Option<Self::Item> {
        let point = (self.x, (self.x * 1.0 / self.period).sin() * self.scale);
        self.x += self.interval;
        Some(point)
    }
}

pub struct App {
    io_tx: tokio::sync::mpsc::Sender<IoEvent>,
    signal1: SinSignal,
    data1: Vec<(f64, f64)>,
    signal2: SinSignal,
    data2: Vec<(f64, f64)>,
    window: [f64; 2],
}

impl App {
    pub fn new(io_tx: tokio::sync::mpsc::Sender<IoEvent>) -> App {
        let mut signal1 = SinSignal::new(0.2, 3.0, 18.0);
        let mut signal2 = SinSignal::new(0.1, 2.0, 10.0);
        let data1 = signal1.by_ref().take(200).collect::<Vec<(f64, f64)>>();
        let data2 = signal2.by_ref().take(200).collect::<Vec<(f64, f64)>>();
        App {
            io_tx,
            signal1,
            data1,
            signal2,
            data2,
            window: [0.0, 20.0],
        }
    }
    pub async fn dispatch(&mut self, action: IoEvent) {
        // `is_loading` will be set to false again after the async action has finished in io/handler.rs
        //self.is_loading = true;
        if let Err(e) = self.io_tx.send(action).await {
            error!("Error from dispatch {}", e);
        };
    }
    pub fn on_tick(&mut self) {
        for _ in 0..5 {
            self.data1.remove(0);
        }
        self.data1.extend(self.signal1.by_ref().take(5));
        for _ in 0..10 {
            self.data2.remove(0);
        }
        self.data2.extend(self.signal2.by_ref().take(10));
        self.window[0] += 1.0;
        self.window[1] += 1.0;
    }
}
