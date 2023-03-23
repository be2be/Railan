pub mod saves;
pub mod kingdom;
pub mod craft;
pub mod village;

/// A trait, which is used to print game data to the main window in the terminal
pub trait TerminalDisplay{
    fn display(&self) -> String;
}

