pub mod event_listener;

mod events;
mod user_interface;
mod current_ui;
mod ui_foundations;
mod assets;
mod rendering;

use crossterm::event::Event;
use std::sync::{Arc,RwLock};
use std::time::Duration;
use crossbeam::channel::{Sender,Receiver};

use crate::menu::events::UIEventBuilder;
use crate::menu::user_interface::UserInterface;
use crate::game::GameState;
use crate::menu::ui_foundations::UIType;

/// Renders a terminal in the console. It will continuously render the terminal until the application
/// terminates. If the user/player wants to quit the app, this method will recognize the corresponding
/// UIType::Terminated, and send two quit-tokens to the two other running game threads, essentially
/// commanding them to shut down.
/// Any key-event, which is found by a concurrently running thread, will be sent to this thread and
/// processed within a loop-cycle.
/// # Arguments
/// * `game_state` the game data, which manageable in the console
/// * `quit_app_tx` the communication channel for sending quit-tokens
/// * `event_rx`the communication channel for receiving input events
pub fn render_terminal(game_state: Arc<RwLock<GameState>>, quit_app_tx: Sender<String>, event_rx: Receiver<Event>){

    let mut ui = UserInterface::new(game_state);

    loop {

        // Check, if the terminal must be updated and update its state if necessary
        ui.update_terminal();

        // (Re)render the terminal
        ui.render_terminal();

        // In case the user wants to quit, inform the other concurrent threads and terminate yourself.
        if ui.get_current_uitype() == UIType::Terminated{

            quit_app_tx.send(String::from("quit")).expect("Should send quit");
            quit_app_tx.send(String::from("quit")).expect("Should send quit");
            break;
        }

        // Listen for key input events for 200 ms each turn. Process any received event.
        if let Ok(event) = event_rx.recv_timeout(Duration::new(0,200)) {

            let event = UIEventBuilder::new().input_key_event(event).build();

            match event{
                Ok(event) => ui.process_ui_event(&event),
                Err(_) => { /* For now, do nothing. In the future, print errors to the UI in some overlay, if possible. */}
            }

        }
    }
}
