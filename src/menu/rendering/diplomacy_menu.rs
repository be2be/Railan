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
use crate::menu::rendering::TerminalRenderer;

/// A view, which displays a player's diplomacy relations
pub struct DiplomacyMenu;

impl TerminalRenderer for DiplomacyMenu{

    /// Renders the DiplomacyMenu in the terminal
    fn render(&mut self, terminal: &mut Terminal<CrosstermBackend<Stdout>>, menu_widget : &List, _game_state: Arc<RwLock<GameState>>) {

        let main_style : Style = Style::default().bg(Color::Rgb(50, 25, 0));

        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(15), Constraint::Percentage(85)].as_ref())
                .split(f.size());

            let village_screen = Paragraph::new("Not done yet")
                .style(main_style)
                .alignment(Alignment::Center)
                .block(Block::default().borders(Borders::ALL).title("Main"));

            f.render_widget(village_screen, chunks[1]);

            f.render_widget(menu_widget.clone(), chunks[0]);
        }).expect("Can render widget");
    }
}
