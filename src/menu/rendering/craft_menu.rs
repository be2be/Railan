use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Alignment},
    style::{Color, Style},
    Terminal,
};

use std::io::Stdout;
use std::sync::{Arc,RwLock};
use tui::widgets::{Block, Borders, List, Paragraph};
use crate::data::TerminalDisplay;
use crate::game::GameState;
use crate::menu::rendering::TerminalRenderer;

/// A view, which displays a player's crafts
pub  struct CraftMenu;

impl TerminalRenderer for CraftMenu{

    /// Renders the CraftMenu in the terminal
    fn render(&mut self, terminal: &mut Terminal<CrosstermBackend<Stdout>>, menu_widget : &List, game_state: Arc<RwLock<GameState>>) {

        let main_style : Style = Style::default().bg(Color::Rgb(50, 25, 0));

        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(15), Constraint::Percentage(85)].as_ref())
                .split(f.size());

            let crafts: String = game_state.read().unwrap().kingdom.crafts.iter()
                .map(|(t,v)| {
                    let formatted_crafts : String = v.iter()
                        .map(|c| c.display())
                        .collect();
                    format!( "{}:\n {formatted_crafts}", t)
                })
                .collect();

            let craft_screen = Paragraph::new(crafts)
                .style(main_style)
                .alignment(Alignment::Center)
                .block(Block::default().borders(Borders::ALL).title("Main"));

            f.render_widget(craft_screen, chunks[1]);

            f.render_widget(menu_widget.clone(), chunks[0]);
        }).expect("Can render widget");
    }
}
