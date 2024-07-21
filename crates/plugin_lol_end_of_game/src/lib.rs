
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use serde_json::{json, Value, to_value};
use reqwest::Method;
use common::IsApiRequest;

mod additional;

// ENDPOINTS

pub struct GetLolEndOfGameV1ChampionMasteryUpdates {}

impl IsApiRequest for GetLolEndOfGameV1ChampionMasteryUpdates {
    const METHOD: Method = Method::GET;
    type ReturnType = LolEndOfGameChampionMasteryUpdate;
    fn get_url(&self) -> String {"/lol-end-of-game/v1/champion-mastery-updates".to_string()}
}

pub fn get_lol_end_of_game_v_1_champion_mastery_updates() -> GetLolEndOfGameV1ChampionMasteryUpdates {
    GetLolEndOfGameV1ChampionMasteryUpdates{}
}


pub struct GetLolEndOfGameV1EogStatsBlock {}

impl IsApiRequest for GetLolEndOfGameV1EogStatsBlock {
    const METHOD: Method = Method::GET;
    type ReturnType = LolEndOfGameEndOfGameStats;
    fn get_url(&self) -> String {"/lol-end-of-game/v1/eog-stats-block".to_string()}
}

pub fn get_lol_end_of_game_v_1_eog_stats_block() -> GetLolEndOfGameV1EogStatsBlock {
    GetLolEndOfGameV1EogStatsBlock{}
}


pub struct GetLolEndOfGameV1GameclientEogStatsBlock {}

impl IsApiRequest for GetLolEndOfGameV1GameclientEogStatsBlock {
    const METHOD: Method = Method::GET;
    type ReturnType = LolEndOfGameGameClientEndOfGameStats;
    fn get_url(&self) -> String {"/lol-end-of-game/v1/gameclient-eog-stats-block".to_string()}
}

pub fn get_lol_end_of_game_v_1_gameclient_eog_stats_block() -> GetLolEndOfGameV1GameclientEogStatsBlock {
    GetLolEndOfGameV1GameclientEogStatsBlock{}
}


pub struct GetLolEndOfGameV1TftEogStats {}

impl IsApiRequest for GetLolEndOfGameV1TftEogStats {
    const METHOD: Method = Method::GET;
    type ReturnType = LolEndOfGameTftEndOfGameViewModel;
    fn get_url(&self) -> String {"/lol-end-of-game/v1/tft-eog-stats".to_string()}
}

pub fn get_lol_end_of_game_v_1_tft_eog_stats() -> GetLolEndOfGameV1TftEogStats {
    GetLolEndOfGameV1TftEogStats{}
}


pub struct PostLolEndOfGameV1GameclientEogStatsBlock {
    pub body: LolEndOfGameGameClientEndOfGameStats,
}

impl IsApiRequest for PostLolEndOfGameV1GameclientEogStatsBlock {
    const METHOD: Method = Method::POST;
    type ReturnType = HashMap<String, String>;
    fn get_url(&self) -> String {"/lol-end-of-game/v1/gameclient-eog-stats-block".to_string()}
    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }
}

pub fn post_lol_end_of_game_v_1_gameclient_eog_stats_block(body: LolEndOfGameGameClientEndOfGameStats) -> PostLolEndOfGameV1GameclientEogStatsBlock {
    PostLolEndOfGameV1GameclientEogStatsBlock{body}
}


pub struct PostLolEndOfGameV1StateDismissStats {}

impl IsApiRequest for PostLolEndOfGameV1StateDismissStats {
    const METHOD: Method = Method::POST;
    type ReturnType = HashMap<String, String>;
    fn get_url(&self) -> String {"/lol-end-of-game/v1/state/dismiss-stats".to_string()}
}

pub fn post_lol_end_of_game_v_1_state_dismiss_stats() -> PostLolEndOfGameV1StateDismissStats {
    PostLolEndOfGameV1StateDismissStats{}
}


// OBJECTS

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolEndOfGameChampionMasteryGrade {
    pub puuid: String,
    pub champion_id: i32,
    pub grade: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolEndOfGameChampionMasteryMini {
    pub puuid: String,
    pub champion_id: i32,
    pub champion_level: i64,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolEndOfGameChampionMasteryUpdate {
    pub id: String,
    pub game_id: u64,
    pub puuid: String,
    pub champion_id: i32,
    pub has_leveled_up: bool,
    pub level: i64,
    pub points_before_game: i64,
    pub points_gained: i64,
    pub points_gained_individual_contribution: i64,
    pub bonus_points_gained: i64,
    pub points_since_last_level_before_game: i64,
    pub points_until_next_level_before_game: i64,
    pub points_until_next_level_after_game: i64,
    pub tokens_earned: i64,
    pub token_earned_after_game: bool,
    pub grade: String,
    pub score: i64,
    pub level_up_list: Vec<LolEndOfGameChampionMasteryMini>,
    pub member_grades: Vec<LolEndOfGameChampionMasteryGrade>,
    pub mvp_puuid: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolEndOfGameEndOfGamePlayer {
    pub stats: HashMap<String, String>,
    pub items: Vec<i32>,
    pub puuid: String,
    pub bot_player: bool,
    pub champion_id: i32,
    pub game_id: u64,
    pub leaver: bool,
    pub leaves: i32,
    pub level: i32,
    pub losses: i32,
    pub profile_icon_id: i32,
    pub spell_1_id: i32,
    pub spell_2_id: i32,
    pub summoner_name: String,
    pub team_id: i32,
    pub wins: i32,
    pub summoner_id: u64,
    pub selected_position: String,
    pub detected_team_position: String,
    pub skin_splash_path: String,
    pub skin_tile_path: String,
    pub skin_emblem_paths: Vec<String>,
    pub champion_name: String,
    pub champion_square_portrait_path: String,
    pub is_local_player: bool,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolEndOfGameEndOfGamePoints {
    pub point_change_from_champions_owned: i32,
    pub point_change_from_gameplay: i32,
    pub points_used: i32,
    pub previous_points: i32,
    pub points_until_next_reroll: i32,
    pub reroll_count: i32,
    pub total_points: i32,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolEndOfGameEndOfGameStats {
    pub difficulty: String,
    pub game_id: u64,
    pub game_length: i32,
    pub game_mode: String,
    pub game_mutators: Vec<String>,
    pub game_type: String,
    pub invalid: bool,
    pub queue_type: String,
    pub ranked: bool,
    pub report_game_id: u64,
    pub multi_user_chat_id: String,
    pub multi_user_chat_password: String,
    pub muc_jwt_dto: LolEndOfGameMucJwtDto,
    pub teams: Vec<LolEndOfGameEndOfGameTeam>,
    pub local_player: LolEndOfGameEndOfGamePlayer,
    pub my_team_status: String,
    pub leveled_up: bool,
    pub new_spells: Vec<i32>,
    pub previous_level: u64,
    pub rp_earned: i32,
    pub base_points: i32,
    pub battle_boost_ip_earned: i32,
    pub boost_ip_earned: i32,
    pub first_win_bonus: i32,
    pub ip_earned: i32,
    pub ip_total: i32,
    pub boost_xp_earned: i32,
    pub experience_earned: i32,
    pub experience_total: i32,
    pub global_boost_xp_earned: i32,
    pub loyalty_boost_xp_earned: i32,
    pub xbgp_boost_xp_earned: i32,
    pub missions_xp_earned: i32,
    pub previous_xp_total: u64,
    pub next_level_xp: u64,
    pub current_level: u64,
    pub pre_level_up_experience_total: u64,
    pub pre_level_up_next_level_xp: u64,
    pub time_until_next_first_win_bonus: i32,
    pub caused_early_surrender: bool,
    pub early_surrender_accomplice: bool,
    pub team_early_surrendered: bool,
    pub game_ended_in_early_surrender: bool,
    pub reroll_data: LolEndOfGameEndOfGamePoints,
    pub team_boost: Option<LolEndOfGameEndOfGameTeamBoost>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolEndOfGameEndOfGameTeam {
    pub stats: HashMap<String, String>,
    pub players: Vec<LolEndOfGameEndOfGamePlayer>,
    pub member_status_string: String,
    pub name: String,
    pub tag: String,
    pub full_id: String,
    pub team_id: i32,
    pub is_bottom_team: bool,
    pub is_player_team: bool,
    pub is_winning_team: bool,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolEndOfGameEndOfGameTeamBoost {
    pub summoner_name: String,
    pub skin_unlock_mode: String,
    pub price: i64,
    pub ip_reward: i64,
    pub ip_reward_for_purchaser: i64,
    pub available_skins: Vec<i64>,
    pub unlocked: bool,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolEndOfGameGameClientEndOfGameStats {
    pub game_id: u64,
    pub game_mode: String,
    pub stats_block: HashMap<String, String>,
    pub queue_id: i32,
    pub queue_type: String,
    pub is_ranked: bool,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolEndOfGameMucJwtDto {
    pub jwt: String,
    pub channel_claim: String,
    pub domain: String,
    pub target_region: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolEndOfGameTftEndOfGameCompanionViewModel {
    pub species_name: String,
    pub color_name: String,
    pub icon: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolEndOfGameTftEndOfGameCustomAugmentContainerViewModel {
    pub name_id: String,
    pub icon_path: String,
    pub display_name: String,
    pub description: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolEndOfGameTftEndOfGameItemViewModel {
    pub name: String,
    pub icon: String,
    pub id: i32,
    pub name_id: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolEndOfGameTftEndOfGamePieceViewModel {
    pub name: String,
    pub icon: String,
    pub level: u32,
    pub price: u32,
    pub items: Vec<LolEndOfGameTftEndOfGameItemViewModel>,
    pub traits: Vec<LolEndOfGameTftEndOfGameTraitViewModel>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolEndOfGameTftEndOfGamePlaybookViewModel {
    pub item_id: u32,
    pub name: String,
    pub icon_small: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolEndOfGameTftEndOfGamePlayerViewModel {
    pub summoner_id: u64,
    pub summoner_name: String,
    pub riot_id_game_name: String,
    pub riot_id_tag_line: String,
    pub icon_id: i32,
    pub puuid: String,
    pub ffa_standing: u8,
    pub health: u8,
    pub rank: u8,
    pub is_local_player: bool,
    pub is_interactable: bool,
    pub partner_group_id: u8,
    pub board_pieces: Vec<LolEndOfGameTftEndOfGamePieceViewModel>,
    pub augments: Vec<LolEndOfGameTftEndOfGameItemViewModel>,
    pub companion: LolEndOfGameTftEndOfGameCompanionViewModel,
    pub playbook: LolEndOfGameTftEndOfGamePlaybookViewModel,
    pub custom_augment_container: LolEndOfGameTftEndOfGameCustomAugmentContainerViewModel,
    pub set_core_name: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolEndOfGameTftEndOfGameTraitViewModel {
    pub id: String,
    pub name: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolEndOfGameTftEndOfGameViewModel {
    pub players: Vec<LolEndOfGameTftEndOfGamePlayerViewModel>,
    pub local_player: Option<LolEndOfGameTftEndOfGamePlayerViewModel>,
    pub game_length: u32,
    pub game_id: u64,
    pub queue_id: i32,
    pub queue_type: String,
    pub is_ranked: bool,
}


// ENUMS

