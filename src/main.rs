mod menu;
mod data;
mod game;
mod threadcom;

use game::GameState;
use threadcom::ThreadCommunication;

use std::thread;
use std::sync::{Arc, RwLock};

/// Opens a terminal application game, which runs three separate threads:
/// 1) A thread running the game-loop
/// 2) A thread running an listener for key inputs
/// 3) A thread running the terminal renderer
/// Upon a player quitting the terminal, every thread receives a "quit"-token and quits graciously.
///
/// In order to facilitate the communication between threads, a ThreadCommunication-struct is used.
/// Please have a look at its documentation.
fn main(){

    // Load the provided save-file
    let kingdom = data::saves::load_save_file("k_best_kingdom.json");
    let game_state = Arc::new(RwLock::new(GameState::new(kingdom)));

    let mut thread_communication = ThreadCommunication::new();

    thread_communication.add_handle({

        let quit_rc = thread_communication.get_quit_rx();
        let game_state = Arc::clone(&game_state);

        thread::spawn(move|| game::game_loop::game_loop(game_state, quit_rc) )
    });

    // Start a thread, which listens for input key events
    thread_communication.add_handle({

        let quit_rc = thread_communication.get_quit_rx();
        let event_tx = thread_communication.get_event_tx();

        thread::spawn(move|| menu::event_listener::listen_for_input_events(quit_rc, event_tx)
        )
    });

    // Start a thread, which renders the terminal-ui
    thread_communication.add_handle({

        let quit_tc = thread_communication.get_quit_tx();
        let event_rx = thread_communication.get_event_rx();
        let game_state = Arc::clone(&game_state);

        thread::spawn(move|| menu::render_terminal(game_state,quit_tc, event_rx))
    });

    // Wait for all threads to shut down in an orderly fashion
    thread_communication.wait_on_shutdown();
}
