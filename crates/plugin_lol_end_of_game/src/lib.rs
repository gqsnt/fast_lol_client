
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use serde_json::{json, Value, to_value};
use std::collections::hash_map::Values;
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

pub fn get_lol_end_of_game_v1_champion_mastery_updates() -> GetLolEndOfGameV1ChampionMasteryUpdates {
    GetLolEndOfGameV1ChampionMasteryUpdates{}
}


pub struct GetLolEndOfGameV1EogStatsBlock {}

impl IsApiRequest for GetLolEndOfGameV1EogStatsBlock {
    const METHOD: Method = Method::GET;
    type ReturnType = LolEndOfGameEndOfGameStats;
    fn get_url(&self) -> String {"/lol-end-of-game/v1/eog-stats-block".to_string()}
}

pub fn get_lol_end_of_game_v1_eog_stats_block() -> GetLolEndOfGameV1EogStatsBlock {
    GetLolEndOfGameV1EogStatsBlock{}
}


pub struct GetLolEndOfGameV1GameclientEogStatsBlock {}

impl IsApiRequest for GetLolEndOfGameV1GameclientEogStatsBlock {
    const METHOD: Method = Method::GET;
    type ReturnType = LolEndOfGameGameClientEndOfGameStats;
    fn get_url(&self) -> String {"/lol-end-of-game/v1/gameclient-eog-stats-block".to_string()}
}

pub fn get_lol_end_of_game_v1_gameclient_eog_stats_block() -> GetLolEndOfGameV1GameclientEogStatsBlock {
    GetLolEndOfGameV1GameclientEogStatsBlock{}
}


pub struct GetLolEndOfGameV1TftEogStats {}

impl IsApiRequest for GetLolEndOfGameV1TftEogStats {
    const METHOD: Method = Method::GET;
    type ReturnType = LolEndOfGameTftEndOfGameViewModel;
    fn get_url(&self) -> String {"/lol-end-of-game/v1/tft-eog-stats".to_string()}
}

pub fn get_lol_end_of_game_v1_tft_eog_stats() -> GetLolEndOfGameV1TftEogStats {
    GetLolEndOfGameV1TftEogStats{}
}


pub struct PostLolEndOfGameV1GameclientEogStatsBlock {
    pub body: LolEndOfGameGameClientEndOfGameStats,
}

impl IsApiRequest for PostLolEndOfGameV1GameclientEogStatsBlock {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;
    fn get_url(&self) -> String {"/lol-end-of-game/v1/gameclient-eog-stats-block".to_string()}
    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }
}

pub fn post_lol_end_of_game_v1_gameclient_eog_stats_block(body: LolEndOfGameGameClientEndOfGameStats) -> PostLolEndOfGameV1GameclientEogStatsBlock {
    PostLolEndOfGameV1GameclientEogStatsBlock{body}
}


pub struct PostLolEndOfGameV1StateDismissStats {}

impl IsApiRequest for PostLolEndOfGameV1StateDismissStats {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;
    fn get_url(&self) -> String {"/lol-end-of-game/v1/state/dismiss-stats".to_string()}
}

pub fn post_lol_end_of_game_v1_state_dismiss_stats() -> PostLolEndOfGameV1StateDismissStats {
    PostLolEndOfGameV1StateDismissStats{}
}


// OBJECTS

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolEndOfGameChampionMasteryGrade {
    pub puuid: String,
    #[serde(rename = "championId")]
    pub champion_id: i32,
    pub grade: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolEndOfGameChampionMasteryMini {
    pub puuid: String,
    #[serde(rename = "championId")]
    pub champion_id: i32,
    #[serde(rename = "championLevel")]
    pub champion_level: i64,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolEndOfGameChampionMasteryUpdate {
    pub id: String,
    #[serde(rename = "gameId")]
    pub game_id: u64,
    pub puuid: String,
    #[serde(rename = "championId")]
    pub champion_id: i32,
    #[serde(rename = "hasLeveledUp")]
    pub has_leveled_up: bool,
    pub level: i64,
    #[serde(rename = "pointsBeforeGame")]
    pub points_before_game: i64,
    #[serde(rename = "pointsGained")]
    pub points_gained: i64,
    #[serde(rename = "pointsGainedIndividualContribution")]
    pub points_gained_individual_contribution: i64,
    #[serde(rename = "bonusPointsGained")]
    pub bonus_points_gained: i64,
    #[serde(rename = "pointsSinceLastLevelBeforeGame")]
    pub points_since_last_level_before_game: i64,
    #[serde(rename = "pointsUntilNextLevelBeforeGame")]
    pub points_until_next_level_before_game: i64,
    #[serde(rename = "pointsUntilNextLevelAfterGame")]
    pub points_until_next_level_after_game: i64,
    #[serde(rename = "tokensEarned")]
    pub tokens_earned: i64,
    #[serde(rename = "tokenEarnedAfterGame")]
    pub token_earned_after_game: bool,
    pub grade: String,
    pub score: i64,
    #[serde(rename = "levelUpList")]
    pub level_up_list: Vec<LolEndOfGameChampionMasteryMini>,
    #[serde(rename = "memberGrades")]
    pub member_grades: Vec<LolEndOfGameChampionMasteryGrade>,
    #[serde(rename = "mvpPuuid")]
    pub mvp_puuid: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolEndOfGameEndOfGamePlayer {
    pub stats: Value,
    pub items: Vec<i32>,
    pub puuid: String,
    #[serde(rename = "botPlayer")]
    pub bot_player: bool,
    #[serde(rename = "championId")]
    pub champion_id: i32,
    #[serde(rename = "gameId")]
    pub game_id: u64,
    pub leaver: bool,
    pub leaves: i32,
    pub level: i32,
    pub losses: i32,
    #[serde(rename = "profileIconId")]
    pub profile_icon_id: i32,
    #[serde(rename = "spell1Id")]
    pub spell_1_id: i32,
    #[serde(rename = "spell2Id")]
    pub spell_2_id: i32,
    #[serde(rename = "summonerName")]
    pub summoner_name: String,
    #[serde(rename = "teamId")]
    pub team_id: i32,
    pub wins: i32,
    #[serde(rename = "summonerId")]
    pub summoner_id: u64,
    #[serde(rename = "selectedPosition")]
    pub selected_position: String,
    #[serde(rename = "detectedTeamPosition")]
    pub detected_team_position: String,
    #[serde(rename = "skinSplashPath")]
    pub skin_splash_path: String,
    #[serde(rename = "skinTilePath")]
    pub skin_tile_path: String,
    #[serde(rename = "skinEmblemPaths")]
    pub skin_emblem_paths: Vec<String>,
    #[serde(rename = "championName")]
    pub champion_name: String,
    #[serde(rename = "championSquarePortraitPath")]
    pub champion_square_portrait_path: String,
    #[serde(rename = "isLocalPlayer")]
    pub is_local_player: bool,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolEndOfGameEndOfGamePoints {
    #[serde(rename = "pointChangeFromChampionsOwned")]
    pub point_change_from_champions_owned: i32,
    #[serde(rename = "pointChangeFromGameplay")]
    pub point_change_from_gameplay: i32,
    #[serde(rename = "pointsUsed")]
    pub points_used: i32,
    #[serde(rename = "previousPoints")]
    pub previous_points: i32,
    #[serde(rename = "pointsUntilNextReroll")]
    pub points_until_next_reroll: i32,
    #[serde(rename = "rerollCount")]
    pub reroll_count: i32,
    #[serde(rename = "totalPoints")]
    pub total_points: i32,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolEndOfGameEndOfGameStats {
    pub difficulty: String,
    #[serde(rename = "gameId")]
    pub game_id: u64,
    #[serde(rename = "gameLength")]
    pub game_length: i32,
    #[serde(rename = "gameMode")]
    pub game_mode: String,
    #[serde(rename = "gameMutators")]
    pub game_mutators: Vec<String>,
    #[serde(rename = "gameType")]
    pub game_type: String,
    pub invalid: bool,
    #[serde(rename = "queueType")]
    pub queue_type: String,
    pub ranked: bool,
    #[serde(rename = "reportGameId")]
    pub report_game_id: u64,
    #[serde(rename = "multiUserChatId")]
    pub multi_user_chat_id: String,
    #[serde(rename = "multiUserChatPassword")]
    pub multi_user_chat_password: String,
    #[serde(rename = "mucJwtDto")]
    pub muc_jwt_dto: LolEndOfGameMucJwtDto,
    pub teams: Vec<LolEndOfGameEndOfGameTeam>,
    #[serde(rename = "localPlayer")]
    pub local_player: LolEndOfGameEndOfGamePlayer,
    #[serde(rename = "myTeamStatus")]
    pub my_team_status: String,
    #[serde(rename = "leveledUp")]
    pub leveled_up: bool,
    #[serde(rename = "newSpells")]
    pub new_spells: Vec<i32>,
    #[serde(rename = "previousLevel")]
    pub previous_level: u64,
    #[serde(rename = "rpEarned")]
    pub rp_earned: i32,
    #[serde(rename = "basePoints")]
    pub base_points: i32,
    #[serde(rename = "battleBoostIpEarned")]
    pub battle_boost_ip_earned: i32,
    #[serde(rename = "boostIpEarned")]
    pub boost_ip_earned: i32,
    #[serde(rename = "firstWinBonus")]
    pub first_win_bonus: i32,
    #[serde(rename = "ipEarned")]
    pub ip_earned: i32,
    #[serde(rename = "ipTotal")]
    pub ip_total: i32,
    #[serde(rename = "boostXpEarned")]
    pub boost_xp_earned: i32,
    #[serde(rename = "experienceEarned")]
    pub experience_earned: i32,
    #[serde(rename = "experienceTotal")]
    pub experience_total: i32,
    #[serde(rename = "globalBoostXpEarned")]
    pub global_boost_xp_earned: i32,
    #[serde(rename = "loyaltyBoostXpEarned")]
    pub loyalty_boost_xp_earned: i32,
    #[serde(rename = "xbgpBoostXpEarned")]
    pub xbgp_boost_xp_earned: i32,
    #[serde(rename = "missionsXpEarned")]
    pub missions_xp_earned: i32,
    #[serde(rename = "previousXpTotal")]
    pub previous_xp_total: u64,
    #[serde(rename = "nextLevelXp")]
    pub next_level_xp: u64,
    #[serde(rename = "currentLevel")]
    pub current_level: u64,
    #[serde(rename = "preLevelUpExperienceTotal")]
    pub pre_level_up_experience_total: u64,
    #[serde(rename = "preLevelUpNextLevelXp")]
    pub pre_level_up_next_level_xp: u64,
    #[serde(rename = "timeUntilNextFirstWinBonus")]
    pub time_until_next_first_win_bonus: i32,
    #[serde(rename = "causedEarlySurrender")]
    pub caused_early_surrender: bool,
    #[serde(rename = "earlySurrenderAccomplice")]
    pub early_surrender_accomplice: bool,
    #[serde(rename = "teamEarlySurrendered")]
    pub team_early_surrendered: bool,
    #[serde(rename = "gameEndedInEarlySurrender")]
    pub game_ended_in_early_surrender: bool,
    #[serde(rename = "rerollData")]
    pub reroll_data: LolEndOfGameEndOfGamePoints,
    #[serde(rename = "teamBoost")]
    pub team_boost: Option<LolEndOfGameEndOfGameTeamBoost>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolEndOfGameEndOfGameTeam {
    pub stats: Value,
    pub players: Vec<LolEndOfGameEndOfGamePlayer>,
    #[serde(rename = "memberStatusString")]
    pub member_status_string: String,
    pub name: String,
    pub tag: String,
    #[serde(rename = "fullId")]
    pub full_id: String,
    #[serde(rename = "teamId")]
    pub team_id: i32,
    #[serde(rename = "isBottomTeam")]
    pub is_bottom_team: bool,
    #[serde(rename = "isPlayerTeam")]
    pub is_player_team: bool,
    #[serde(rename = "isWinningTeam")]
    pub is_winning_team: bool,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolEndOfGameEndOfGameTeamBoost {
    #[serde(rename = "summonerName")]
    pub summoner_name: String,
    #[serde(rename = "skinUnlockMode")]
    pub skin_unlock_mode: String,
    pub price: i64,
    #[serde(rename = "ipReward")]
    pub ip_reward: i64,
    #[serde(rename = "ipRewardForPurchaser")]
    pub ip_reward_for_purchaser: i64,
    #[serde(rename = "availableSkins")]
    pub available_skins: Vec<i64>,
    pub unlocked: bool,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolEndOfGameGameClientEndOfGameStats {
    #[serde(rename = "gameId")]
    pub game_id: u64,
    #[serde(rename = "gameMode")]
    pub game_mode: String,
    #[serde(rename = "statsBlock")]
    pub stats_block: Value,
    #[serde(rename = "queueId")]
    pub queue_id: i32,
    #[serde(rename = "queueType")]
    pub queue_type: String,
    #[serde(rename = "isRanked")]
    pub is_ranked: bool,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolEndOfGameMucJwtDto {
    pub jwt: String,
    #[serde(rename = "channelClaim")]
    pub channel_claim: String,
    pub domain: String,
    #[serde(rename = "targetRegion")]
    pub target_region: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolEndOfGameTftEndOfGameCompanionViewModel {
    #[serde(rename = "speciesName")]
    pub species_name: String,
    #[serde(rename = "colorName")]
    pub color_name: String,
    pub icon: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolEndOfGameTftEndOfGameCustomAugmentContainerViewModel {
    #[serde(rename = "nameId")]
    pub name_id: String,
    #[serde(rename = "iconPath")]
    pub icon_path: String,
    #[serde(rename = "displayName")]
    pub display_name: String,
    pub description: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolEndOfGameTftEndOfGameItemViewModel {
    pub name: String,
    pub icon: String,
    pub id: i32,
    #[serde(rename = "nameId")]
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
    #[serde(rename = "itemId")]
    pub item_id: u32,
    pub name: String,
    #[serde(rename = "iconSmall")]
    pub icon_small: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolEndOfGameTftEndOfGamePlayerViewModel {
    #[serde(rename = "summonerId")]
    pub summoner_id: u64,
    #[serde(rename = "summonerName")]
    pub summoner_name: String,
    #[serde(rename = "riotIdGameName")]
    pub riot_id_game_name: String,
    #[serde(rename = "riotIdTagLine")]
    pub riot_id_tag_line: String,
    #[serde(rename = "iconId")]
    pub icon_id: i32,
    pub puuid: String,
    #[serde(rename = "ffaStanding")]
    pub ffa_standing: u8,
    pub health: u8,
    pub rank: u8,
    #[serde(rename = "isLocalPlayer")]
    pub is_local_player: bool,
    #[serde(rename = "isInteractable")]
    pub is_interactable: bool,
    #[serde(rename = "partnerGroupId")]
    pub partner_group_id: u8,
    #[serde(rename = "boardPieces")]
    pub board_pieces: Vec<LolEndOfGameTftEndOfGamePieceViewModel>,
    pub augments: Vec<LolEndOfGameTftEndOfGameItemViewModel>,
    pub companion: LolEndOfGameTftEndOfGameCompanionViewModel,
    pub playbook: LolEndOfGameTftEndOfGamePlaybookViewModel,
    #[serde(rename = "customAugmentContainer")]
    pub custom_augment_container: LolEndOfGameTftEndOfGameCustomAugmentContainerViewModel,
    #[serde(rename = "setCoreName")]
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
    #[serde(rename = "localPlayer")]
    pub local_player: Option<LolEndOfGameTftEndOfGamePlayerViewModel>,
    #[serde(rename = "gameLength")]
    pub game_length: u32,
    #[serde(rename = "gameId")]
    pub game_id: u64,
    #[serde(rename = "queueId")]
    pub queue_id: i32,
    #[serde(rename = "queueType")]
    pub queue_type: String,
    #[serde(rename = "isRanked")]
    pub is_ranked: bool,
}


// ENUMS

