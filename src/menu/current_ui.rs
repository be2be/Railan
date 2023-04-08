use std::collections::HashMap;
use std::io::Stdout;
use std::ops::Add;
use std::sync::{Arc, RwLock};
use std::time::{Instant,Duration};
use tui::backend::CrosstermBackend;
use tui::Terminal;
use tui::widgets::List;
use crate::game::GameState;

use crate::menu::ui_foundations::{ScrollingDirection, UIType};
use crate::menu::rendering::{
    TerminalRenderer,
    full_screen::FullScreenMessage,
    village_menu::VillageMenu,
    craft_menu::CraftMenu,
    main_menu::MainMenu,
    diplomacy_menu::DiplomacyMenu,
};

/// This struct represents the current user-interface, which has a UIType, which specifies the
/// type of widget, which is displayed, e.g. the widget for displaying the player's villages. This
/// struct is also used to render the terminal.
pub struct CurrentUI{
    /// The current type of user-interface
    pub ui_type: UIType,
    /// A map containing for every UIType the corresponding rendering-trait,
    /// which will draw the whole terminal, that is, main-widget and menu-widget
    renderer_for_ui_type: HashMap<UIType,Box<dyn TerminalRenderer>>,
    /// A possiblle expiratione time, from which on the corresponding UIType is not valid anymore.
    expiration_time: Option<Instant>,
}

impl CurrentUI{

    /// Creates a new CurrentUI struct. Initially it will be of type UIType::Start and contain an
    /// expiration_time of three seconds.
    pub fn new() -> CurrentUI{
        CurrentUI{
            ui_type: UIType::Start,
            renderer_for_ui_type: {
                let mut map : HashMap<UIType,Box<dyn TerminalRenderer>> = HashMap::new();

                // add a rendering trait for every UIType
                map.insert( UIType::Start, Box::new(FullScreenMessage { ui_type: UIType::Start}));
                map.insert( UIType::Quit, Box::new( FullScreenMessage { ui_type: UIType::Quit} ));
                map.insert( UIType::Main, Box::new(MainMenu{}));
                map.insert( UIType::Villages, Box::new(VillageMenu::new()));
                map.insert( UIType::Crafts, Box::new(CraftMenu::new()));
                map.insert( UIType::Diplomacy, Box::new(DiplomacyMenu::new()));

                map
            },
            expiration_time: Some(Instant::now().add(Duration::from_secs(3))),
        }
    }

    /// Returns true, if the struct has an expiration time that lies in the past. Otherwise it will
    /// return false.
    pub fn is_expired(&self) -> bool {
        match self.expiration_time{
            None => false,
            Some(expiration_time) => {

                expiration_time.lt(&Instant::now())
            },
        }
    }

    /// Updates the UIType of the current ui. In case of type UIType::Quit (the user quits the terminal)
    /// it will add an expiration time of three seconds so that the closing message can be displayed
    /// as long.
    /// * `ui_type` is the new ui-type of the current ui
    pub fn change_ui_type(&mut self, ui_type: UIType){
        self.ui_type = ui_type;

        if self.ui_type == UIType::Quit{
            self.expiration_time = Some(Instant::now().add(Duration::from_secs(3)));
        }else{
            self.expiration_time = None;
        }
        
        self.reset_scroll_offset();

    }

    /// Increments or decrements the scrolling offset of the current view, depending on its
    /// scrolling-direction.
    /// # Arguments
    /// `direction` - The scrolling direction, either up or down
    pub fn scroll(&mut self, direction: ScrollingDirection){
        let mut scroll_offset = self.get_current_ui_renderer_mut().get_scroll_offset();

        match direction{
            ScrollingDirection::Up => {
                if scroll_offset > 0{
                    scroll_offset -= 1
                }
            }
            ScrollingDirection::Down => scroll_offset += 1,
        }

        self.get_current_ui_renderer_mut().set_scroll_offset(scroll_offset);
    }

    /// Resets the scroll offset to zero in order to ensure that a menu will be displayed from the
    /// very top onwards
    fn reset_scroll_offset(&mut self) {
        self.get_current_ui_renderer_mut().set_scroll_offset(0);
    }

    /// Draws the current ui to the terminal. The rendering is done by a struct, which implements the
    /// TerminalRenderer-trait. For every UIType, there is exactly one such struct in the
    /// renderer_for_ui_type.
    pub fn render(&mut self, terminal: &mut Terminal<CrosstermBackend<Stdout>>, menu_widget : &List, game_state: Arc<RwLock<GameState>>) {

        self.get_current_ui_renderer_mut().render(terminal,menu_widget,game_state);
    }

    /// A helper method to return the TerminalRenderer-trait for the current view
    fn get_current_ui_renderer_mut(&mut self) -> &mut Box<dyn TerminalRenderer>{
        self.renderer_for_ui_type.get_mut(&self.ui_type).unwrap()
    }
}