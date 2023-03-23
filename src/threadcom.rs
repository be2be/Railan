use crossbeam::channel::{bounded,Sender,Receiver};
use std::thread::JoinHandle;
use crossterm::event::Event;

/// The ThreadCommunication struct holds some multi-sender-multi-receiver channels.
pub struct ThreadCommunication {
    /// A channel to communicate a "quit"-string, if a user wants to quit the program
    terminate_app_channel: (Sender<String>, Receiver<String>),
    /// A channel to communicate input-key events from a listener to executor
    input_key_event_channel: (Sender<Event>, Receiver<Event>),
    /// A vector with handles over all spawned threads
    handles: Vec<JoinHandle<()>>,
}

impl ThreadCommunication {

    /// Returns a ThreadCommunication containing all necessary communication-channels.
    /// As the implementation is based upon three threads, one for the game-loop, one
    /// for the event-listener and one for the menu-renderer, the channels are set up
    /// as follows:
    /// * `terminate_app_channel` - will be bounded by 2 messages so the menu thread can
    /// inform the other two threads about termination by sending a "quit"-token.
    /// * `input_key_event_channel` - will be bounded by 1. It is used to send
    /// key input events from the event-listener thread to the menu-thread
    pub fn new() -> ThreadCommunication {

        let quit_x = bounded(2);
        let event_x = bounded(1);
        ThreadCommunication {
            terminate_app_channel: quit_x,
            input_key_event_channel: event_x,
            handles: vec![],
        }
    }

    pub fn get_quit_rx(&self) -> Receiver<String>{
        Receiver::clone(&self.terminate_app_channel.1)
    }

    pub fn get_quit_tx(&self) -> Sender<String>{
        Sender::clone(&self.terminate_app_channel.0)
    }

    pub fn get_event_tx(&self) -> Sender<Event>{
        Sender::clone(&self.input_key_event_channel.0)
    }

    pub fn get_event_rx(&self) -> Receiver<Event>{
        Receiver::clone(&self.input_key_event_channel.1)
    }

    /// Adds another handle to the handles-vector
    /// # Arguments
    /// * `handle` the handle, which should be added to the handles-vector
    pub fn add_handle(&mut self, handle: JoinHandle<()>) {
        self.handles.push(handle);
    }

    /// Waits until all threads in the handles-vector have terminated
    pub fn wait_on_shutdown(self) {

        self.handles
            .into_iter()
            .for_each(|h| h.join().unwrap());
    }
}
