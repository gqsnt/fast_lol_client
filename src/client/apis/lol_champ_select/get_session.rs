
use serde::{Serialize, Deserialize};

#[derive(Default, Clone, Serialize, Deserialize, Debug)]
pub struct LolChampSelectChampSelectSession {
    pub game_id: u64,
    pub timer: LolChampSelectChampSelectTimer,
    pub chat_details: LolChampSelectChampSelectChatRoomDetails,
    pub my_team: Vec<LolChampSelectChampSelectPlayerSelection>,
    pub their_team: Vec<LolChampSelectChampSelectPlayerSelection>,
    pub trades: Vec<LolChampSelectChampSelectTradeContract>,
    pub pick_order_swaps: Vec<LolChampSelectChampSelectSwapContract>,
    pub actions: Vec<LolChampSelectAction>,
    pub bans: LolChampSelectChampSelectBannedChampions,
    pub local_player_cell_id: i64,
    pub is_spectating: bool,
    pub allow_skin_selection: bool,
    pub allow_duplicate_picks: bool,
    pub allow_battle_boost: bool,
    pub boostable_skin_count: i32,
    pub allow_rerolling: bool,
    pub rerolls_remaining: u64,
    pub allow_locked_events: bool,
    pub locked_event_index: i32,
    pub bench_enabled: bool,
    pub bench_champions: Vec<LolChampSelectBenchChampion>,
    pub counter: i64,
    pub recovery_counter: i64,
    pub skip_champion_select: bool,
    pub has_simultaneous_bans: bool,
    pub has_simultaneous_picks: bool,
    pub is_custom_game: bool,
}

#[derive(Default, Clone, Serialize, Deserialize, Debug)]
pub struct LolChampSelectChampSelectTimer {
    pub adjusted_time_left_in_phase: i64,
    pub total_time_in_phase: i64,
    pub phase: String,
    pub is_infinite: bool,
    pub internal_now_in_epoch_ms: u64,
}

#[derive(Default, Clone, Serialize, Deserialize, Debug)]
pub struct LolChampSelectChampSelectChatRoomDetails {
    pub multi_user_chat_id: String,
    pub multi_user_chat_password: String,
    pub muc_jwt_dto: LolChampSelectMucJwtDto,
}

#[derive(Default, Clone, Serialize, Deserialize, Debug)]
pub struct LolChampSelectMucJwtDto {
    pub jwt: String,
    pub channel_claim: String,
    pub domain: String,
    pub target_region: String,
}

#[derive(Default, Clone, Serialize, Deserialize, Debug)]
pub struct LolChampSelectChampSelectPlayerSelection {
    pub cell_id: i64,
    pub champion_id: i32,
    pub selected_skin_id: i32,
    pub ward_skin_id: i64,
    pub spell1_id: u64,
    pub spell2_id: u64,
    pub team: i32,
    pub assigned_position: String,
    pub champion_pick_intent: i32,
    pub summoner_id: u64,
    pub puuid: String,
    pub name_visibility_type: String,
    pub obfuscated_summoner_id: u64,
    pub obfuscated_puuid: String,
}

#[derive(Default, Clone, Serialize, Deserialize, Debug)]
pub struct LolChampSelectChampSelectTradeContract {
    pub id: i64,
    pub cell_id: i64,
    pub state: LolChampSelectChampSelectTradeState,
}

#[derive(Default, Clone, Serialize, Deserialize, Debug)]
pub enum LolChampSelectChampSelectTradeState {
    Accepted,
    Cancelled,
    Declined,
    Sent,
    Received,
    Invalid,
    Busy,
    Available,
    #[default]
    Undefined,
}

#[derive(Default, Clone, Serialize, Deserialize, Debug)]
pub struct LolChampSelectChampSelectSwapContract {
    pub id: i64,
    pub cell_id: i64,
    pub state: LolChampSelectChampSelectSwapState,
}

#[derive(Default, Clone, Serialize, Deserialize, Debug)]
pub enum LolChampSelectChampSelectSwapState {
    Accepted,
    Cancelled,
    Declined,
    Sent,
    Received,
    Invalid,
    Busy,
    Available,
    #[default]
    Undefined,
}

#[derive(Default, Clone, Serialize, Deserialize, Debug)]
pub struct LolChampSelectChampSelectBannedChampions {
    pub my_team_bans: Vec<i32>,
    pub their_team_bans: Vec<i32>,
    pub num_bans: i32,
}

#[derive(Default, Clone, Serialize, Deserialize, Debug)]
pub struct LolChampSelectBenchChampion {
    pub champion_id: i32,
    pub is_priority: bool,
}


#[derive(Default, Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolChampSelectAction {
    pub actor_cell_id: i64,
    pub champion_id: i32,
    pub completed: bool,
    pub id: i64,
    pub is_ally_action: bool,
    pub is_in_progress: bool,
    #[serde(rename = "type")]
    pub action_type: String, // type is a reserved keyword in Rust, so using type_
}