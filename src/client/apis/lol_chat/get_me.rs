use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LolChatUserResource {
    pub summoner_id: u64,
    pub id: String,
    pub name: String,
    pub pid: String,
    pub puuid: String,
    pub obfuscated_summoner_id: u64,
    pub game_name: String,
    pub game_tag: String,
    pub icon: i32,
    pub availability: String,
    pub platform_id: String,
    pub patchline: String,
    pub product: String,
    pub product_name: String,
    pub summary: String,
    pub time: u64,
    pub status_message: Option<String>,
    pub last_seen_online_timestamp: Option<String>,
    pub lol: LolDetails,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LolDetails {
    pub champion_id: Option<String>,
    pub ranked_league_queue: Option<String>,
    pub game_status: Option<String>,
    pub ranked_prev_season_tier: Option<String>,
    pub map_id: Option<String>,
    pub icon_override: Option<String>,
    pub ranked_league_tier: Option<String>,
    pub game_queue_type: Option<String>,
    pub profile_icon: Option<String>,
    pub ranked_losses: Option<String>,
    pub regalia: Option<String>,
    pub skin_variant: Option<String>,
    pub puuid: Option<String>,
    pub ranked_league_division: Option<String>,
    pub ranked_prev_season_division: Option<String>,
    pub damage_skin_id: Option<String>,
    pub legendary_mastery_score: Option<String>,
    pub level: Option<String>,
    pub ranked_split_reward_level: Option<String>,
    pub ranked_wins: Option<String>,
    pub skinname: Option<String>,
    pub companion_id: Option<String>,
    pub map_skin_id: Option<String>,
    pub game_id: Option<String>, // Present in the second example but not in the first
    pub banner_id_selected: Option<String>, // Present in the second example but not in the first
    pub player_title_selected: Option<String>, // Present in the second example but not in the first
    pub game_mode: Option<String>, // Present in the second example but not in the first
    pub queue_id: Option<String>, // Present in the second example but not in the first
    pub challenge_crystal_level: Option<String>, // Present in the second example but not in the first
    pub challenge_points: Option<String>, // Present in the second example but not in the first
    pub is_observable: Option<String>, // Present in the second example but not in the first
    pub time_stamp: Option<String>, // Present in the second example but not in the first
    pub challenge_tokens_selected: Option<String>, // Present in the second example but not in the first
    pub pty: Option<String>, // Present in the second example but not in the first
}