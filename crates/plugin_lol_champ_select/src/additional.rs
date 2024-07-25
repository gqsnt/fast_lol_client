use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Action {
    pub actor_cell_id: i32,
    pub champion_id: i32,
    pub completed: bool,
    pub id: u64,
    pub is_ally_action: bool,
    pub is_in_progress: bool,
    pub pick_turn: u32,
    #[serde(rename = "type")]
    pub type_: ActionType,

}


#[derive(Serialize, Deserialize, Clone, Debug,PartialEq, Default)]
pub enum ActionType {
    #[serde(rename = "pick")]
    #[default]
    Pick,
    #[serde(rename = "ban")]
    Ban,
}

