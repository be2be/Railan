use std::collections::HashMap;

use serde::Deserialize;
use serde_with::{serde_as};

use super::craft::{Craft, CraftType};
use super::village::Village;

/// A Kingdom is the main struct of the game. A player will access and manipulate its data.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[serde_as]
pub struct Kingdom {
    /// A set of villages, which make up the kingdom
    pub villages: Vec<Village>,
    /// A set of crafts, which are practices in the kingdom
    #[serde_as(as = "Vec<(DisplayFromStr, _)>")]
    pub crafts: HashMap<CraftType, Vec<Craft>>,
}

impl Kingdom {
    /// A first PoC- function so that the game loop has something to do :)
    pub fn add_citizen(&mut self, citizen: i32) {
        self.villages.get_mut(0).unwrap().num_citizen += citizen;
    }
}
