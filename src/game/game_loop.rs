use std::thread;
use std::time::Duration;
use std::sync::{Arc, RwLock};
use crossbeam::channel::{Receiver};
use super::GameState;

pub fn game_loop(game_state: Arc<RwLock<GameState>>, quit_rx: Receiver<String>){

    loop{

        thread::sleep(Duration::new (1,0));

        if let Ok(msg) = quit_rx.try_recv() {
            if msg.eq("quit"){
                break;
            }
        }

        let mut write_guard = game_state.write().unwrap();
        write_guard.kingdom.add_citizen(1);
    }
}