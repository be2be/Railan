use std::cell::{RefCell};
use std::rc::Rc;
use std::io::Stdout;
use tui::{
    backend::CrosstermBackend,
    Terminal,
};
use std::io;

/// UIType contains a set of view-types, which can be displayed in the terminal
#[derive(Debug,Eq,PartialEq,Copy, Clone,Hash)]
pub enum UIType{
    Start,
    Main,
    Villages,
    Crafts,
    Diplomacy,
    Quit,
    Terminated,
}

/// ActionType contains a set of different actions, which can be executed in a ui
pub enum ActionType{
    /// An action to change the current view, wherein the argument refers to the new view type
    ChangeView(UIType),
    /// An action to handle a timeout event for a view, wherein the argument refers to the new view type
    HandleUITimeout(UIType),
}

/// Creates a crossterm terminal
pub fn create_crossterm_terminal() -> Rc<RefCell<Terminal<CrosstermBackend<Stdout>>>>{
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    Rc::new(RefCell::new(Terminal::new(backend).unwrap()))
}

#[derive(Hash,Debug)]
pub enum UIError{
    MisconfiguredUIEvent(&'static str),
}