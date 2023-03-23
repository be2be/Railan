use serde::Deserialize;

use super::TerminalDisplay;

/// A village is a struct representing a village in the kingdom
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Village{
    /// The name of the village
    pub name: String,
    /// The number of citizens in the village
    pub num_citizen: i32,
}

impl TerminalDisplay for Village{
    fn display(&self) -> String {
        format!("{}: {} Citizen\n", self.name, self.num_citizen)
    }
}