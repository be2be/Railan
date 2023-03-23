use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Layout, Alignment},
    style::{Color, Style},
    widgets::{Block, Borders, List, Paragraph},
    Terminal,
};

use std::io::Stdout;
use std::sync::{Arc,RwLock};
use std::thread;
use std::time::Duration;
use crate::game::GameState;
use crate::menu::ui_foundations::UIType;
use crate::menu::assets;
use crate::menu::rendering::TerminalRenderer;

/// A view, which displays a fullscreen message
pub struct FullScreenMessage {
    /// A view type, e.g. in order to differentiate between the startup- and termination message
    pub ui_type: UIType,
}

impl TerminalRenderer for FullScreenMessage {

    /// Draws a message over the full screen of the terminal.
    fn render(&mut self, terminal: &mut Terminal<CrosstermBackend<Stdout>>, _menu_widget : &List, _game_state: Arc<RwLock<GameState>>) {

        let mut msg = None;

        match self.ui_type{
            UIType::Start => {
                msg = Some(assets::read_asset("welcome.txt"));
            }
            UIType::Quit => {
                msg = Some(assets::read_asset("exit.txt"));
            }
            _ => {}
        }

        if let Some(msg) = msg {

            let main_style : Style = Style::default().bg(Color::Rgb(50, 25, 0));

            let widget = Paragraph::new(msg)
                .style(main_style)
                .alignment(Alignment::Center)
                .block(Block::default().borders(Borders::ALL).title("Railan"));

            terminal.draw(move |f| {
                let layout = Layout::default()
                    .constraints([Constraint::Percentage(100)].as_ref())
                    .split(f.size());

                f.render_widget(widget, layout[0]);
            }).expect("Can render widget");

            thread::sleep(Duration::from_secs(3));
        }
    }
}