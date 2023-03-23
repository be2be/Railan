use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Alignment},
    style::{Color, Style},
    widgets::{Block, Borders, List, Paragraph},
    Terminal,
};

use std::io::Stdout;
use std::sync::{Arc,RwLock};
use crate::game::GameState;
use crate::menu::assets;
use crate::menu::rendering::TerminalRenderer;

/// A view, which displays a fullscreen message
pub struct MainMenu;

impl TerminalRenderer for MainMenu{

    /// Draws the Main Menu in the terminal
    fn render(&mut self, terminal: &mut Terminal<CrosstermBackend<Stdout>>, menu_widget : &List, _game_state: Arc<RwLock<GameState>>) {

        let main_style : Style = Style::default().bg(Color::Rgb(50, 25, 0));

        let main = Paragraph::new(assets::read_asset("main.txt"))
            .style(Style::default().fg(Color::White))
            .alignment(Alignment::Center)
            .style(main_style)
            .block(Block::default().borders(Borders::ALL).title("Main"));

        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(15), Constraint::Percentage(85)].as_ref())
                .split(f.size());

            f.render_widget(menu_widget.clone(), chunks[0]);

            f.render_widget(main.clone(), chunks[1]);
        }).expect("Can render widget");
    }
}
