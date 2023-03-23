use crossbeam::channel::{Sender,Receiver};
use crossterm::event::{Event, poll, read};
use std::time::Duration;

/// Listens for input key events, polling every 100 ms. Events are communicated to other
/// threads via crossbeam multi-sender-multi-receiver channel.
/// # Arguments
/// * `quit_rx` a receiver for the channel, which will receive a quit-token
/// * `event_tx` a transmitter for the channel, which will communication input events
pub fn listen_for_input_events(quit_rx: Receiver<String>, event_tx: Sender<Event>) -> () {

    loop{

        if let Ok(msg) = quit_rx.try_recv() {
            if msg.eq("quit"){
                break;
            }
        }

        if let Ok(true) = poll(Duration::from_millis(100)){
            match read() {
                Ok(event) => event_tx.send(event).expect("can send events"),
                Err(e) => println!("Error: {:?}\r", e),
            }
        }
    }
}