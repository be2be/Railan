use std::sync::{Arc,RwLock};
use std::cell::{RefCell};
use std::rc::Rc;
use std::io::Stdout;
use std::collections::HashMap;
use std::ops::DerefMut;

use tui::{
    backend::CrosstermBackend,
    style::{Color, Style},
    widgets::{Block, Borders, List, ListItem},
    Terminal,
};

use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers, KeyEventState, KeyEventKind};

use crate::menu::ui_foundations::*;
use crate::menu::events::{UIEventBuilder,UIEvent};
use crate::menu::assets;
use crate::game::GameState;
use crate::menu::ui_foundations::ActionType::{ChangeView, HandleUITimeout, Scroll};
use crate::menu::current_ui::CurrentUI;

/// Holds the current user-interface the common terminal as well as some general data structures, which
/// map specified events to specified actions, e.g. a key input event 'v' will be mapped to an action
/// to change the menu to the 'village' screen. It also holds the GameState in as a read-write-lock
/// to ensure safe concurrent access.
pub struct UserInterface{
    /// The current user-interface, containing the UIType and a terminal-renderer
    cur_ui: CurrentUI,
    /// A reference to the common terminal, which will be used by every terminal-renderer of
    /// the current user-interface
    terminal: Rc<RefCell<Terminal<CrosstermBackend<Stdout>>>>,
    /// The general GameState, which is secured behind a Read-Write Lock.
    game_state: Arc<RwLock<GameState>>,
    /// Some mapping from ui-events to actions
    event_to_action_type: HashMap<UIEvent,ActionType>
}

impl UserInterface {

    /// Creates a new UserInterface, which makes use of the passed GameState. This method will create
    /// a new crossterm-terminal and initialise the event-to-action map, which will be used by the
    /// user-interface.
    /// # Arguments
    /// * `game_state` is the common game-state, which is accessible in the terminal.
    pub fn new(game_state: Arc<RwLock<GameState>>) -> UserInterface{
        UserInterface{
            terminal: create_crossterm_terminal(),
            game_state,
            cur_ui: CurrentUI::new(),
            event_to_action_type: {

                let village_menu_event = UIEventBuilder::new().input_key_event(Event::Key(KeyEvent {
                    code: KeyCode::Char('v'),
                    modifiers: KeyModifiers::NONE,
                    kind: KeyEventKind::Press,
                    state: KeyEventState::NONE,
                })).build().unwrap();

                let craft_menu_event = UIEventBuilder::new().input_key_event(Event::Key(KeyEvent {
                    code: KeyCode::Char('c'),
                    modifiers: KeyModifiers::NONE,
                    kind: KeyEventKind::Press,
                    state: KeyEventState::NONE,
                })).build().unwrap();

                let diplomacy_menu_event = UIEventBuilder::new().input_key_event(Event::Key(KeyEvent {
                    code: KeyCode::Char('d'),
                    modifiers: KeyModifiers::NONE,
                    kind: KeyEventKind::Press,
                    state: KeyEventState::NONE,
                })).build().unwrap();

                let quit_event = UIEventBuilder::new().input_key_event(Event::Key(KeyEvent {
                    code: KeyCode::Char('q'),
                    modifiers: KeyModifiers::NONE,
                    kind: KeyEventKind::Press,
                    state: KeyEventState::NONE,
                })).build().unwrap();

                let timeout_start = UIEventBuilder::new()
                    .timeout_ui(UIType::Start)
                    .build()
                    .unwrap();

                let timeout_quit = UIEventBuilder::new()
                    .timeout_ui(UIType::Quit)
                    .build()
                    .unwrap();

                let scroll_down = UIEventBuilder::new().input_key_event(Event::Key(KeyEvent {
                    code: KeyCode::Down,
                    modifiers: KeyModifiers::NONE,
                    kind: KeyEventKind::Press,
                    state: KeyEventState::NONE,
                })).build().unwrap();

                let scroll_up = UIEventBuilder::new().input_key_event(Event::Key(KeyEvent {
                    code: KeyCode::Up,
                    modifiers: KeyModifiers::NONE,
                    kind: KeyEventKind::Press,
                    state: KeyEventState::NONE,
                })).build().unwrap();

                let mut event_to_action_type  = HashMap::new();
                event_to_action_type.insert(village_menu_event, ChangeView(UIType::Villages));
                event_to_action_type.insert(craft_menu_event, ChangeView(UIType::Crafts));
                event_to_action_type.insert(diplomacy_menu_event, ChangeView(UIType::Diplomacy));
                event_to_action_type.insert(quit_event, ChangeView(UIType::Quit));
                event_to_action_type.insert(timeout_start, HandleUITimeout(UIType::Start));
                event_to_action_type.insert(timeout_quit, HandleUITimeout(UIType::Quit));
                event_to_action_type.insert(scroll_down, Scroll(ScrollingDirection::Down));
                event_to_action_type.insert(scroll_up, Scroll(ScrollingDirection::Up));

                event_to_action_type
            }
        }
    }

    /// For a given ui-event the corresponding action will be returned. If no action is found, the
    /// method will return Option::None.
    /// # Arguments
    /// * `event` - is the UIEvent, for which an action will be looked at.
    fn get_action_for_event(&self, event: &UIEvent) -> Option<&ActionType> {
        self.event_to_action_type.get(event)
    }

    /// The method will return true, if the current user-interface has an expiration time, which
    /// lies in the past. If no expiration time was set, it will return false.
    fn is_current_ui_type_expired(&self) -> bool{
        self.cur_ui.is_expired()
    }

    /// Returns the current UIType of the current user-interface.
    pub fn get_current_uitype(&self) -> UIType{
        self.cur_ui.ui_type
    }

    /// Logically updates the terminal, if the need arises. For now it will only check, if the current
    /// ui has expired and trigger a timeout-event in that case.
    pub fn update_terminal(&mut self){

        if self.is_current_ui_type_expired() {

            let timeout_event = UIEventBuilder::new()
                .timeout_ui(self.get_current_uitype())
                .build()
                .unwrap();

            self.process_ui_event(&timeout_event);
        }
    }

    /// Renders/Redraws the terminal. The rendering will be handled by the RenderTerminal-trait, which
    /// is implemented for every menu, e.g. there is a renderer for the village-view, another one for
    /// the craft-view, etc.
    pub fn render_terminal(&mut self){

        let menu_items = assets::read_asset("menu.txt");

        let menu_items : Vec<ListItem>= menu_items.lines().map(|s| ListItem::new(s)).collect();

        // A style with a brown background color
        let menu_style : Style = Style::default().bg(Color::Rgb(139, 69, 19));

        let menu_widget = List::new(menu_items)
            .style(menu_style)
            .block(Block::default().borders(Borders::ALL).title("Menu"));

        self.cur_ui.render(self.terminal.borrow_mut().deref_mut(),&menu_widget, Arc::clone(&self.game_state));
    }

    /// Processes ui-events, by determining the corresponding action for that event, which is either
    /// a ChangeView to change the view, e.g. from a village-view to a crafts-view, or handling of a
    /// timeout-event. As the latter only occurs for the initial startup-screen and the final termination
    /// screen, only those two specific cases must be addressed.
    /// # Arguments
    /// * `event` is the UIEvent, which must be handled. It is either a timeout-event or a key-input event.
    pub fn process_ui_event(&mut self, event: &UIEvent) {

        if let Some(action_for_event) = self.get_action_for_event(&event){

            match action_for_event{

                Scroll(scroll_direction) => {
                    self.cur_ui.scroll(*scroll_direction);
                }
                ChangeView(ui_type) => {

                    if self.get_current_uitype() == *ui_type {
                        self.cur_ui.change_ui_type(UIType::Main)
                    } else{
                        self.cur_ui.change_ui_type(*ui_type);
                    }
                }
                HandleUITimeout(ui_type) => {

                    match ui_type {
                        UIType::Start => {
                            self.cur_ui.change_ui_type(UIType::Main)
                        },
                        UIType::Quit => {
                            self.cur_ui.change_ui_type(UIType::Terminated)
                        },
                        _ => {}
                    }
                }
            }
        }

    }
}
