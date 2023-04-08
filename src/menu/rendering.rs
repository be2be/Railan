pub mod diplomacy_menu;
pub mod full_screen;
pub mod craft_menu;
pub mod main_menu;
pub mod village_menu;

use tui::{
    backend::CrosstermBackend,
    widgets::List,
    Terminal,
};
use std::io::Stdout;
use std::sync::{Arc,RwLock};
use crate::game::GameState;

/// A trait, which is used to render views in the terminal. For every type of view there will be
/// a corresponding struct, which implements this trait.
pub trait TerminalRenderer{

    /// Renders a view to the terminal.
    /// # Arguments
    /// * `terminal` is the terminal in which the rendering shall be done
    /// * `menu_widget` is the crossterm-menu widget, which must be included in the rendering
    /// * `game_state` contains the player's game-data, which will in some way be used in the rendering
    /// for most widgets.
    fn render(&mut self, terminal: &mut Terminal<CrosstermBackend<Stdout>>, menu_widget : &List, game_state: Arc<RwLock<GameState>>);

    /// Every struct, which implements the TerminalRenderer trait has to implement the following two
    /// methods, which allow every menu to implement scrolling functionality
    /// # Arguments
    /// * `offset` The scrolling offset
    fn set_scroll_offset(&mut self, offset: u16);

    /// Similar to the setter, this method will return some scrolling offset. In case the menu does
    /// not allow any scrolling, it will be 0.
    fn get_scroll_offset(&self) -> u16;
}