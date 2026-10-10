use std::hash::Hash;

use terminal_size::{terminal_size, Height, Width};

use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Rect, Direction};
use ratatui::style::{Style, Stylize};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, BorderType};

//минимальные размеры: w: 86, h: 49

#[allow(dead_code, unused)]
fn check_size() -> Result<(usize, usize), String> {
    let size = terminal_size();

    if let Some((Width(w), Height(h))) = size {
        

        if h < 49 || w < 86 {
            return Err("Small size".to_string());
        }
        return Ok((w as usize, h as usize));
        
    }
    Err("Error with catching size".to_string())

}

pub fn render(frame: &mut Frame) {
    let areas = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Percentage(20),
            Constraint::Percentage(80),
        ])
        .split(frame.area());

    let overall = areas[0];
    let areas = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![
            Constraint::Percentage(30),
            Constraint::Percentage(40),
            Constraint::Percentage(30),
        ])
        .split(areas[1]);
    let cell_data = areas[0];

    let center = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Percentage(75),
            Constraint::Percentage(25),
        ])
        .split(areas[1]);
    let map = center[0];
    let log = center[1];

    let right = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Percentage(50),
            Constraint::Percentage(50),
        ])
        .split(areas[2]);
    let player_army = right[0];
    let enemy_army = right[1];

    render_block(frame, overall, "overall");
    render_block(frame, cell_data, "cell data");
    render_block(frame, map, "map");
    render_block(frame, log, "log");
    render_block(frame, player_army, "player's army");
    render_block(frame, enemy_army, "enemy's army");
}

#[allow(unused)]
pub fn render_default(frame: &mut Frame) {

    let vertical = Layout::vertical([Constraint::Length(1), Constraint::Fill(1)]).spacing(1);
    let horizontal = Layout::horizontal([Constraint::Percentage(33); 3]).spacing(1);
    let [top, main] = frame.area().layout(&vertical);
    let [left, middle, right] = main.layout(&horizontal);

    let title = Line::from_iter([
        Span::from("Block Widget").bold(),
        Span::from(" (Press 'q' to quit)"),
    ]);
    frame.render_widget(title.centered(), top);

    render_block(frame, left, "a");
    render_block(frame, middle, "b");
    render_block(frame, right, "c");
}

/// Render a block with borders.
pub fn render_block(frame: &mut Frame, area: Rect, title: &str) {
    let block = Block::bordered().title(title);
    frame.render_widget(block, area);
}

