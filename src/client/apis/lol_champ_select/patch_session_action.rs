use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolChampSelectPatchSessionActionBody {
    pub champion_id: u32,
    pub completed: bool,
}



