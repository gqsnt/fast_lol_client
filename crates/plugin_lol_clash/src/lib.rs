
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use serde_json::{json, Value, to_value};
use std::collections::hash_map::Values;
use reqwest::Method;
use common::IsApiRequest;

mod additional;

// ENDPOINTS

pub struct DeleteLolClashV1Voice {}

impl IsApiRequest for DeleteLolClashV1Voice {
    const METHOD: Method = Method::DELETE;
    type ReturnType = Value;
    fn get_url(&self) -> String {"/lol-clash/v1/voice".to_string()}
}

pub fn delete_lol_clash_v1_voice() -> DeleteLolClashV1Voice {
    DeleteLolClashV1Voice{}
}


pub struct DeleteLolClashV1VoiceDelayByDelaySeconds {
    pub delay_seconds: f64,
}

impl IsApiRequest for DeleteLolClashV1VoiceDelayByDelaySeconds {
    const METHOD: Method = Method::DELETE;
    type ReturnType = Value;
    fn get_url(&self) -> String {format!("/lol-clash/v1/voice-delay/{}", self.delay_seconds)}
}

pub fn delete_lol_clash_v1_voice_delay_by_delay_seconds(delay_seconds: f64) -> DeleteLolClashV1VoiceDelayByDelaySeconds {
    DeleteLolClashV1VoiceDelayByDelaySeconds{delay_seconds}
}


pub struct GetLolClashV1AllTournaments {}

impl IsApiRequest for GetLolClashV1AllTournaments {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<TournamentDto>;
    fn get_url(&self) -> String {"/lol-clash/v1/all-tournaments".to_string()}
}

pub fn get_lol_clash_v1_all_tournaments() -> GetLolClashV1AllTournaments {
    GetLolClashV1AllTournaments{}
}


pub struct GetLolClashV1AwaitingResentEog {}

impl IsApiRequest for GetLolClashV1AwaitingResentEog {
    const METHOD: Method = Method::GET;
    type ReturnType = bool;
    fn get_url(&self) -> String {"/lol-clash/v1/awaiting-resent-eog".to_string()}
}

pub fn get_lol_clash_v1_awaiting_resent_eog() -> GetLolClashV1AwaitingResentEog {
    GetLolClashV1AwaitingResentEog{}
}


pub struct GetLolClashV1BracketByBracketId {
    pub bracket_id: i64,
}

impl IsApiRequest for GetLolClashV1BracketByBracketId {
    const METHOD: Method = Method::GET;
    type ReturnType = LolClashBracket;
    fn get_url(&self) -> String {format!("/lol-clash/v1/bracket/{}", self.bracket_id)}
}

pub fn get_lol_clash_v1_bracket_by_bracket_id(bracket_id: i64) -> GetLolClashV1BracketByBracketId {
    GetLolClashV1BracketByBracketId{bracket_id}
}


pub struct GetLolClashV1CheckinAllowed {}

impl IsApiRequest for GetLolClashV1CheckinAllowed {
    const METHOD: Method = Method::GET;
    type ReturnType = bool;
    fn get_url(&self) -> String {"/lol-clash/v1/checkin-allowed".to_string()}
}

pub fn get_lol_clash_v1_checkin_allowed() -> GetLolClashV1CheckinAllowed {
    GetLolClashV1CheckinAllowed{}
}


pub struct GetLolClashV1CurrentTournamentIds {}

impl IsApiRequest for GetLolClashV1CurrentTournamentIds {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<i64>;
    fn get_url(&self) -> String {"/lol-clash/v1/currentTournamentIds".to_string()}
}

pub fn get_lol_clash_v1_current_tournament_ids() -> GetLolClashV1CurrentTournamentIds {
    GetLolClashV1CurrentTournamentIds{}
}


pub struct GetLolClashV1DisabledConfig {}

impl IsApiRequest for GetLolClashV1DisabledConfig {
    const METHOD: Method = Method::GET;
    type ReturnType = LolClashClashDisabledConfig;
    fn get_url(&self) -> String {"/lol-clash/v1/disabled-config".to_string()}
}

pub fn get_lol_clash_v1_disabled_config() -> GetLolClashV1DisabledConfig {
    GetLolClashV1DisabledConfig{}
}


pub struct GetLolClashV1Enabled {}

impl IsApiRequest for GetLolClashV1Enabled {
    const METHOD: Method = Method::GET;
    type ReturnType = bool;
    fn get_url(&self) -> String {"/lol-clash/v1/enabled".to_string()}
}

pub fn get_lol_clash_v1_enabled() -> GetLolClashV1Enabled {
    GetLolClashV1Enabled{}
}


pub struct GetLolClashV1EogPlayerUpdate {}

impl IsApiRequest for GetLolClashV1EogPlayerUpdate {
    const METHOD: Method = Method::GET;
    type ReturnType = LolClashEogPlayerUpdateDto;
    fn get_url(&self) -> String {"/lol-clash/v1/eog-player-update".to_string()}
}

pub fn get_lol_clash_v1_eog_player_update() -> GetLolClashV1EogPlayerUpdate {
    GetLolClashV1EogPlayerUpdate{}
}


pub struct GetLolClashV1EventByUuid {
    pub uuid: String,
}

impl IsApiRequest for GetLolClashV1EventByUuid {
    const METHOD: Method = Method::GET;
    type ReturnType = ClashEventData;
    fn get_url(&self) -> String {format!("/lol-clash/v1/event/{}", self.uuid)}
}

pub fn get_lol_clash_v1_event_by_uuid(uuid: String) -> GetLolClashV1EventByUuid {
    GetLolClashV1EventByUuid{uuid}
}


pub struct GetLolClashV1GameEnd {}

impl IsApiRequest for GetLolClashV1GameEnd {
    const METHOD: Method = Method::GET;
    type ReturnType = LolClashTournamentGameEnd;
    fn get_url(&self) -> String {"/lol-clash/v1/game-end".to_string()}
}

pub fn get_lol_clash_v1_game_end() -> GetLolClashV1GameEnd {
    GetLolClashV1GameEnd{}
}


pub struct GetLolClashV1Historyandwinners {}

impl IsApiRequest for GetLolClashV1Historyandwinners {
    const METHOD: Method = Method::GET;
    type ReturnType = LolClashTournamentHistoryAndWinners;
    fn get_url(&self) -> String {"/lol-clash/v1/historyandwinners".to_string()}
}

pub fn get_lol_clash_v1_historyandwinners() -> GetLolClashV1Historyandwinners {
    GetLolClashV1Historyandwinners{}
}


pub struct GetLolClashV1Iconconfig {}

impl IsApiRequest for GetLolClashV1Iconconfig {
    const METHOD: Method = Method::GET;
    type ReturnType = Value;
    fn get_url(&self) -> String {"/lol-clash/v1/iconconfig".to_string()}
}

pub fn get_lol_clash_v1_iconconfig() -> GetLolClashV1Iconconfig {
    GetLolClashV1Iconconfig{}
}


pub struct GetLolClashV1InvitedRosterIds {}

impl IsApiRequest for GetLolClashV1InvitedRosterIds {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<String>;
    fn get_url(&self) -> String {"/lol-clash/v1/invited-roster-ids".to_string()}
}

pub fn get_lol_clash_v1_invited_roster_ids() -> GetLolClashV1InvitedRosterIds {
    GetLolClashV1InvitedRosterIds{}
}


pub struct GetLolClashV1LftTeamRequests {}

impl IsApiRequest for GetLolClashV1LftTeamRequests {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<PendingOpenedTeamDto>;
    fn get_url(&self) -> String {"/lol-clash/v1/lft/team/requests".to_string()}
}

pub fn get_lol_clash_v1_lft_team_requests() -> GetLolClashV1LftTeamRequests {
    GetLolClashV1LftTeamRequests{}
}


pub struct GetLolClashV1Notifications {}

impl IsApiRequest for GetLolClashV1Notifications {
    const METHOD: Method = Method::GET;
    type ReturnType = LolClashPlayerNotificationData;
    fn get_url(&self) -> String {"/lol-clash/v1/notifications".to_string()}
}

pub fn get_lol_clash_v1_notifications() -> GetLolClashV1Notifications {
    GetLolClashV1Notifications{}
}


pub struct GetLolClashV1Ping {}

impl IsApiRequest for GetLolClashV1Ping {
    const METHOD: Method = Method::GET;
    type ReturnType = Value;
    fn get_url(&self) -> String {"/lol-clash/v1/ping".to_string()}
}

pub fn get_lol_clash_v1_ping() -> GetLolClashV1Ping {
    GetLolClashV1Ping{}
}


pub struct GetLolClashV1Player {}

impl IsApiRequest for GetLolClashV1Player {
    const METHOD: Method = Method::GET;
    type ReturnType = LolClashPlayerData;
    fn get_url(&self) -> String {"/lol-clash/v1/player".to_string()}
}

pub fn get_lol_clash_v1_player() -> GetLolClashV1Player {
    GetLolClashV1Player{}
}


pub struct GetLolClashV1PlayerChatRosters {}

impl IsApiRequest for GetLolClashV1PlayerChatRosters {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<LolClashPlayerChatRoster>;
    fn get_url(&self) -> String {"/lol-clash/v1/player/chat-rosters".to_string()}
}

pub fn get_lol_clash_v1_player_chat_rosters() -> GetLolClashV1PlayerChatRosters {
    GetLolClashV1PlayerChatRosters{}
}


pub struct GetLolClashV1PlayerHistory {}

impl IsApiRequest for GetLolClashV1PlayerHistory {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<LolClashRosterStats>;
    fn get_url(&self) -> String {"/lol-clash/v1/player/history".to_string()}
}

pub fn get_lol_clash_v1_player_history() -> GetLolClashV1PlayerHistory {
    GetLolClashV1PlayerHistory{}
}


pub struct GetLolClashV1PlaymodeRestricted {}

impl IsApiRequest for GetLolClashV1PlaymodeRestricted {
    const METHOD: Method = Method::GET;
    type ReturnType = bool;
    fn get_url(&self) -> String {"/lol-clash/v1/playmode-restricted".to_string()}
}

pub fn get_lol_clash_v1_playmode_restricted() -> GetLolClashV1PlaymodeRestricted {
    GetLolClashV1PlaymodeRestricted{}
}


pub struct GetLolClashV1Ready {}

impl IsApiRequest for GetLolClashV1Ready {
    const METHOD: Method = Method::GET;
    type ReturnType = bool;
    fn get_url(&self) -> String {"/lol-clash/v1/ready".to_string()}
}

pub fn get_lol_clash_v1_ready() -> GetLolClashV1Ready {
    GetLolClashV1Ready{}
}


pub struct GetLolClashV1Rewards {}

impl IsApiRequest for GetLolClashV1Rewards {
    const METHOD: Method = Method::GET;
    type ReturnType = LolClashPlayerRewards;
    fn get_url(&self) -> String {"/lol-clash/v1/rewards".to_string()}
}

pub fn get_lol_clash_v1_rewards() -> GetLolClashV1Rewards {
    GetLolClashV1Rewards{}
}


pub struct GetLolClashV1RosterByRosterId {
    pub roster_id: String,
}

impl IsApiRequest for GetLolClashV1RosterByRosterId {
    const METHOD: Method = Method::GET;
    type ReturnType = LolClashRoster;
    fn get_url(&self) -> String {format!("/lol-clash/v1/roster/{}", self.roster_id)}
}

pub fn get_lol_clash_v1_roster_by_roster_id(roster_id: String) -> GetLolClashV1RosterByRosterId {
    GetLolClashV1RosterByRosterId{roster_id}
}


pub struct GetLolClashV1RosterByRosterIdStats {
    pub roster_id: i64,
}

impl IsApiRequest for GetLolClashV1RosterByRosterIdStats {
    const METHOD: Method = Method::GET;
    type ReturnType = LolClashRosterStats;
    fn get_url(&self) -> String {format!("/lol-clash/v1/roster/{}/stats", self.roster_id)}
}

pub fn get_lol_clash_v1_roster_by_roster_id_stats(roster_id: i64) -> GetLolClashV1RosterByRosterIdStats {
    GetLolClashV1RosterByRosterIdStats{roster_id}
}


pub struct GetLolClashV1ScoutingChampions {
    pub puuids: Vec<String>,
}

impl IsApiRequest for GetLolClashV1ScoutingChampions {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<LolClashScoutingChampions>;
    fn get_url(&self) -> String {"/lol-clash/v1/scouting/champions".to_string()}
    fn get_query(&self) -> Option<Vec<(String,String)>> {
        Some(vec![
            ("puuids".to_string(), serde_json::to_string(&self.puuids).unwrap())
        ])
    }
}

pub fn get_lol_clash_v1_scouting_champions(puuids: Vec<String>) -> GetLolClashV1ScoutingChampions {
    GetLolClashV1ScoutingChampions{puuids}
}


pub struct GetLolClashV1ScoutingMatchhistory {
    pub summoner_ids: Vec<u64>,
}

impl IsApiRequest for GetLolClashV1ScoutingMatchhistory {
    const METHOD: Method = Method::GET;
    type ReturnType = Value;
    fn get_url(&self) -> String {"/lol-clash/v1/scouting/matchhistory".to_string()}
    fn get_query(&self) -> Option<Vec<(String,String)>> {
        Some(vec![
            ("summonerIds".to_string(), serde_json::to_string(&self.summoner_ids).unwrap())
        ])
    }
}

pub fn get_lol_clash_v1_scouting_matchhistory(summoner_ids: Vec<u64>) -> GetLolClashV1ScoutingMatchhistory {
    GetLolClashV1ScoutingMatchhistory{summoner_ids}
}


pub struct GetLolClashV1SeasonRewardsBySeasonId {
    pub season_id: i32,
}

impl IsApiRequest for GetLolClashV1SeasonRewardsBySeasonId {
    const METHOD: Method = Method::GET;
    type ReturnType = ClashSeasonRewardResult;
    fn get_url(&self) -> String {format!("/lol-clash/v1/season-rewards/{}", self.season_id)}
}

pub fn get_lol_clash_v1_season_rewards_by_season_id(season_id: i32) -> GetLolClashV1SeasonRewardsBySeasonId {
    GetLolClashV1SeasonRewardsBySeasonId{season_id}
}


pub struct GetLolClashV1SimpleStateFlags {}

impl IsApiRequest for GetLolClashV1SimpleStateFlags {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<LolClashSimpleStateFlag>;
    fn get_url(&self) -> String {"/lol-clash/v1/simple-state-flags".to_string()}
}

pub fn get_lol_clash_v1_simple_state_flags() -> GetLolClashV1SimpleStateFlags {
    GetLolClashV1SimpleStateFlags{}
}


pub struct GetLolClashV1ThirdpartyTeamData {}

impl IsApiRequest for GetLolClashV1ThirdpartyTeamData {
    const METHOD: Method = Method::GET;
    type ReturnType = LolClashThirdPartyApiRoster;
    fn get_url(&self) -> String {"/lol-clash/v1/thirdparty/team-data".to_string()}
}

pub fn get_lol_clash_v1_thirdparty_team_data() -> GetLolClashV1ThirdpartyTeamData {
    GetLolClashV1ThirdpartyTeamData{}
}


pub struct GetLolClashV1Time {}

impl IsApiRequest for GetLolClashV1Time {
    const METHOD: Method = Method::GET;
    type ReturnType = i64;
    fn get_url(&self) -> String {"/lol-clash/v1/time".to_string()}
}

pub fn get_lol_clash_v1_time() -> GetLolClashV1Time {
    GetLolClashV1Time{}
}


pub struct GetLolClashV1TournamentByTournamentId {
    pub tournament_id: i64,
}

impl IsApiRequest for GetLolClashV1TournamentByTournamentId {
    const METHOD: Method = Method::GET;
    type ReturnType = LolClashTournament;
    fn get_url(&self) -> String {format!("/lol-clash/v1/tournament/{}", self.tournament_id)}
}

pub fn get_lol_clash_v1_tournament_by_tournament_id(tournament_id: i64) -> GetLolClashV1TournamentByTournamentId {
    GetLolClashV1TournamentByTournamentId{tournament_id}
}


pub struct GetLolClashV1TournamentByTournamentIdGetPlayerTiers {
    pub tournament_id: i64,
    pub summoner_ids: Vec<u64>,
}

impl IsApiRequest for GetLolClashV1TournamentByTournamentIdGetPlayerTiers {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<PlayerTierDto>;
    fn get_url(&self) -> String {format!("/lol-clash/v1/tournament/{}/get-player-tiers", self.tournament_id)}
    fn get_query(&self) -> Option<Vec<(String,String)>> {
        Some(vec![
            ("summonerIds".to_string(), serde_json::to_string(&self.summoner_ids).unwrap())
        ])
    }
}

pub fn get_lol_clash_v1_tournament_by_tournament_id_get_player_tiers(tournament_id: i64, summoner_ids: Vec<u64>) -> GetLolClashV1TournamentByTournamentIdGetPlayerTiers {
    GetLolClashV1TournamentByTournamentIdGetPlayerTiers{tournament_id, summoner_ids}
}


pub struct GetLolClashV1TournamentByTournamentIdPlayer {
    pub tournament_id: i64,
}

impl IsApiRequest for GetLolClashV1TournamentByTournamentIdPlayer {
    const METHOD: Method = Method::GET;
    type ReturnType = LolClashPlayerTournamentData;
    fn get_url(&self) -> String {format!("/lol-clash/v1/tournament/{}/player", self.tournament_id)}
}

pub fn get_lol_clash_v1_tournament_by_tournament_id_player(tournament_id: i64) -> GetLolClashV1TournamentByTournamentIdPlayer {
    GetLolClashV1TournamentByTournamentIdPlayer{tournament_id}
}


pub struct GetLolClashV1TournamentByTournamentIdPlayerHonorRestricted {
    pub tournament_id: i64,
}

impl IsApiRequest for GetLolClashV1TournamentByTournamentIdPlayerHonorRestricted {
    const METHOD: Method = Method::GET;
    type ReturnType = bool;
    fn get_url(&self) -> String {format!("/lol-clash/v1/tournament/{}/player-honor-restricted", self.tournament_id)}
}

pub fn get_lol_clash_v1_tournament_by_tournament_id_player_honor_restricted(tournament_id: i64) -> GetLolClashV1TournamentByTournamentIdPlayerHonorRestricted {
    GetLolClashV1TournamentByTournamentIdPlayerHonorRestricted{tournament_id}
}


pub struct GetLolClashV1TournamentByTournamentIdStateInfo {
    pub tournament_id: i64,
}

impl IsApiRequest for GetLolClashV1TournamentByTournamentIdStateInfo {
    const METHOD: Method = Method::GET;
    type ReturnType = LolClashTournamentStateInfo;
    fn get_url(&self) -> String {format!("/lol-clash/v1/tournament/{}/stateInfo", self.tournament_id)}
}

pub fn get_lol_clash_v1_tournament_by_tournament_id_state_info(tournament_id: i64) -> GetLolClashV1TournamentByTournamentIdStateInfo {
    GetLolClashV1TournamentByTournamentIdStateInfo{tournament_id}
}


pub struct GetLolClashV1TournamentByTournamentIdWinners {
    pub tournament_id: i64,
}

impl IsApiRequest for GetLolClashV1TournamentByTournamentIdWinners {
    const METHOD: Method = Method::GET;
    type ReturnType = LolClashTournamentWinnerHistory;
    fn get_url(&self) -> String {format!("/lol-clash/v1/tournament/{}/winners", self.tournament_id)}
}

pub fn get_lol_clash_v1_tournament_by_tournament_id_winners(tournament_id: i64) -> GetLolClashV1TournamentByTournamentIdWinners {
    GetLolClashV1TournamentByTournamentIdWinners{tournament_id}
}


pub struct GetLolClashV1TournamentCancelled {}

impl IsApiRequest for GetLolClashV1TournamentCancelled {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<i64>;
    fn get_url(&self) -> String {"/lol-clash/v1/tournament/cancelled".to_string()}
}

pub fn get_lol_clash_v1_tournament_cancelled() -> GetLolClashV1TournamentCancelled {
    GetLolClashV1TournamentCancelled{}
}


pub struct GetLolClashV1TournamentGetAllPlayerTiers {}

impl IsApiRequest for GetLolClashV1TournamentGetAllPlayerTiers {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<PlayerTierDto>;
    fn get_url(&self) -> String {"/lol-clash/v1/tournament/get-all-player-tiers".to_string()}
}

pub fn get_lol_clash_v1_tournament_get_all_player_tiers() -> GetLolClashV1TournamentGetAllPlayerTiers {
    GetLolClashV1TournamentGetAllPlayerTiers{}
}


pub struct GetLolClashV1TournamentStateInfo {}

impl IsApiRequest for GetLolClashV1TournamentStateInfo {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<LolClashTournamentStateInfo>;
    fn get_url(&self) -> String {"/lol-clash/v1/tournament-state-info".to_string()}
}

pub fn get_lol_clash_v1_tournament_state_info() -> GetLolClashV1TournamentStateInfo {
    GetLolClashV1TournamentStateInfo{}
}


pub struct GetLolClashV1TournamentSummary {}

impl IsApiRequest for GetLolClashV1TournamentSummary {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<LolClashTournamentSummary>;
    fn get_url(&self) -> String {"/lol-clash/v1/tournament-summary".to_string()}
}

pub fn get_lol_clash_v1_tournament_summary() -> GetLolClashV1TournamentSummary {
    GetLolClashV1TournamentSummary{}
}


pub struct GetLolClashV1Visible {}

impl IsApiRequest for GetLolClashV1Visible {
    const METHOD: Method = Method::GET;
    type ReturnType = bool;
    fn get_url(&self) -> String {"/lol-clash/v1/visible".to_string()}
}

pub fn get_lol_clash_v1_visible() -> GetLolClashV1Visible {
    GetLolClashV1Visible{}
}


pub struct GetLolClashV1VoiceEnabled {}

impl IsApiRequest for GetLolClashV1VoiceEnabled {
    const METHOD: Method = Method::GET;
    type ReturnType = bool;
    fn get_url(&self) -> String {"/lol-clash/v1/voice-enabled".to_string()}
}

pub fn get_lol_clash_v1_voice_enabled() -> GetLolClashV1VoiceEnabled {
    GetLolClashV1VoiceEnabled{}
}


pub struct GetLolClashV2PlaymodeRestricted {}

impl IsApiRequest for GetLolClashV2PlaymodeRestricted {
    const METHOD: Method = Method::GET;
    type ReturnType = LolClashPlaymodeRestrictedInfo;
    fn get_url(&self) -> String {"/lol-clash/v2/playmode-restricted".to_string()}
}

pub fn get_lol_clash_v2_playmode_restricted() -> GetLolClashV2PlaymodeRestricted {
    GetLolClashV2PlaymodeRestricted{}
}


pub struct PostLolClashV1EogPlayerUpdateAcknowledge {}

impl IsApiRequest for PostLolClashV1EogPlayerUpdateAcknowledge {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;
    fn get_url(&self) -> String {"/lol-clash/v1/eog-player-update/acknowledge".to_string()}
}

pub fn post_lol_clash_v1_eog_player_update_acknowledge() -> PostLolClashV1EogPlayerUpdateAcknowledge {
    PostLolClashV1EogPlayerUpdateAcknowledge{}
}


pub struct PostLolClashV1Events {
    pub body: Vec<String>,
}

impl IsApiRequest for PostLolClashV1Events {
    const METHOD: Method = Method::POST;
    type ReturnType = HashMap<String, ClashEventData>;
    fn get_url(&self) -> String {"/lol-clash/v1/events".to_string()}
    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }
}

pub fn post_lol_clash_v1_events(body: Vec<String>) -> PostLolClashV1Events {
    PostLolClashV1Events{body}
}


pub struct PostLolClashV1GameEndAcknowledge {}

impl IsApiRequest for PostLolClashV1GameEndAcknowledge {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;
    fn get_url(&self) -> String {"/lol-clash/v1/game-end/acknowledge".to_string()}
}

pub fn post_lol_clash_v1_game_end_acknowledge() -> PostLolClashV1GameEndAcknowledge {
    PostLolClashV1GameEndAcknowledge{}
}


pub struct PostLolClashV1LftPlayer {
    pub body: LolClashLftState,
}

impl IsApiRequest for PostLolClashV1LftPlayer {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;
    fn get_url(&self) -> String {"/lol-clash/v1/lft/player".to_string()}
    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }
}

pub fn post_lol_clash_v1_lft_player(body: LolClashLftState) -> PostLolClashV1LftPlayer {
    PostLolClashV1LftPlayer{body}
}


pub struct PostLolClashV1LftPlayerFind {
    pub body: LolClashFindPlayers,
}

impl IsApiRequest for PostLolClashV1LftPlayerFind {
    const METHOD: Method = Method::POST;
    type ReturnType = Vec<PlayerFinderDto>;
    fn get_url(&self) -> String {"/lol-clash/v1/lft/player/find".to_string()}
    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }
}

pub fn post_lol_clash_v1_lft_player_find(body: LolClashFindPlayers) -> PostLolClashV1LftPlayerFind {
    PostLolClashV1LftPlayerFind{body}
}


pub struct PostLolClashV1LftTeam {
    pub body: LolClashTeamOpenState,
}

impl IsApiRequest for PostLolClashV1LftTeam {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;
    fn get_url(&self) -> String {"/lol-clash/v1/lft/team".to_string()}
    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }
}

pub fn post_lol_clash_v1_lft_team(body: LolClashTeamOpenState) -> PostLolClashV1LftTeam {
    PostLolClashV1LftTeam{body}
}


pub struct PostLolClashV1LftTeamByRosterIdRequest {
    pub roster_id: String,
}

impl IsApiRequest for PostLolClashV1LftTeamByRosterIdRequest {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;
    fn get_url(&self) -> String {format!("/lol-clash/v1/lft/team/{}/request", self.roster_id)}
}

pub fn post_lol_clash_v1_lft_team_by_roster_id_request(roster_id: String) -> PostLolClashV1LftTeamByRosterIdRequest {
    PostLolClashV1LftTeamByRosterIdRequest{roster_id}
}


pub struct PostLolClashV1LftTeamFetchRequests {
    pub body: i64,
}

impl IsApiRequest for PostLolClashV1LftTeamFetchRequests {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;
    fn get_url(&self) -> String {"/lol-clash/v1/lft/team/fetch-requests".to_string()}
    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }
}

pub fn post_lol_clash_v1_lft_team_fetch_requests(body: i64) -> PostLolClashV1LftTeamFetchRequests {
    PostLolClashV1LftTeamFetchRequests{body}
}


pub struct PostLolClashV1LftTeamFind {
    pub body: LolClashFindTeams,
}

impl IsApiRequest for PostLolClashV1LftTeamFind {
    const METHOD: Method = Method::POST;
    type ReturnType = Vec<OpenedTeamDto>;
    fn get_url(&self) -> String {"/lol-clash/v1/lft/team/find".to_string()}
    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }
}

pub fn post_lol_clash_v1_lft_team_find(body: LolClashFindTeams) -> PostLolClashV1LftTeamFind {
    PostLolClashV1LftTeamFind{body}
}


pub struct PostLolClashV1NotificationsAcknowledge {}

impl IsApiRequest for PostLolClashV1NotificationsAcknowledge {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;
    fn get_url(&self) -> String {"/lol-clash/v1/notifications/acknowledge".to_string()}
}

pub fn post_lol_clash_v1_notifications_acknowledge() -> PostLolClashV1NotificationsAcknowledge {
    PostLolClashV1NotificationsAcknowledge{}
}


pub struct PostLolClashV1Refresh {}

impl IsApiRequest for PostLolClashV1Refresh {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;
    fn get_url(&self) -> String {"/lol-clash/v1/refresh".to_string()}
}

pub fn post_lol_clash_v1_refresh() -> PostLolClashV1Refresh {
    PostLolClashV1Refresh{}
}


pub struct PostLolClashV1RosterByRosterIdAccept {
    pub roster_id: String,
}

impl IsApiRequest for PostLolClashV1RosterByRosterIdAccept {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;
    fn get_url(&self) -> String {format!("/lol-clash/v1/roster/{}/accept", self.roster_id)}
}

pub fn post_lol_clash_v1_roster_by_roster_id_accept(roster_id: String) -> PostLolClashV1RosterByRosterIdAccept {
    PostLolClashV1RosterByRosterIdAccept{roster_id}
}


pub struct PostLolClashV1RosterByRosterIdCancelWithdraw {
    pub roster_id: String,
}

impl IsApiRequest for PostLolClashV1RosterByRosterIdCancelWithdraw {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;
    fn get_url(&self) -> String {format!("/lol-clash/v1/roster/{}/cancel-withdraw", self.roster_id)}
}

pub fn post_lol_clash_v1_roster_by_roster_id_cancel_withdraw(roster_id: String) -> PostLolClashV1RosterByRosterIdCancelWithdraw {
    PostLolClashV1RosterByRosterIdCancelWithdraw{roster_id}
}


pub struct PostLolClashV1RosterByRosterIdChangeAllDetails {
    pub roster_id: String,
    pub body: LolClashRosterDetails,
}

impl IsApiRequest for PostLolClashV1RosterByRosterIdChangeAllDetails {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;
    fn get_url(&self) -> String {format!("/lol-clash/v1/roster/{}/change-all-details", self.roster_id)}
    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }
}

pub fn post_lol_clash_v1_roster_by_roster_id_change_all_details(roster_id: String, body: LolClashRosterDetails) -> PostLolClashV1RosterByRosterIdChangeAllDetails {
    PostLolClashV1RosterByRosterIdChangeAllDetails{roster_id, body}
}


pub struct PostLolClashV1RosterByRosterIdChangeIcon {
    pub roster_id: String,
    pub body: LolClashChangeIconRequest,
}

impl IsApiRequest for PostLolClashV1RosterByRosterIdChangeIcon {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;
    fn get_url(&self) -> String {format!("/lol-clash/v1/roster/{}/change-icon", self.roster_id)}
    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }
}

pub fn post_lol_clash_v1_roster_by_roster_id_change_icon(roster_id: String, body: LolClashChangeIconRequest) -> PostLolClashV1RosterByRosterIdChangeIcon {
    PostLolClashV1RosterByRosterIdChangeIcon{roster_id, body}
}


pub struct PostLolClashV1RosterByRosterIdChangeName {
    pub roster_id: String,
    pub body: LolClashChangeNameRequest,
}

impl IsApiRequest for PostLolClashV1RosterByRosterIdChangeName {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;
    fn get_url(&self) -> String {format!("/lol-clash/v1/roster/{}/change-name", self.roster_id)}
    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }
}

pub fn post_lol_clash_v1_roster_by_roster_id_change_name(roster_id: String, body: LolClashChangeNameRequest) -> PostLolClashV1RosterByRosterIdChangeName {
    PostLolClashV1RosterByRosterIdChangeName{roster_id, body}
}


pub struct PostLolClashV1RosterByRosterIdChangeShortName {
    pub roster_id: String,
    pub body: LolClashChangeNameRequest,
}

impl IsApiRequest for PostLolClashV1RosterByRosterIdChangeShortName {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;
    fn get_url(&self) -> String {format!("/lol-clash/v1/roster/{}/change-short-name", self.roster_id)}
    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }
}

pub fn post_lol_clash_v1_roster_by_roster_id_change_short_name(roster_id: String, body: LolClashChangeNameRequest) -> PostLolClashV1RosterByRosterIdChangeShortName {
    PostLolClashV1RosterByRosterIdChangeShortName{roster_id, body}
}


pub struct PostLolClashV1RosterByRosterIdDecline {
    pub roster_id: String,
}

impl IsApiRequest for PostLolClashV1RosterByRosterIdDecline {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;
    fn get_url(&self) -> String {format!("/lol-clash/v1/roster/{}/decline", self.roster_id)}
}

pub fn post_lol_clash_v1_roster_by_roster_id_decline(roster_id: String) -> PostLolClashV1RosterByRosterIdDecline {
    PostLolClashV1RosterByRosterIdDecline{roster_id}
}


pub struct PostLolClashV1RosterByRosterIdDisband {
    pub roster_id: String,
}

impl IsApiRequest for PostLolClashV1RosterByRosterIdDisband {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;
    fn get_url(&self) -> String {format!("/lol-clash/v1/roster/{}/disband", self.roster_id)}
}

pub fn post_lol_clash_v1_roster_by_roster_id_disband(roster_id: String) -> PostLolClashV1RosterByRosterIdDisband {
    PostLolClashV1RosterByRosterIdDisband{roster_id}
}


pub struct PostLolClashV1RosterByRosterIdInvite {
    pub roster_id: String,
    pub body: Vec<u64>,
}

impl IsApiRequest for PostLolClashV1RosterByRosterIdInvite {
    const METHOD: Method = Method::POST;
    type ReturnType = Vec<LolClashClientFailedInvite>;
    fn get_url(&self) -> String {format!("/lol-clash/v1/roster/{}/invite", self.roster_id)}
    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }
}

pub fn post_lol_clash_v1_roster_by_roster_id_invite(roster_id: String, body: Vec<u64>) -> PostLolClashV1RosterByRosterIdInvite {
    PostLolClashV1RosterByRosterIdInvite{roster_id, body}
}


pub struct PostLolClashV1RosterByRosterIdKick {
    pub roster_id: String,
    pub body: LolClashKickRequest,
}

impl IsApiRequest for PostLolClashV1RosterByRosterIdKick {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;
    fn get_url(&self) -> String {format!("/lol-clash/v1/roster/{}/kick", self.roster_id)}
    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }
}

pub fn post_lol_clash_v1_roster_by_roster_id_kick(roster_id: String, body: LolClashKickRequest) -> PostLolClashV1RosterByRosterIdKick {
    PostLolClashV1RosterByRosterIdKick{roster_id, body}
}


pub struct PostLolClashV1RosterByRosterIdLeave {
    pub roster_id: String,
}

impl IsApiRequest for PostLolClashV1RosterByRosterIdLeave {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;
    fn get_url(&self) -> String {format!("/lol-clash/v1/roster/{}/leave", self.roster_id)}
}

pub fn post_lol_clash_v1_roster_by_roster_id_leave(roster_id: String) -> PostLolClashV1RosterByRosterIdLeave {
    PostLolClashV1RosterByRosterIdLeave{roster_id}
}


pub struct PostLolClashV1RosterByRosterIdLockin {
    pub roster_id: String,
}

impl IsApiRequest for PostLolClashV1RosterByRosterIdLockin {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;
    fn get_url(&self) -> String {format!("/lol-clash/v1/roster/{}/lockin", self.roster_id)}
}

pub fn post_lol_clash_v1_roster_by_roster_id_lockin(roster_id: String) -> PostLolClashV1RosterByRosterIdLockin {
    PostLolClashV1RosterByRosterIdLockin{roster_id}
}


pub struct PostLolClashV1RosterByRosterIdSetPosition {
    pub roster_id: String,
    pub body: LolClashSetPositionRequest,
}

impl IsApiRequest for PostLolClashV1RosterByRosterIdSetPosition {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;
    fn get_url(&self) -> String {format!("/lol-clash/v1/roster/{}/set-position", self.roster_id)}
    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }
}

pub fn post_lol_clash_v1_roster_by_roster_id_set_position(roster_id: String, body: LolClashSetPositionRequest) -> PostLolClashV1RosterByRosterIdSetPosition {
    PostLolClashV1RosterByRosterIdSetPosition{roster_id, body}
}


pub struct PostLolClashV1RosterByRosterIdSetTicket {
    pub roster_id: String,
    pub body: LolClashSetTicketRequest,
}

impl IsApiRequest for PostLolClashV1RosterByRosterIdSetTicket {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;
    fn get_url(&self) -> String {format!("/lol-clash/v1/roster/{}/set-ticket", self.roster_id)}
    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }
}

pub fn post_lol_clash_v1_roster_by_roster_id_set_ticket(roster_id: String, body: LolClashSetTicketRequest) -> PostLolClashV1RosterByRosterIdSetTicket {
    PostLolClashV1RosterByRosterIdSetTicket{roster_id, body}
}


pub struct PostLolClashV1RosterByRosterIdSuggest {
    pub roster_id: String,
    pub body: Vec<u64>,
}

impl IsApiRequest for PostLolClashV1RosterByRosterIdSuggest {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;
    fn get_url(&self) -> String {format!("/lol-clash/v1/roster/{}/suggest", self.roster_id)}
    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }
}

pub fn post_lol_clash_v1_roster_by_roster_id_suggest(roster_id: String, body: Vec<u64>) -> PostLolClashV1RosterByRosterIdSuggest {
    PostLolClashV1RosterByRosterIdSuggest{roster_id, body}
}


pub struct PostLolClashV1RosterByRosterIdSuggestBySummonerIdAccept {
    pub roster_id: String,
    pub summoner_id: u64,
}

impl IsApiRequest for PostLolClashV1RosterByRosterIdSuggestBySummonerIdAccept {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;
    fn get_url(&self) -> String {format!("/lol-clash/v1/roster/{}/suggest/{}/accept", self.roster_id, self.summoner_id)}
}

pub fn post_lol_clash_v1_roster_by_roster_id_suggest_by_summoner_id_accept(roster_id: String, summoner_id: u64) -> PostLolClashV1RosterByRosterIdSuggestBySummonerIdAccept {
    PostLolClashV1RosterByRosterIdSuggestBySummonerIdAccept{roster_id, summoner_id}
}


pub struct PostLolClashV1RosterByRosterIdSuggestBySummonerIdDecline {
    pub roster_id: String,
    pub summoner_id: u64,
}

impl IsApiRequest for PostLolClashV1RosterByRosterIdSuggestBySummonerIdDecline {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;
    fn get_url(&self) -> String {format!("/lol-clash/v1/roster/{}/suggest/{}/decline", self.roster_id, self.summoner_id)}
}

pub fn post_lol_clash_v1_roster_by_roster_id_suggest_by_summoner_id_decline(roster_id: String, summoner_id: u64) -> PostLolClashV1RosterByRosterIdSuggestBySummonerIdDecline {
    PostLolClashV1RosterByRosterIdSuggestBySummonerIdDecline{roster_id, summoner_id}
}


pub struct PostLolClashV1RosterByRosterIdSuggestBySummonerIdRevoke {
    pub roster_id: String,
    pub summoner_id: u64,
}

impl IsApiRequest for PostLolClashV1RosterByRosterIdSuggestBySummonerIdRevoke {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;
    fn get_url(&self) -> String {format!("/lol-clash/v1/roster/{}/suggest/{}/revoke", self.roster_id, self.summoner_id)}
}

pub fn post_lol_clash_v1_roster_by_roster_id_suggest_by_summoner_id_revoke(roster_id: String, summoner_id: u64) -> PostLolClashV1RosterByRosterIdSuggestBySummonerIdRevoke {
    PostLolClashV1RosterByRosterIdSuggestBySummonerIdRevoke{roster_id, summoner_id}
}


pub struct PostLolClashV1RosterByRosterIdTicketOfferBySummonerIdAccept {
    pub roster_id: String,
    pub summoner_id: u64,
}

impl IsApiRequest for PostLolClashV1RosterByRosterIdTicketOfferBySummonerIdAccept {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;
    fn get_url(&self) -> String {format!("/lol-clash/v1/roster/{}/ticket-offer/{}/accept", self.roster_id, self.summoner_id)}
}

pub fn post_lol_clash_v1_roster_by_roster_id_ticket_offer_by_summoner_id_accept(roster_id: String, summoner_id: u64) -> PostLolClashV1RosterByRosterIdTicketOfferBySummonerIdAccept {
    PostLolClashV1RosterByRosterIdTicketOfferBySummonerIdAccept{roster_id, summoner_id}
}


pub struct PostLolClashV1RosterByRosterIdTicketOfferBySummonerIdDecline {
    pub roster_id: String,
    pub summoner_id: u64,
}

impl IsApiRequest for PostLolClashV1RosterByRosterIdTicketOfferBySummonerIdDecline {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;
    fn get_url(&self) -> String {format!("/lol-clash/v1/roster/{}/ticket-offer/{}/decline", self.roster_id, self.summoner_id)}
}

pub fn post_lol_clash_v1_roster_by_roster_id_ticket_offer_by_summoner_id_decline(roster_id: String, summoner_id: u64) -> PostLolClashV1RosterByRosterIdTicketOfferBySummonerIdDecline {
    PostLolClashV1RosterByRosterIdTicketOfferBySummonerIdDecline{roster_id, summoner_id}
}


pub struct PostLolClashV1RosterByRosterIdTicketOfferBySummonerIdOffer {
    pub roster_id: String,
    pub summoner_id: u64,
    pub body: LolClashOfferTicketRequest,
}

impl IsApiRequest for PostLolClashV1RosterByRosterIdTicketOfferBySummonerIdOffer {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;
    fn get_url(&self) -> String {format!("/lol-clash/v1/roster/{}/ticket-offer/{}/offer", self.roster_id, self.summoner_id)}
    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }
}

pub fn post_lol_clash_v1_roster_by_roster_id_ticket_offer_by_summoner_id_offer(roster_id: String, summoner_id: u64, body: LolClashOfferTicketRequest) -> PostLolClashV1RosterByRosterIdTicketOfferBySummonerIdOffer {
    PostLolClashV1RosterByRosterIdTicketOfferBySummonerIdOffer{roster_id, summoner_id, body}
}


pub struct PostLolClashV1RosterByRosterIdTicketOfferBySummonerIdRevoke {
    pub roster_id: String,
    pub summoner_id: u64,
}

impl IsApiRequest for PostLolClashV1RosterByRosterIdTicketOfferBySummonerIdRevoke {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;
    fn get_url(&self) -> String {format!("/lol-clash/v1/roster/{}/ticket-offer/{}/revoke", self.roster_id, self.summoner_id)}
}

pub fn post_lol_clash_v1_roster_by_roster_id_ticket_offer_by_summoner_id_revoke(roster_id: String, summoner_id: u64) -> PostLolClashV1RosterByRosterIdTicketOfferBySummonerIdRevoke {
    PostLolClashV1RosterByRosterIdTicketOfferBySummonerIdRevoke{roster_id, summoner_id}
}


pub struct PostLolClashV1RosterByRosterIdTransferCaptain {
    pub roster_id: String,
    pub body: u64,
}

impl IsApiRequest for PostLolClashV1RosterByRosterIdTransferCaptain {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;
    fn get_url(&self) -> String {format!("/lol-clash/v1/roster/{}/transfer-captain", self.roster_id)}
    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }
}

pub fn post_lol_clash_v1_roster_by_roster_id_transfer_captain(roster_id: String, body: u64) -> PostLolClashV1RosterByRosterIdTransferCaptain {
    PostLolClashV1RosterByRosterIdTransferCaptain{roster_id, body}
}


pub struct PostLolClashV1RosterByRosterIdUnlockin {
    pub roster_id: String,
}

impl IsApiRequest for PostLolClashV1RosterByRosterIdUnlockin {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;
    fn get_url(&self) -> String {format!("/lol-clash/v1/roster/{}/unlockin", self.roster_id)}
}

pub fn post_lol_clash_v1_roster_by_roster_id_unlockin(roster_id: String) -> PostLolClashV1RosterByRosterIdUnlockin {
    PostLolClashV1RosterByRosterIdUnlockin{roster_id}
}


pub struct PostLolClashV1RosterByRosterIdUnwithdraw {
    pub roster_id: String,
}

impl IsApiRequest for PostLolClashV1RosterByRosterIdUnwithdraw {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;
    fn get_url(&self) -> String {format!("/lol-clash/v1/roster/{}/unwithdraw", self.roster_id)}
}

pub fn post_lol_clash_v1_roster_by_roster_id_unwithdraw(roster_id: String) -> PostLolClashV1RosterByRosterIdUnwithdraw {
    PostLolClashV1RosterByRosterIdUnwithdraw{roster_id}
}


pub struct PostLolClashV1RosterByRosterIdUpdateLogos {
    pub roster_id: String,
}

impl IsApiRequest for PostLolClashV1RosterByRosterIdUpdateLogos {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;
    fn get_url(&self) -> String {format!("/lol-clash/v1/roster/{}/update-logos", self.roster_id)}
}

pub fn post_lol_clash_v1_roster_by_roster_id_update_logos(roster_id: String) -> PostLolClashV1RosterByRosterIdUpdateLogos {
    PostLolClashV1RosterByRosterIdUpdateLogos{roster_id}
}


pub struct PostLolClashV1RosterByRosterIdWithdraw {
    pub roster_id: String,
}

impl IsApiRequest for PostLolClashV1RosterByRosterIdWithdraw {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;
    fn get_url(&self) -> String {format!("/lol-clash/v1/roster/{}/withdraw", self.roster_id)}
}

pub fn post_lol_clash_v1_roster_by_roster_id_withdraw(roster_id: String) -> PostLolClashV1RosterByRosterIdWithdraw {
    PostLolClashV1RosterByRosterIdWithdraw{roster_id}
}


pub struct PostLolClashV1SimpleStateFlagsByIdAcknowledge {
    pub id: String,
}

impl IsApiRequest for PostLolClashV1SimpleStateFlagsByIdAcknowledge {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;
    fn get_url(&self) -> String {format!("/lol-clash/v1/simple-state-flags/{}/acknowledge", self.id)}
}

pub fn post_lol_clash_v1_simple_state_flags_by_id_acknowledge(id: String) -> PostLolClashV1SimpleStateFlagsByIdAcknowledge {
    PostLolClashV1SimpleStateFlagsByIdAcknowledge{id}
}


pub struct PostLolClashV1TournamentByTournamentIdCreateRoster {
    pub tournament_id: i64,
    pub body: LolClashRosterDetails,
}

impl IsApiRequest for PostLolClashV1TournamentByTournamentIdCreateRoster {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;
    fn get_url(&self) -> String {format!("/lol-clash/v1/tournament/{}/create-roster", self.tournament_id)}
    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }
}

pub fn post_lol_clash_v1_tournament_by_tournament_id_create_roster(tournament_id: i64, body: LolClashRosterDetails) -> PostLolClashV1TournamentByTournamentIdCreateRoster {
    PostLolClashV1TournamentByTournamentIdCreateRoster{tournament_id, body}
}


pub struct PostLolClashV1UpdateLogos {}

impl IsApiRequest for PostLolClashV1UpdateLogos {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;
    fn get_url(&self) -> String {"/lol-clash/v1/update-logos".to_string()}
}

pub fn post_lol_clash_v1_update_logos() -> PostLolClashV1UpdateLogos {
    PostLolClashV1UpdateLogos{}
}


pub struct PostLolClashV1Voice {}

impl IsApiRequest for PostLolClashV1Voice {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;
    fn get_url(&self) -> String {"/lol-clash/v1/voice".to_string()}
}

pub fn post_lol_clash_v1_voice() -> PostLolClashV1Voice {
    PostLolClashV1Voice{}
}


pub struct PostLolClashV1VoiceDelayByDelaySeconds {
    pub delay_seconds: f64,
}

impl IsApiRequest for PostLolClashV1VoiceDelayByDelaySeconds {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;
    fn get_url(&self) -> String {format!("/lol-clash/v1/voice-delay/{}", self.delay_seconds)}
}

pub fn post_lol_clash_v1_voice_delay_by_delay_seconds(delay_seconds: f64) -> PostLolClashV1VoiceDelayByDelaySeconds {
    PostLolClashV1VoiceDelayByDelaySeconds{delay_seconds}
}


// OBJECTS

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BracketMatch {
    pub id: i64,
    pub round: i32,
    pub order: i32,
    #[serde(rename = "rosterId1")]
    pub roster_id_1: i64,
    #[serde(rename = "rosterId2")]
    pub roster_id_2: i64,
    #[serde(rename = "resultHistory")]
    pub result_history: String,
    #[serde(rename = "lowestPossiblePosition")]
    pub lowest_possible_position: i32,
    #[serde(rename = "highestPossiblePosition")]
    pub highest_possible_position: i32,
    #[serde(rename = "roundStartTime")]
    pub round_start_time: i64,
    #[serde(rename = "gameStartTime")]
    pub game_start_time: i64,
    pub status: ClientBracketMatchStatus,
    #[serde(rename = "winnerId")]
    pub winner_id: i64,
    #[serde(rename = "gameId")]
    pub game_id: i64,
    #[serde(rename = "loserBracket")]
    pub loser_bracket: bool,
    #[serde(rename = "forfeitRosterId")]
    pub forfeit_roster_id: i64,
    #[serde(rename = "failRosterStatus")]
    pub fail_roster_status: i32,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BracketRoster {
    #[serde(rename = "rosterId")]
    pub roster_id: i64,
    pub name: String,
    #[serde(rename = "shortName")]
    pub short_name: String,
    pub logo: i32,
    #[serde(rename = "logoColor")]
    pub logo_color: i32,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ClashEventData {
    #[serde(rename = "earnedDate")]
    pub earned_date: String,
    #[serde(rename = "rewardType")]
    pub reward_type: String,
    #[serde(rename = "tournamentId")]
    pub tournament_id: i64,
    #[serde(rename = "tournamentName")]
    pub tournament_name: String,
    pub tier: String,
    pub bracket: i64,
    #[serde(rename = "seasonId")]
    pub season_id: i32,
    pub theme: String,
    #[serde(rename = "rosterId")]
    pub roster_id: i64,
    #[serde(rename = "teamName")]
    pub team_name: String,
    #[serde(rename = "teamShortName")]
    pub team_short_name: String,
    #[serde(rename = "teamLogoName")]
    pub team_logo_name: String,
    #[serde(rename = "teamLogoChromaId")]
    pub team_logo_chroma_id: String,
    #[serde(rename = "playerUUIDs")]
    pub player_uui_ds: Vec<String>,
    #[serde(rename = "rewardSpec")]
    pub reward_spec: ClashRewardSpec,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ClashRewardConfigClient {
    pub name: String,
    #[serde(rename = "keyDef")]
    pub key_def: Vec<ClashRewardKeyType>,
    pub entries: Vec<ClashRewardConfigEntry>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ClashRewardConfigEntry {
    pub key: String,
    pub vals: Vec<ClashRewardOutput>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ClashRewardDefinition {
    #[serde(rename = "rewardType")]
    pub reward_type: ClashRewardType,
    #[serde(rename = "rewardSpec")]
    pub reward_spec: ClashRewardSpec,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ClashRewardOutput {
    pub primary: ClashRewardDefinition,
    pub alternative: ClashRewardDefinition,
    pub grant: ClashRewardTime,
    pub show: ClashRewardTime,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ClashRewardSpec {
    pub pedestal: String,
    pub cup: String,
    pub gem: String,
    pub tier: String,
    pub bracket: String,
    pub theme: String,
    pub level: String,
    #[serde(rename = "seasonId")]
    pub season_id: String,
    pub name: String,
    pub quantity: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ClashSeasonRewardResult {
    #[serde(rename = "playerId")]
    pub player_id: u64,
    #[serde(rename = "seasonId")]
    pub season_id: i32,
    #[serde(rename = "seasonVp")]
    pub season_vp: i32,
    pub banned: bool,
    #[serde(rename = "honorLevel")]
    pub honor_level: i32,
    pub eligible: bool,
    pub rewards: Vec<ClashRewardDefinition>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolClashBracket {
    #[serde(rename = "tournamentId")]
    pub tournament_id: i64,
    pub id: i64,
    pub size: i32,
    pub matches: Vec<BracketMatch>,
    pub rosters: Vec<BracketRoster>,
    pub version: i32,
    pub period: i32,
    #[serde(rename = "isComplete")]
    pub is_complete: bool,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolClashChangeIconRequest {
    #[serde(rename = "iconId")]
    pub icon_id: i32,
    #[serde(rename = "iconColorId")]
    pub icon_color_id: i32,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolClashChangeNameRequest {
    pub name: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolClashClashDisabledConfig {
    #[serde(rename = "disabledReason")]
    pub disabled_reason: String,
    #[serde(rename = "estimatedEnableTimeMillis")]
    pub estimated_enable_time_millis: u64,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolClashClientFailedInvite {
    #[serde(rename = "playerId")]
    pub player_id: u64,
    pub exception: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolClashEogPlayerUpdateDto {
    #[serde(rename = "tournamentId")]
    pub tournament_id: i64,
    #[serde(rename = "gameId")]
    pub game_id: i64,
    pub winner: bool,
    #[serde(rename = "themeVp")]
    pub theme_vp: i32,
    #[serde(rename = "seasonVp")]
    pub season_vp: i32,
    #[serde(rename = "lowestPosition")]
    pub lowest_position: i32,
    #[serde(rename = "bracketSize")]
    pub bracket_size: i32,
    pub bid: i32,
    pub tier: i32,
    #[serde(rename = "earnedRewards")]
    pub earned_rewards: Vec<ClashRewardDefinition>,
    #[serde(rename = "rewardProgress")]
    pub reward_progress: Vec<ClashRewardDefinition>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolClashFindPlayers {
    #[serde(rename = "invitationId")]
    pub invitation_id: String,
    #[serde(rename = "memberId")]
    pub member_id: i64,
    pub page: i32,
    pub count: i32,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolClashFindTeams {
    #[serde(rename = "tournamentId")]
    pub tournament_id: i64,
    pub page: i32,
    pub count: i32,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolClashKickRequest {
    #[serde(rename = "summonerId")]
    pub summoner_id: u64,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolClashLftState {
    pub lft: bool,
    #[serde(rename = "primaryPos")]
    pub primary_pos: String,
    #[serde(rename = "secondaryPos")]
    pub secondary_pos: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolClashMucJwtDto {
    pub jwt: String,
    #[serde(rename = "channelClaim")]
    pub channel_claim: String,
    pub domain: String,
    #[serde(rename = "targetRegion")]
    pub target_region: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolClashOfferTicketRequest {
    #[serde(rename = "ticketAmount")]
    pub ticket_amount: i32,
    #[serde(rename = "ticketType")]
    pub ticket_type: TicketType,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolClashPlayerChatRoster {
    #[serde(rename = "tournamentId")]
    pub tournament_id: i64,
    #[serde(rename = "startTimeMs")]
    pub start_time_ms: i64,
    #[serde(rename = "endTimeMs")]
    pub end_time_ms: i64,
    #[serde(rename = "tournamentState")]
    pub tournament_state: LolClashTournamentState,
    #[serde(rename = "playerState")]
    pub player_state: LolClashPlayerState,
    #[serde(rename = "isRegistered")]
    pub is_registered: bool,
    pub name: String,
    #[serde(rename = "shortName")]
    pub short_name: String,
    #[serde(rename = "iconId")]
    pub icon_id: i32,
    #[serde(rename = "iconColorId")]
    pub icon_color_id: i32,
    #[serde(rename = "logoUrl")]
    pub logo_url: String,
    #[serde(rename = "invitationId")]
    pub invitation_id: String,
    #[serde(rename = "multiUserChatId")]
    pub multi_user_chat_id: String,
    #[serde(rename = "multiUserChatPassword")]
    pub multi_user_chat_password: String,
    #[serde(rename = "mucJwtDto")]
    pub muc_jwt_dto: LolClashMucJwtDto,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolClashPlayerData {
    pub tickets: HashMap<String, i32>,
    #[serde(rename = "isClashBanned")]
    pub is_clash_banned: bool,
    pub tier: i32,
    pub lft: bool,
    #[serde(rename = "primaryPos")]
    pub primary_pos: String,
    #[serde(rename = "secondaryPos")]
    pub secondary_pos: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolClashPlayerNotification {
    pub source: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub id: u64,
    #[serde(rename = "backgroundUrl")]
    pub background_url: String,
    pub data: HashMap<String, String>,
    pub state: String,
    #[serde(rename = "iconUrl")]
    pub icon_url: String,
    #[serde(rename = "titleKey")]
    pub title_key: String,
    #[serde(rename = "detailKey")]
    pub detail_key: String,
    pub created: String,
    pub expires: String,
    pub critical: bool,
    pub dismissible: bool,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolClashPlayerNotificationData {
    #[serde(rename = "notifyReason")]
    pub notify_reason: LolClashNotifyReason,
    #[serde(rename = "rosterNotifyReason")]
    pub roster_notify_reason: LolClashRosterNotifyReason,
    #[serde(rename = "tournamentNotifyReason")]
    pub tournament_notify_reason: LolClashTournamentNotifyReason,
    #[serde(rename = "sourceSummonerId")]
    pub source_summoner_id: u64,
    #[serde(rename = "targetSummonerId")]
    pub target_summoner_id: u64,
    pub notification: LolClashPlayerNotification,
    #[serde(rename = "keySuffix")]
    pub key_suffix: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolClashPlayerRewards {
    #[serde(rename = "seasonVp")]
    pub season_vp: i32,
    #[serde(rename = "themeVp")]
    pub theme_vp: Vec<LolClashThemeVp>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolClashPlayerTournamentData {
    pub state: LolClashPlayerState,
    #[serde(rename = "rosterId")]
    pub roster_id: String,
    #[serde(rename = "bracketId")]
    pub bracket_id: i64,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolClashPlaymodeRestrictedInfo {
    #[serde(rename = "isRestricted")]
    pub is_restricted: bool,
    #[serde(rename = "tournamentId")]
    pub tournament_id: i64,
    #[serde(rename = "presenceState")]
    pub presence_state: LolClashPresenceState,
    #[serde(rename = "rosterId")]
    pub roster_id: String,
    #[serde(rename = "phaseId")]
    pub phase_id: i64,
    #[serde(rename = "readyForVoice")]
    pub ready_for_voice: bool,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolClashRoster {
    #[serde(rename = "tournamentId")]
    pub tournament_id: i64,
    #[serde(rename = "invitationId")]
    pub invitation_id: String,
    pub id: String,
    pub name: String,
    #[serde(rename = "shortName")]
    pub short_name: String,
    #[serde(rename = "iconId")]
    pub icon_id: i32,
    #[serde(rename = "iconColorId")]
    pub icon_color_id: i32,
    #[serde(rename = "captainSummonerId")]
    pub captain_summoner_id: u64,
    pub tier: i32,
    pub points: i32,
    pub wins: i32,
    pub losses: i32,
    #[serde(rename = "currentBracketWins")]
    pub current_bracket_wins: i32,
    #[serde(rename = "numCompletedPeriods")]
    pub num_completed_periods: i32,
    #[serde(rename = "isEliminated")]
    pub is_eliminated: bool,
    #[serde(rename = "isRegistered")]
    pub is_registered: bool,
    #[serde(rename = "isActiveInCurrentPhase")]
    pub is_active_in_current_phase: bool,
    #[serde(rename = "isCurrentBracketComplete")]
    pub is_current_bracket_complete: bool,
    #[serde(rename = "highTierVariance")]
    pub high_tier_variance: bool,
    pub members: Vec<LolClashRosterMember>,
    #[serde(rename = "availableLogos")]
    pub available_logos: Vec<RewardLogo>,
    #[serde(rename = "suggestedInvites")]
    pub suggested_invites: Vec<LolClashSuggestedInvite>,
    #[serde(rename = "phaseInfos")]
    pub phase_infos: Vec<LolClashRosterPhaseInfo>,
    pub withdraw: Option<RosterWithdraw>,
    #[serde(rename = "isClashBanned")]
    pub is_clash_banned: bool,
    pub lft: bool,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolClashRosterDetails {
    pub name: String,
    #[serde(rename = "shortName")]
    pub short_name: String,
    #[serde(rename = "iconId")]
    pub icon_id: i32,
    #[serde(rename = "iconColorId")]
    pub icon_color_id: i32,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolClashRosterMatchAggregatedStats {
    pub round: i32,
    #[serde(rename = "durationMs")]
    pub duration_ms: i64,
    #[serde(rename = "opponentShortName")]
    pub opponent_short_name: String,
    #[serde(rename = "opponentIconId")]
    pub opponent_icon_id: i32,
    #[serde(rename = "opponentIconColorId")]
    pub opponent_icon_color_id: i32,
    pub win: bool,
    #[serde(rename = "loserBracket")]
    pub loser_bracket: bool,
    #[serde(rename = "gameId")]
    pub game_id: i64,
    pub kills: i32,
    #[serde(rename = "opponentKills")]
    pub opponent_kills: i32,
    #[serde(rename = "playerChampionIds")]
    pub player_champion_ids: HashMap<String, i32>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolClashRosterMember {
    #[serde(rename = "summonerId")]
    pub summoner_id: u64,
    pub state: LolClashRosterMemberState,
    #[serde(rename = "currentBuyin")]
    pub current_buyin: i32,
    #[serde(rename = "buyinType")]
    pub buyin_type: TicketType,
    #[serde(rename = "previousBuyin")]
    pub previous_buyin: i32,
    #[serde(rename = "incomingOffers")]
    pub incoming_offers: Vec<LolClashTicketOffer>,
    pub position: Position,
    #[serde(rename = "replacedSummonerId")]
    pub replaced_summoner_id: u64,
    pub tier: i32,
    #[serde(rename = "inviteType")]
    pub invite_type: InviteType,
    #[serde(rename = "inviterId")]
    pub inviter_id: u64,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolClashRosterPeriodAggregatedStats {
    pub period: i32,
    #[serde(rename = "bracketSize")]
    pub bracket_size: i32,
    pub time: i64,
    #[serde(rename = "matchStats")]
    pub match_stats: Vec<LolClashRosterMatchAggregatedStats>,
    #[serde(rename = "playerBids")]
    pub player_bids: HashMap<String, i32>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolClashRosterPhaseInfo {
    #[serde(rename = "phaseId")]
    pub phase_id: i64,
    pub period: i32,
    #[serde(rename = "checkinTime")]
    pub checkin_time: i64,
    #[serde(rename = "isBracketComplete")]
    pub is_bracket_complete: bool,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolClashRosterPlayerAggregatedStats {
    #[serde(rename = "rawStatsSum")]
    pub raw_stats_sum: HashMap<String, i32>,
    #[serde(rename = "rawStatsMax")]
    pub raw_stats_max: HashMap<String, i32>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolClashRosterStats {
    #[serde(rename = "rosterId")]
    pub roster_id: i64,
    #[serde(rename = "tournamentThemeId")]
    pub tournament_theme_id: i32,
    #[serde(rename = "tournamentNameLocKey")]
    pub tournament_name_loc_key: String,
    #[serde(rename = "tournamentNameLocKeySecondary")]
    pub tournament_name_loc_key_secondary: String,
    #[serde(rename = "startTimeMs")]
    pub start_time_ms: i64,
    #[serde(rename = "endTimeMs")]
    pub end_time_ms: i64,
    #[serde(rename = "tournamentPeriods")]
    pub tournament_periods: i32,
    pub tier: i32,
    #[serde(rename = "rosterName")]
    pub roster_name: String,
    #[serde(rename = "rosterShortName")]
    pub roster_short_name: String,
    #[serde(rename = "rosterIconId")]
    pub roster_icon_id: i32,
    #[serde(rename = "rosterIconColorId")]
    pub roster_icon_color_id: i32,
    #[serde(rename = "periodStats")]
    pub period_stats: Vec<LolClashRosterPeriodAggregatedStats>,
    #[serde(rename = "playerStats")]
    pub player_stats: HashMap<String, LolClashRosterPlayerAggregatedStats>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolClashScoutingChampionMastery {
    #[serde(rename = "championId")]
    pub champion_id: i32,
    #[serde(rename = "championLevel")]
    pub champion_level: i32,
    #[serde(rename = "championPoints")]
    pub champion_points: i32,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolClashScoutingChampions {
    #[serde(rename = "playerId")]
    pub player_id: u64,
    #[serde(rename = "totalMasteryScore")]
    pub total_mastery_score: u64,
    #[serde(rename = "topMasteries")]
    pub top_masteries: Vec<LolClashScoutingChampionMastery>,
    #[serde(rename = "topSeasonChampions")]
    pub top_season_champions: Vec<LolClashScoutingSeasonChampion>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolClashScoutingSeasonChampion {
    #[serde(rename = "championId")]
    pub champion_id: i32,
    #[serde(rename = "winCount")]
    pub win_count: i32,
    #[serde(rename = "gameCount")]
    pub game_count: i32,
    #[serde(rename = "winRate")]
    pub win_rate: i32,
    pub kda: String,
    #[serde(rename = "kdaClassification")]
    pub kda_classification: LolClashKdaClassification,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolClashSetPositionRequest {
    pub position: Position,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolClashSetTicketRequest {
    #[serde(rename = "ticketAmount")]
    pub ticket_amount: i32,
    #[serde(rename = "ticketType")]
    pub ticket_type: TicketType,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolClashSimpleStateFlag {
    pub id: String,
    pub status: LolClashSimpleStateStatus,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolClashSuggestedInvite {
    #[serde(rename = "summonerId")]
    pub summoner_id: u64,
    #[serde(rename = "suggesterSummonerId")]
    pub suggester_summoner_id: u64,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolClashTeamOpenState {
    #[serde(rename = "invitationId")]
    pub invitation_id: String,
    #[serde(rename = "captainId")]
    pub captain_id: i64,
    #[serde(rename = "openTeam")]
    pub open_team: bool,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolClashThemeVp {
    #[serde(rename = "themeId")]
    pub theme_id: i32,
    pub vp: i32,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolClashThirdPartyApiPlayer {
    #[serde(rename = "summonerId")]
    pub summoner_id: u64,
    pub role: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolClashThirdPartyApiRoster {
    pub captain: LolClashThirdPartyApiPlayer,
    pub members: Vec<LolClashThirdPartyApiPlayer>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolClashTicketOffer {
    #[serde(rename = "summonerId")]
    pub summoner_id: u64,
    pub amount: i32,
    #[serde(rename = "ticketType")]
    pub ticket_type: TicketType,
    #[serde(rename = "isAccepted")]
    pub is_accepted: bool,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolClashTournament {
    pub id: i64,
    #[serde(rename = "themeId")]
    pub theme_id: i32,
    #[serde(rename = "nameLocKey")]
    pub name_loc_key: String,
    #[serde(rename = "nameLocKeySecondary")]
    pub name_loc_key_secondary: String,
    #[serde(rename = "buyInOptions")]
    pub buy_in_options: Vec<i32>,
    #[serde(rename = "buyInOptionsPremium")]
    pub buy_in_options_premium: Vec<i32>,
    #[serde(rename = "entryFee")]
    pub entry_fee: i32,
    #[serde(rename = "rosterSize")]
    pub roster_size: i32,
    #[serde(rename = "allowRosterCreation")]
    pub allow_roster_creation: bool,
    #[serde(rename = "rosterCreateDeadline")]
    pub roster_create_deadline: i64,
    #[serde(rename = "scoutingDurationMs")]
    pub scouting_duration_ms: i64,
    #[serde(rename = "startTimeMs")]
    pub start_time_ms: i64,
    #[serde(rename = "endTimeMs")]
    pub end_time_ms: i64,
    #[serde(rename = "lastThemeOfSeason")]
    pub last_theme_of_season: bool,
    #[serde(rename = "bracketSize")]
    pub bracket_size: String,
    #[serde(rename = "queueId")]
    pub queue_id: i32,
    #[serde(rename = "isSmsRestrictionEnabled")]
    pub is_sms_restriction_enabled: bool,
    #[serde(rename = "isHonorRestrictionEnabled")]
    pub is_honor_restriction_enabled: bool,
    #[serde(rename = "isRankedRestrictionEnabled")]
    pub is_ranked_restriction_enabled: bool,
    pub phases: Vec<LolClashTournamentPhase>,
    #[serde(rename = "rewardConfig")]
    pub reward_config: Vec<ClashRewardConfigClient>,
    #[serde(rename = "tierConfigs")]
    pub tier_configs: Vec<TierConfig>,
    #[serde(rename = "bracketFormationInitDelayMs")]
    pub bracket_formation_init_delay_ms: i64,
    #[serde(rename = "bracketFormationIntervalMs")]
    pub bracket_formation_interval_ms: i64,
    pub status: TournamentStatusEnum,
    #[serde(rename = "resumeTime")]
    pub resume_time: i64,
    pub lft: bool,
    #[serde(rename = "maxInvites")]
    pub max_invites: i32,
    #[serde(rename = "maxSuggestionsPerPlayer")]
    pub max_suggestions_per_player: i32,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolClashTournamentGameEnd {
    #[serde(rename = "tournamentId")]
    pub tournament_id: i64,
    #[serde(rename = "tournamentNameLocKey")]
    pub tournament_name_loc_key: String,
    #[serde(rename = "tournamentNameLocKeySecondary")]
    pub tournament_name_loc_key_secondary: String,
    #[serde(rename = "bracketId")]
    pub bracket_id: i64,
    #[serde(rename = "oldBracket")]
    pub old_bracket: Option<LolClashBracket>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolClashTournamentHistoryAndWinners {
    #[serde(rename = "tournamentHistory")]
    pub tournament_history: Vec<LolClashTournament>,
    #[serde(rename = "tournamentWinners")]
    pub tournament_winners: LolClashTournamentWinnerHistory,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolClashTournamentPhase {
    pub id: i64,
    #[serde(rename = "tournamentId")]
    pub tournament_id: i64,
    pub period: i32,
    #[serde(rename = "lockinStartTime")]
    pub lockin_start_time: i64,
    #[serde(rename = "scoutingStartTime")]
    pub scouting_start_time: i64,
    pub cancelled: bool,
    #[serde(rename = "limitTiers")]
    pub limit_tiers: Vec<i32>,
    #[serde(rename = "capacityStatus")]
    pub capacity_status: CapacityEnum,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolClashTournamentStateInfo {
    #[serde(rename = "tournamentId")]
    pub tournament_id: i64,
    pub state: LolClashTournamentState,
    #[serde(rename = "currentPhaseId")]
    pub current_phase_id: i64,
    #[serde(rename = "nextPhaseId")]
    pub next_phase_id: i64,
    #[serde(rename = "nextStateChangeTime")]
    pub next_state_change_time: i64,
    #[serde(rename = "numRemainingPeriods")]
    pub num_remaining_periods: i32,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolClashTournamentSummary {
    pub state: LolClashTournamentState,
    #[serde(rename = "tournamentId")]
    pub tournament_id: i64,
    #[serde(rename = "rosterId")]
    pub roster_id: String,
    #[serde(rename = "bracketId")]
    pub bracket_id: i64,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolClashTournamentWinnerHistory {
    #[serde(rename = "tournamentId")]
    pub tournament_id: i64,
    pub winners: Vec<LolClashTournamentWinnerInfo>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolClashTournamentWinnerInfo {
    #[serde(rename = "rosterId")]
    pub roster_id: i64,
    pub tier: i32,
    #[serde(rename = "shortName")]
    pub short_name: String,
    pub name: String,
    pub logo: i32,
    #[serde(rename = "logoColor")]
    pub logo_color: i32,
    #[serde(rename = "createTime")]
    pub create_time: i64,
    #[serde(rename = "averageWinDuration")]
    pub average_win_duration: i64,
    #[serde(rename = "playerIds")]
    pub player_ids: Vec<u64>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OpenedTeamDto {
    pub name: String,
    #[serde(rename = "shortName")]
    pub short_name: String,
    pub logo: i32,
    #[serde(rename = "logoColor")]
    pub logo_color: i32,
    #[serde(rename = "invitationId")]
    pub invitation_id: String,
    #[serde(rename = "captainId")]
    pub captain_id: u64,
    pub tier: i32,
    pub members: Vec<OpenedTeamMemberDto>,
    pub invitees: Vec<PendingRosterInviteeDto>,
    #[serde(rename = "openPositions")]
    pub open_positions: Vec<Position>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OpenedTeamMemberDto {
    #[serde(rename = "playerId")]
    pub player_id: i64,
    pub position: Position,
    pub tier: i32,
    pub friendship: i32,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PendingOpenedTeamDto {
    #[serde(rename = "invitationId")]
    pub invitation_id: String,
    pub name: String,
    #[serde(rename = "shortName")]
    pub short_name: String,
    pub logo: i32,
    #[serde(rename = "logoColor")]
    pub logo_color: i32,
    pub tier: i32,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PendingRosterInviteeDto {
    #[serde(rename = "inviteeId")]
    pub invitee_id: u64,
    #[serde(rename = "inviteeState")]
    pub invitee_state: PendingRosterInviteeState,
    pub inviter: u64,
    #[serde(rename = "inviteTime")]
    pub invite_time: i64,
    #[serde(rename = "inviteType")]
    pub invite_type: InviteType,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PlayerFinderDto {
    #[serde(rename = "playerId")]
    pub player_id: u64,
    pub tier: i32,
    #[serde(rename = "primaryPos")]
    pub primary_pos: Position,
    #[serde(rename = "secondaryPos")]
    pub secondary_pos: Position,
    #[serde(rename = "type")]
    pub type_: PlayerFinderEnum,
    #[serde(rename = "friendId")]
    pub friend_id: i64,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PlayerTierDto {
    #[serde(rename = "playerId")]
    pub player_id: u64,
    pub tier: i32,
    #[serde(rename = "primaryPos")]
    pub primary_pos: Position,
    #[serde(rename = "secondPos")]
    pub second_pos: Position,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RewardLogo {
    pub logo: i32,
    #[serde(rename = "memberOwnedCount")]
    pub member_owned_count: i32,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RosterWithdraw {
    #[serde(rename = "initVoteTime")]
    pub init_vote_time: i64,
    #[serde(rename = "initVoteMember")]
    pub init_vote_member: u64,
    #[serde(rename = "voteTimeoutMs")]
    pub vote_timeout_ms: i64,
    #[serde(rename = "lockoutTimeMs")]
    pub lockout_time_ms: i64,
    #[serde(rename = "gameStartBufferMs")]
    pub game_start_buffer_ms: i64,
    #[serde(rename = "voteWithdrawMembers")]
    pub vote_withdraw_members: Vec<i64>,
    #[serde(rename = "declineWithdrawMembers")]
    pub decline_withdraw_members: Vec<i64>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TierConfig {
    pub tier: i32,
    #[serde(rename = "delayTime")]
    pub delay_time: i64,
    #[serde(rename = "estimateTime")]
    pub estimate_time: i64,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TournamentDto {
    pub id: i64,
    #[serde(rename = "scheduleTime")]
    pub schedule_time: i64,
    #[serde(rename = "scheduleEndTime")]
    pub schedule_end_time: i64,
    #[serde(rename = "rosterCreateDeadline")]
    pub roster_create_deadline: i64,
    #[serde(rename = "entryFee")]
    pub entry_fee: i32,
    #[serde(rename = "rosterSize")]
    pub roster_size: i32,
    #[serde(rename = "themeId")]
    pub theme_id: i32,
    #[serde(rename = "nameLocKey")]
    pub name_loc_key: String,
    #[serde(rename = "nameLocKeySecondary")]
    pub name_loc_key_secondary: String,
    #[serde(rename = "buyInOptions")]
    pub buy_in_options: Vec<i32>,
    #[serde(rename = "buyInOptionsPremium")]
    pub buy_in_options_premium: Vec<i32>,
    #[serde(rename = "queueId")]
    pub queue_id: i32,
    #[serde(rename = "scoutingTimeMs")]
    pub scouting_time_ms: i64,
    #[serde(rename = "lastThemeOfSeason")]
    pub last_theme_of_season: bool,
    #[serde(rename = "bracketSize")]
    pub bracket_size: String,
    #[serde(rename = "minGames")]
    pub min_games: i32,
    #[serde(rename = "smsRestriction")]
    pub sms_restriction: bool,
    #[serde(rename = "honorRestriction")]
    pub honor_restriction: bool,
    #[serde(rename = "rankRestriction")]
    pub rank_restriction: bool,
    #[serde(rename = "voiceEnabled")]
    pub voice_enabled: bool,
    pub phases: Vec<TournamentPhaseDto>,
    #[serde(rename = "rewardConfig")]
    pub reward_config: Vec<ClashRewardConfigClient>,
    #[serde(rename = "tierConfigs")]
    pub tier_configs: Vec<TierConfig>,
    #[serde(rename = "bracketFormationInitDelayMs")]
    pub bracket_formation_init_delay_ms: i64,
    #[serde(rename = "bracketFormationIntervalMs")]
    pub bracket_formation_interval_ms: i64,
    pub status: TournamentStatusEnum,
    #[serde(rename = "resumeTime")]
    pub resume_time: i64,
    pub lft: bool,
    #[serde(rename = "maxInvites")]
    pub max_invites: i32,
    #[serde(rename = "maxSuggestionsPerPlayer")]
    pub max_suggestions_per_player: i32,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TournamentPhaseDto {
    pub id: i64,
    #[serde(rename = "tournamentId")]
    pub tournament_id: i64,
    #[serde(rename = "registrationTime")]
    pub registration_time: i64,
    #[serde(rename = "startTime")]
    pub start_time: i64,
    pub period: i32,
    pub cancelled: bool,
    #[serde(rename = "limitTiers")]
    pub limit_tiers: Vec<i32>,
    #[serde(rename = "capacityStatus")]
    pub capacity_status: CapacityEnum,
}


// ENUMS

#[derive(Serialize, Deserialize, Clone, PartialEq, Default, Debug)]
pub enum CapacityEnum {
    #[default]
    #[serde(rename = "FULL")]
    Full,
    #[serde(rename = "HIGH")]
    High,
    #[serde(rename = "MEDIUM")]
    Medium,
    #[serde(rename = "LOW")]
    Low,
}


#[derive(Serialize, Deserialize, Clone, PartialEq, Default, Debug)]
pub enum ClashRewardKeyType {
    #[default]
    #[serde(rename = "TOC_STATE")]
    TocState,
    #[serde(rename = "SEASON_FLAG_COUNT")]
    SeasonFlagCount,
    #[serde(rename = "SEASON_VP")]
    SeasonVp,
    #[serde(rename = "THEME_VP")]
    ThemeVp,
    #[serde(rename = "POINTS")]
    Points,
    #[serde(rename = "WINS")]
    Wins,
    #[serde(rename = "TOURNAMENT_WIN_POS")]
    TournamentWinPos,
    #[serde(rename = "LOWEST_POSITION")]
    LowestPosition,
    #[serde(rename = "TICKET_TYPE")]
    TicketType,
    #[serde(rename = "TICKET_COUNT")]
    TicketCount,
    #[serde(rename = "CUP")]
    Cup,
    #[serde(rename = "TIER")]
    Tier,
}


#[derive(Serialize, Deserialize, Clone, PartialEq, Default, Debug)]
pub enum ClashRewardTime {
    #[default]
    #[serde(rename = "EOT")]
    Eot,
    #[serde(rename = "EOB")]
    Eob,
    #[serde(rename = "EOG")]
    Eog,
    #[serde(rename = "NONE")]
    None,
}


#[derive(Serialize, Deserialize, Clone, PartialEq, Default, Debug)]
pub enum ClashRewardType {
    #[default]
    #[serde(rename = "TOC")]
    Toc,
    #[serde(rename = "VP")]
    Vp,
    #[serde(rename = "LOOT")]
    Loot,
    #[serde(rename = "LOGO")]
    Logo,
    #[serde(rename = "FRAME")]
    Frame,
    #[serde(rename = "FLAG")]
    Flag,
    #[serde(rename = "TROPHY")]
    Trophy,
}


#[derive(Serialize, Deserialize, Clone, PartialEq, Default, Debug)]
pub enum ClientBracketMatchStatus {
    #[default]
    #[serde(rename = "COMPLETED")]
    Completed,
    #[serde(rename = "STARTED")]
    Started,
    #[serde(rename = "UPCOMING")]
    Upcoming,
}


#[derive(Serialize, Deserialize, Clone, PartialEq, Default, Debug)]
pub enum InviteType {
    #[default]
    #[serde(rename = "NONE")]
    None,
    #[serde(rename = "SELFJOIN")]
    Selfjoin,
    #[serde(rename = "SUGGEST")]
    Suggest,
    #[serde(rename = "FRIEND")]
    Friend,
    #[serde(rename = "FREEAGENT")]
    Freeagent,
}


#[derive(Serialize, Deserialize, Clone, PartialEq, Default, Debug)]
pub enum LolClashKdaClassification {
    #[default]
    #[serde(rename = "HIGH")]
    High,
    #[serde(rename = "AVERAGE")]
    Average,
    #[serde(rename = "LOW")]
    Low,
}


#[derive(Serialize, Deserialize, Clone, PartialEq, Default, Debug)]
pub enum LolClashNotifyReason {
    #[default]
    #[serde(rename = "TEAMMATE_UNBAN")]
    TeammateUnban,
    #[serde(rename = "TEAMMATE_BAN")]
    TeammateBan,
    #[serde(rename = "MEMBER_BAN")]
    MemberBan,
    #[serde(rename = "UNBAN")]
    Unban,
    #[serde(rename = "BAN")]
    Ban,
    #[serde(rename = "REVERTED_REGISTRATION")]
    RevertedRegistration,
    #[serde(rename = "REWARD_GRANT_RETRY")]
    RewardGrantRetry,
    #[serde(rename = "REWARD_GRANT_FAILED")]
    RewardGrantFailed,
    #[serde(rename = "ACCEPT_TICKET")]
    AcceptTicket,
    #[serde(rename = "DECLINE_TICKET")]
    DeclineTicket,
    #[serde(rename = "REVOKED_TICKET")]
    RevokedTicket,
    #[serde(rename = "OFFER_TICKET")]
    OfferTicket,
    #[serde(rename = "SET_TICKET")]
    SetTicket,
    #[serde(rename = "KICK")]
    Kick,
    #[serde(rename = "CAPTAIN_LEAVE")]
    CaptainLeave,
    #[serde(rename = "LEAVE")]
    Leave,
    #[serde(rename = "REVOKE_INVITE")]
    RevokeInvite,
    #[serde(rename = "ACCEPT_INVITE")]
    AcceptInvite,
    #[serde(rename = "DECLINE_INVITE")]
    DeclineInvite,
    #[serde(rename = "RESENT_INVITE")]
    ResentInvite,
    #[serde(rename = "INVITE")]
    Invite,
    #[serde(rename = "CHANGE_LFT")]
    ChangeLft,
    #[serde(rename = "CHANGE_NAMETAGLOGO")]
    ChangeNametaglogo,
    #[serde(rename = "CHANGE_POSITION")]
    ChangePosition,
    #[serde(rename = "CHANGE_SHORTNAME")]
    ChangeShortname,
    #[serde(rename = "CHANGE_NAME")]
    ChangeName,
    #[serde(rename = "CHANGE_LOGO")]
    ChangeLogo,
    #[serde(rename = "OWNER_TRANSFER")]
    OwnerTransfer,
    #[serde(rename = "ROSTER_DELETE")]
    RosterDelete,
    #[serde(rename = "DISMISS")]
    Dismiss,
    #[serde(rename = "OWNER_CLOSE")]
    OwnerClose,
    #[serde(rename = "UNREADY")]
    Unready,
    #[serde(rename = "READY")]
    Ready,
    #[serde(rename = "SELFJOIN")]
    Selfjoin,
    #[serde(rename = "REVOKE_SELFJOIN")]
    RevokeSelfjoin,
    #[serde(rename = "ACCEPT_SELFJOIN")]
    AcceptSelfjoin,
    #[serde(rename = "DECLINE_SELFJOIN")]
    DeclineSelfjoin,
    #[serde(rename = "REVOKE_SUGGESTION")]
    RevokeSuggestion,
    #[serde(rename = "ACCEPT_SUGGESTION")]
    AcceptSuggestion,
    #[serde(rename = "DECLINE_SUGGESTION")]
    DeclineSuggestion,
    #[serde(rename = "SUGGESTION")]
    Suggestion,
}


#[derive(Serialize, Deserialize, Clone, PartialEq, Default, Debug)]
pub enum LolClashPlayerState {
    #[default]
    #[serde(rename = "ELIMINATED")]
    Eliminated,
    #[serde(rename = "BRACKET_ROSTER")]
    BracketRoster,
    #[serde(rename = "REGISTERED_ROSTER")]
    RegisteredRoster,
    #[serde(rename = "PENDING_ROSTER")]
    PendingRoster,
    #[serde(rename = "NO_ROSTER")]
    NoRoster,
}


#[derive(Serialize, Deserialize, Clone, PartialEq, Default, Debug)]
pub enum LolClashPresenceState {
    #[default]
    #[serde(rename = "SCOUTING")]
    Scouting,
    #[serde(rename = "LOCKED_IN")]
    LockedIn,
    #[serde(rename = "NONE")]
    None,
}


#[derive(Serialize, Deserialize, Clone, PartialEq, Default, Debug)]
pub enum LolClashRosterMemberState {
    #[default]
    #[serde(rename = "READY")]
    Ready,
    #[serde(rename = "FORCED_NOT_READY")]
    ForcedNotReady,
    #[serde(rename = "NOT_READY")]
    NotReady,
    #[serde(rename = "PENDING")]
    Pending,
    #[serde(rename = "DECLINED")]
    Declined,
}


#[derive(Serialize, Deserialize, Clone, PartialEq, Default, Debug)]
pub enum LolClashRosterNotifyReason {
    #[default]
    #[serde(rename = "GAME_RESCHEDULED")]
    GameRescheduled,
    #[serde(rename = "GAME_START_FAILED_OPPONENT")]
    GameStartFailedOpponent,
    #[serde(rename = "GAME_START_FAILED_SUMMONERS")]
    GameStartFailedSummoners,
    #[serde(rename = "GAME_START_FAILED")]
    GameStartFailed,
    #[serde(rename = "GAME_START_RETRY_OPPONENT")]
    GameStartRetryOpponent,
    #[serde(rename = "GAME_START_RETRY_SUMMONERS")]
    GameStartRetrySummoners,
    #[serde(rename = "GAME_START_RETRY")]
    GameStartRetry,
    #[serde(rename = "TICKET_COULD_NOT_BE_CHARGED")]
    TicketCouldNotBeCharged,
    #[serde(rename = "TICKET_REFUNDED")]
    TicketRefunded,
    #[serde(rename = "TICKET_CHARGED")]
    TicketCharged,
    #[serde(rename = "BANNED_SMURF_OPPONENT")]
    BannedSmurfOpponent,
    #[serde(rename = "BANNED_SMURF_TEAMMATE")]
    BannedSmurfTeammate,
    #[serde(rename = "BANNED_SMURF")]
    BannedSmurf,
    #[serde(rename = "CANNOT_FIND_MATCH")]
    CannotFindMatch,
    #[serde(rename = "BRACKET_ROSTER_REPLACED")]
    BracketRosterReplaced,
    #[serde(rename = "BRACKET_ROSTER_REMOVED")]
    BracketRosterRemoved,
    #[serde(rename = "TIER_CHANGED")]
    TierChanged,
    #[serde(rename = "NO_SHOW_PING")]
    NoShowPing,
    #[serde(rename = "ROUND_COMPLETE")]
    RoundComplete,
    #[serde(rename = "WITHDRAW")]
    Withdraw,
    #[serde(rename = "VOTE_WITHDRAW_DISMISS")]
    VoteWithdrawDismiss,
    #[serde(rename = "VOTE_WITHDRAW_UPDATE")]
    VoteWithdrawUpdate,
    #[serde(rename = "OWNER_TRANSFER")]
    OwnerTransfer,
    #[serde(rename = "QUEUE_DODGE")]
    QueueDodge,
    #[serde(rename = "GAME_END_ERROR")]
    GameEndError,
    #[serde(rename = "GAME_STARTED_ERROR")]
    GameStartedError,
    #[serde(rename = "GAME_STARTED")]
    GameStarted,
    #[serde(rename = "GAME_SCHEDULED")]
    GameScheduled,
    #[serde(rename = "GAME_COMPLETED")]
    GameCompleted,
    #[serde(rename = "PERIOD_SPLIT")]
    PeriodSplit,
    #[serde(rename = "PERIOD_CANCEL")]
    PeriodCancel,
    #[serde(rename = "PHASE_BACKOUT")]
    PhaseBackout,
    #[serde(rename = "PHASE_CHECKIN")]
    PhaseCheckin,
    #[serde(rename = "PHASE_READY")]
    PhaseReady,
    #[serde(rename = "PHASE_UNREADY")]
    PhaseUnready,
    #[serde(rename = "RESTRICTION_AUTO_WIN")]
    RestrictionAutoWin,
    #[serde(rename = "REGISTERED")]
    Registered,
    #[serde(rename = "EOG_PLAYER_UPDATE")]
    EogPlayerUpdate,
    #[serde(rename = "CHEATER_DETECT")]
    CheaterDetect,
    #[serde(rename = "CHANGE_POSITION")]
    ChangePosition,
    #[serde(rename = "BRACKET_READY")]
    BracketReady,
    #[serde(rename = "BYE_AUTO_WIN")]
    ByeAutoWin,
    #[serde(rename = "ROSTER_REVOKED_TICKET")]
    RosterRevokedTicket,
    #[serde(rename = "ROSTER_DECLINE_TICKET")]
    RosterDeclineTicket,
    #[serde(rename = "ROSTER_ACCEPT_TICKET")]
    RosterAcceptTicket,
    #[serde(rename = "ROSTER_OFFER_TICKET")]
    RosterOfferTicket,
    #[serde(rename = "ROSTER_SET_TICKET")]
    RosterSetTicket,
}


#[derive(Serialize, Deserialize, Clone, PartialEq, Default, Debug)]
pub enum LolClashSimpleStateStatus {
    #[default]
    #[serde(rename = "ACKNOWLEDGED")]
    Acknowledged,
    #[serde(rename = "UNACKNOWLEDGED")]
    Unacknowledged,
}


#[derive(Serialize, Deserialize, Clone, PartialEq, Default, Debug)]
pub enum LolClashTournamentNotifyReason {
    #[default]
    #[serde(rename = "UPDATE_STATUS")]
    UpdateStatus,
    #[serde(rename = "REVERT_PHASE")]
    RevertPhase,
    #[serde(rename = "UPDATE_PHASE")]
    UpdatePhase,
    #[serde(rename = "ADD_PHASE")]
    AddPhase,
    #[serde(rename = "CANCEL_PERIOD")]
    CancelPeriod,
    #[serde(rename = "CANCEL_TOURNAMENT")]
    CancelTournament,
    #[serde(rename = "UPDATE_TOURNAMENT")]
    UpdateTournament,
    #[serde(rename = "NEW_TOURNAMENT")]
    NewTournament,
}


#[derive(Serialize, Deserialize, Clone, PartialEq, Default, Debug)]
pub enum LolClashTournamentState {
    #[default]
    #[serde(rename = "RESULTS")]
    Results,
    #[serde(rename = "IN_GAME")]
    InGame,
    #[serde(rename = "SCOUTING")]
    Scouting,
    #[serde(rename = "LOCK_IN")]
    LockIn,
    #[serde(rename = "IDLE")]
    Idle,
    #[serde(rename = "UPCOMING")]
    Upcoming,
}


#[derive(Serialize, Deserialize, Clone, PartialEq, Default, Debug)]
pub enum PendingRosterInviteeState {
    #[default]
    #[serde(rename = "SELFJOIN_REVOKED")]
    SelfjoinRevoked,
    #[serde(rename = "SELFJOIN_DECLINED")]
    SelfjoinDeclined,
    #[serde(rename = "SELFJOIN")]
    Selfjoin,
    #[serde(rename = "ACCEPTED")]
    Accepted,
    #[serde(rename = "SUGGEST_REVOKED")]
    SuggestRevoked,
    #[serde(rename = "SUGGEST_DECLINED")]
    SuggestDeclined,
    #[serde(rename = "REVOKED")]
    Revoked,
    #[serde(rename = "DECLINED")]
    Declined,
    #[serde(rename = "PENDING")]
    Pending,
    #[serde(rename = "SUGGESTED")]
    Suggested,
}


#[derive(Serialize, Deserialize, Clone, PartialEq, Default, Debug)]
pub enum PlayerFinderEnum {
    #[default]
    #[serde(rename = "FRIEND")]
    Friend,
    #[serde(rename = "FREEAGENT")]
    Freeagent,
}


#[derive(Serialize, Deserialize, Clone, PartialEq, Default, Debug)]
pub enum Position {
    #[default]
    #[serde(rename = "UNSELECTED")]
    Unselected,
    #[serde(rename = "FILL")]
    Fill,
    #[serde(rename = "UTILITY")]
    Utility,
    #[serde(rename = "JUNGLE")]
    Jungle,
    #[serde(rename = "BOTTOM")]
    Bottom,
    #[serde(rename = "MIDDLE")]
    Middle,
    #[serde(rename = "TOP")]
    Top,
}


#[derive(Serialize, Deserialize, Clone, PartialEq, Default, Debug)]
pub enum TicketType {
    #[default]
    #[serde(rename = "PREMIUM")]
    Premium,
    #[serde(rename = "BASIC")]
    Basic,
}


#[derive(Serialize, Deserialize, Clone, PartialEq, Default, Debug)]
pub enum TournamentStatusEnum {
    #[default]
    #[serde(rename = "PRERESUME")]
    Preresume,
    #[serde(rename = "PAUSED")]
    Paused,
    #[serde(rename = "CANCELLED")]
    Cancelled,
    #[serde(rename = "DEFAULT")]
    Default,
}

