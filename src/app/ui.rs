use crate::keyboard::MoveDirection;
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Borders},
    Frame,
};

use super::App;
use tui_logger::TuiLoggerWidget;
pub fn ui<B: Backend>(f: &mut Frame<B>, app: &App) {
    let size = f.size();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Ratio(2, 3), Constraint::Ratio(1, 3)].as_ref())
        .split(size);

    let chunks2 = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(5); 20].as_ref())
        .split(chunks[0]);

    // Top left inner block with green background
    for (i, xchunk) in chunks2.iter().enumerate() {
        let top_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(5); 20].as_ref())
            .split(*xchunk);

        for (j, ychunk) in top_chunks.iter().enumerate() {
            if let MoveDirection::Empty = app.grid[i][j] {
                let block = Block::default().style(Style::default().bg(Color::Green));
                f.render_widget(block, *ychunk);
            } else if let MoveDirection::Food = app.grid[i][j] {
                let block = Block::default().style(Style::default().bg(Color::Red));
                f.render_widget(block, *ychunk);
            } else {
                let block = Block::default().style(Style::default().bg(Color::Blue));
                f.render_widget(block, *ychunk);
            }
        }
    }

    let logger = TuiLoggerWidget::default()
        .style_error(Style::default().fg(Color::Red))
        .style_debug(Style::default().fg(Color::Green))
        .style_warn(Style::default().fg(Color::Yellow))
        .style_trace(Style::default().fg(Color::Gray))
        .style_info(Style::default().fg(Color::Blue))
        .block(
            Block::default()
                .title("Logs")
                .border_style(Style::default().fg(Color::White).bg(Color::Black))
                .borders(Borders::ALL),
        )
        .style(Style::default().fg(Color::White).bg(Color::Black));
    f.render_widget(logger, chunks[1]);
}
