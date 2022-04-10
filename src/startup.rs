use std::io::stdout;
use std::sync::Arc;

use crate::app::App;
use anyhow::Result;
//use inputs::events::Events;
//use inputs::InputEvent;
use crossterm::event::{self, Event, KeyCode};
use std::time::{Duration, Instant};
use tui::backend::CrosstermBackend;

use crate::app::ui::ui;
use crate::keyboard::{IoEvent, MoveDirection};
use tui::Terminal;
pub async fn start_ui(app: &Arc<tokio::sync::Mutex<App>>) -> Result<()> {
    // Configure Crossterm backend for tui
    let stdout = stdout();
    crossterm::terminal::enable_raw_mode()?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;
    terminal.hide_cursor()?;

    let mut last_tick = Instant::now();
    // User event handler
    let tick_rate = Duration::from_millis(200);
    //let _: Result<()> = async {
    loop {
        let mut app = app.lock().await;
        terminal.draw(|f| ui(f, &app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));
        if !app.con {
            break;
        }
        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => {
                        break;
                    }
                    KeyCode::Left => {
                        app.dispatch(IoEvent::Move(MoveDirection::Left)).await;
                    }
                    KeyCode::Right => {
                        app.dispatch(IoEvent::Move(MoveDirection::Right)).await;
                    }
                    KeyCode::Up => {
                        app.dispatch(IoEvent::Move(MoveDirection::Up)).await;
                    }
                    KeyCode::Down => {
                        app.dispatch(IoEvent::Move(MoveDirection::Down)).await;
                    }
                    KeyCode::Char(_) => {
                        app.dispatch(crate::keyboard::IoEvent::Initialize).await;
                    }
                    _ => {}
                }
            }
        } else {
            app.dispatch(IoEvent::Move(MoveDirection::Empty)).await;
        }
        if last_tick.elapsed() >= tick_rate {
            //app.on_tick();
            last_tick = Instant::now();
        }
    }
    //}
    terminal.clear()?;
    terminal.show_cursor()?;
    crossterm::terminal::disable_raw_mode()?;
    Ok(())
    // Restore the terminal and close application
}
