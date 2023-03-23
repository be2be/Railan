use crossterm::event::{Event};
use crate::menu::ui_foundations::{UIError, UIType};

/// The UIEvent is an event type, which consists of two different types of events.
/// One of which is an event for key-input, the other a timeout-event. Both cases
/// are mutually exclusive and will rise an error of type MisconfiguredUIEvent in case
/// this mutual exclusivity is not upheld.
#[derive(Hash,Eq,PartialEq,Debug)]
pub struct UIEvent{
    /// A key input event
    input_key_event: Option<Event>,
    /// A boolean flag stating if this event is a timeout-event or not
    /// In case of a timeout event a UIType, which should succeed the current (expired) ui
    timeout_ui: Option<UIType>,
}

/// A builder-struct for the UIEvent.
pub struct UIEventBuilder{
    /// A key input event
    input_key_event: Option<Event>,
    /// In case of a timeout event a UIType, which should succeed the current (expired) ui
    timeout_ui: Option<UIType>,
}

impl UIEventBuilder{

    /// Create a new UIEventBuilder of an empty event.
    pub fn new() -> UIEventBuilder{
        UIEventBuilder{
            input_key_event: None,
            timeout_ui: None,
        }
    }

    /// Mark the event as an input-key event
    /// # Arguments
    /// * `event` the kind of crossterm event
    pub fn input_key_event(&mut self, event: Event) -> &mut UIEventBuilder{
        self.input_key_event = Some(event);
        self.timeout_ui = None;
        self
    }

    /// Add the timeout ui
    /// # Arguments
    /// * `ui_type` the type of the succeeding ui
    pub fn timeout_ui(&mut self, ui_type: UIType) -> &mut UIEventBuilder{
        self.timeout_ui = Some(ui_type);
        self
    }

    /// Builds the corresponding event. Returns a MisconfiguredUIEvent in case of a misconfiguration
    pub fn build(&mut self) -> Result<UIEvent, UIError>{

        if self.timeout_ui.is_some() && self.input_key_event.is_some(){
            return Err(UIError::MisconfiguredUIEvent("Cannot be both, a timeout-event and an input key event."));
        }

        Ok({
            UIEvent{
                input_key_event:
                match &self.input_key_event{
                    None => None,
                    Some(input_key_event) => Some(Clone::clone(input_key_event))
                },
                timeout_ui: {
                    match self.timeout_ui{
                        None => None,
                        Some(timeout_ui) => Some(timeout_ui)
                    }
                }
            }
        })
    }
}