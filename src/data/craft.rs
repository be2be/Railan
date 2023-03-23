use std::str::FromStr;

use serde::Deserialize;

use std::fmt::{Display, Formatter};

use super::TerminalDisplay;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Craft{
    pub craft_type: CraftType,
    pub lvl: i32,
}

impl TerminalDisplay for Craft{
    fn display(&self) -> String {
        format!("{}: Level {} \n", self.craft_type, self.lvl)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum CraftType{
    Woodworker,
    Stonemason
}

impl Display for CraftType{

    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {

        match *self{
            CraftType::Woodworker => { write!(f,"Woodworker")}
            CraftType::Stonemason => { write!(f,"Stonemason")}
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseError;

impl FromStr for CraftType {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Woodworker" => Ok(CraftType::Woodworker),
            "Stonemason" => Ok(CraftType::Stonemason),
            _ => Err(ParseError),
        }
    }
}