
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use serde_json::{json, Value, to_value};
use std::collections::hash_map::Values;
use reqwest::Method;
use common::IsApiRequest;

mod additional;

// ENDPOINTS

pub struct DeleteLolLobbyV1Clash {}

impl IsApiRequest for DeleteLolLobbyV1Clash {
    const METHOD: Method = Method::DELETE;
    type ReturnType = Value;
    fn get_url(&self) -> String {"/lol-lobby/v1/clash".to_string()}
}

pub fn delete_lol_lobby_v1_clash() -> DeleteLolLobbyV1Clash {
    DeleteLolLobbyV1Clash{}
}


pub struct DeleteLolLobbyV1LobbyCustomBotsBySummonerInternalNameByBotUuidToDeleteByTeamId {
    pub summoner_internal_name: String,
    pub bot_uuid_to_delete: String,
    pub team_id: String,
}

impl IsApiRequest for DeleteLolLobbyV1LobbyCustomBotsBySummonerInternalNameByBotUuidToDeleteByTeamId {
    const METHOD: Method = Method::DELETE;
    type ReturnType = Value;
    fn get_url(&self) -> String {format!("/lol-lobby/v1/lobby/custom/bots/{}/{}/{}", self.summoner_internal_name, self.bot_uuid_to_delete, self.team_id)}
}

pub fn delete_lol_lobby_v1_lobby_custom_bots_by_summoner_internal_name_by_bot_uuid_to_delete_by_team_id(summoner_internal_name: String, bot_uuid_to_delete: String, team_id: String) -> DeleteLolLobbyV1LobbyCustomBotsBySummonerInternalNameByBotUuidToDeleteByTeamId {
    DeleteLolLobbyV1LobbyCustomBotsBySummonerInternalNameByBotUuidToDeleteByTeamId{summoner_internal_name, bot_uuid_to_delete, team_id}
}


pub struct DeleteLolLobbyV2Lobby {}

impl IsApiRequest for DeleteLolLobbyV2Lobby {
    const METHOD: Method = Method::DELETE;
    type ReturnType = Value;
    fn get_url(&self) -> String {"/lol-lobby/v2/lobby".to_string()}
}

pub fn delete_lol_lobby_v2_lobby() -> DeleteLolLobbyV2Lobby {
    DeleteLolLobbyV2Lobby{}
}


pub struct DeleteLolLobbyV2LobbyMatchmakingSearch {}

impl IsApiRequest for DeleteLolLobbyV2LobbyMatchmakingSearch {
    const METHOD: Method = Method::DELETE;
    type ReturnType = Value;
    fn get_url(&self) -> String {"/lol-lobby/v2/lobby/matchmaking/search".to_string()}
}

pub fn delete_lol_lobby_v2_lobby_matchmaking_search() -> DeleteLolLobbyV2LobbyMatchmakingSearch {
    DeleteLolLobbyV2LobbyMatchmakingSearch{}
}


pub struct DeleteLolLobbyV2NotificationsByNotificationId {
    pub notification_id: String,
}

impl IsApiRequest for DeleteLolLobbyV2NotificationsByNotificationId {
    const METHOD: Method = Method::DELETE;
    type ReturnType = Value;
    fn get_url(&self) -> String {format!("/lol-lobby/v2/notifications/{}", self.notification_id)}
}

pub fn delete_lol_lobby_v2_notifications_by_notification_id(notification_id: String) -> DeleteLolLobbyV2NotificationsByNotificationId {
    DeleteLolLobbyV2NotificationsByNotificationId{notification_id}
}


pub struct GetLolLobbyV1AutofillDisplayed {}

impl IsApiRequest for GetLolLobbyV1AutofillDisplayed {
    const METHOD: Method = Method::GET;
    type ReturnType = bool;
    fn get_url(&self) -> String {"/lol-lobby/v1/autofill-displayed".to_string()}
}

pub fn get_lol_lobby_v1_autofill_displayed() -> GetLolLobbyV1AutofillDisplayed {
    GetLolLobbyV1AutofillDisplayed{}
}


pub struct GetLolLobbyV1CustomGames {}

impl IsApiRequest for GetLolLobbyV1CustomGames {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<LolLobbyLobbyCustomGame>;
    fn get_url(&self) -> String {"/lol-lobby/v1/custom-games".to_string()}
}

pub fn get_lol_lobby_v1_custom_games() -> GetLolLobbyV1CustomGames {
    GetLolLobbyV1CustomGames{}
}


pub struct GetLolLobbyV1CustomGamesById {
    pub id: i32,
}

impl IsApiRequest for GetLolLobbyV1CustomGamesById {
    const METHOD: Method = Method::GET;
    type ReturnType = LolLobbyLobbyCustomGame;
    fn get_url(&self) -> String {format!("/lol-lobby/v1/custom-games/{}", self.id)}
}

pub fn get_lol_lobby_v1_custom_games_by_id(id: i32) -> GetLolLobbyV1CustomGamesById {
    GetLolLobbyV1CustomGamesById{id}
}


pub struct GetLolLobbyV1LobbyAvailability {}

impl IsApiRequest for GetLolLobbyV1LobbyAvailability {
    const METHOD: Method = Method::GET;
    type ReturnType = LolLobbyQueueAvailability;
    fn get_url(&self) -> String {"/lol-lobby/v1/lobby/availability".to_string()}
}

pub fn get_lol_lobby_v1_lobby_availability() -> GetLolLobbyV1LobbyAvailability {
    GetLolLobbyV1LobbyAvailability{}
}


pub struct GetLolLobbyV1LobbyCountdown {}

impl IsApiRequest for GetLolLobbyV1LobbyCountdown {
    const METHOD: Method = Method::GET;
    type ReturnType = i64;
    fn get_url(&self) -> String {"/lol-lobby/v1/lobby/countdown".to_string()}
}

pub fn get_lol_lobby_v1_lobby_countdown() -> GetLolLobbyV1LobbyCountdown {
    GetLolLobbyV1LobbyCountdown{}
}


pub struct GetLolLobbyV1LobbyInvitations {}

impl IsApiRequest for GetLolLobbyV1LobbyInvitations {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<LolLobbyLobbyInvitation>;
    fn get_url(&self) -> String {"/lol-lobby/v1/lobby/invitations".to_string()}
}

pub fn get_lol_lobby_v1_lobby_invitations() -> GetLolLobbyV1LobbyInvitations {
    GetLolLobbyV1LobbyInvitations{}
}


pub struct GetLolLobbyV1LobbyInvitationsById {
    pub id: String,
}

impl IsApiRequest for GetLolLobbyV1LobbyInvitationsById {
    const METHOD: Method = Method::GET;
    type ReturnType = LolLobbyLobbyInvitation;
    fn get_url(&self) -> String {format!("/lol-lobby/v1/lobby/invitations/{}", self.id)}
}

pub fn get_lol_lobby_v1_lobby_invitations_by_id(id: String) -> GetLolLobbyV1LobbyInvitationsById {
    GetLolLobbyV1LobbyInvitationsById{id}
}


pub struct GetLolLobbyV1LobbyMembersLocalMemberPlayerSlots {}

impl IsApiRequest for GetLolLobbyV1LobbyMembersLocalMemberPlayerSlots {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<LolLobbyQuickPlayPresetSlotDto>;
    fn get_url(&self) -> String {"/lol-lobby/v1/lobby/members/localMember/player-slots".to_string()}
}

pub fn get_lol_lobby_v1_lobby_members_local_member_player_slots() -> GetLolLobbyV1LobbyMembersLocalMemberPlayerSlots {
    GetLolLobbyV1LobbyMembersLocalMemberPlayerSlots{}
}


pub struct GetLolLobbyV1LobbyTftRankedHistory {}

impl IsApiRequest for GetLolLobbyV1LobbyTftRankedHistory {
    const METHOD: Method = Method::GET;
    type ReturnType = bool;
    fn get_url(&self) -> String {"/lol-lobby/v1/lobby/tft-ranked-history".to_string()}
}

pub fn get_lol_lobby_v1_lobby_tft_ranked_history() -> GetLolLobbyV1LobbyTftRankedHistory {
    GetLolLobbyV1LobbyTftRankedHistory{}
}


pub struct GetLolLobbyV1PartiesGamemode {}

impl IsApiRequest for GetLolLobbyV1PartiesGamemode {
    const METHOD: Method = Method::GET;
    type ReturnType = LolLobbyGameModeDto;
    fn get_url(&self) -> String {"/lol-lobby/v1/parties/gamemode".to_string()}
}

pub fn get_lol_lobby_v1_parties_gamemode() -> GetLolLobbyV1PartiesGamemode {
    GetLolLobbyV1PartiesGamemode{}
}


pub struct GetLolLobbyV1PartiesPlayer {}

impl IsApiRequest for GetLolLobbyV1PartiesPlayer {
    const METHOD: Method = Method::GET;
    type ReturnType = LolLobbyPlayerDto;
    fn get_url(&self) -> String {"/lol-lobby/v1/parties/player".to_string()}
}

pub fn get_lol_lobby_v1_parties_player() -> GetLolLobbyV1PartiesPlayer {
    GetLolLobbyV1PartiesPlayer{}
}


pub struct GetLolLobbyV1PartyRewards {}

impl IsApiRequest for GetLolLobbyV1PartyRewards {
    const METHOD: Method = Method::GET;
    type ReturnType = LolLobbyLobbyPartyRewards;
    fn get_url(&self) -> String {"/lol-lobby/v1/party-rewards".to_string()}
}

pub fn get_lol_lobby_v1_party_rewards() -> GetLolLobbyV1PartyRewards {
    GetLolLobbyV1PartyRewards{}
}


pub struct GetLolLobbyV2CommsMembers {}

impl IsApiRequest for GetLolLobbyV2CommsMembers {
    const METHOD: Method = Method::GET;
    type ReturnType = LolLobbyPremadePartyDto;
    fn get_url(&self) -> String {"/lol-lobby/v2/comms/members".to_string()}
}

pub fn get_lol_lobby_v2_comms_members() -> GetLolLobbyV2CommsMembers {
    GetLolLobbyV2CommsMembers{}
}


pub struct GetLolLobbyV2CommsToken {}

impl IsApiRequest for GetLolLobbyV2CommsToken {
    const METHOD: Method = Method::GET;
    type ReturnType = String;
    fn get_url(&self) -> String {"/lol-lobby/v2/comms/token".to_string()}
}

pub fn get_lol_lobby_v2_comms_token() -> GetLolLobbyV2CommsToken {
    GetLolLobbyV2CommsToken{}
}


pub struct GetLolLobbyV2EligibilityGameSelectEligibilityHash {}

impl IsApiRequest for GetLolLobbyV2EligibilityGameSelectEligibilityHash {
    const METHOD: Method = Method::GET;
    type ReturnType = i64;
    fn get_url(&self) -> String {"/lol-lobby/v2/eligibility/game-select-eligibility-hash".to_string()}
}

pub fn get_lol_lobby_v2_eligibility_game_select_eligibility_hash() -> GetLolLobbyV2EligibilityGameSelectEligibilityHash {
    GetLolLobbyV2EligibilityGameSelectEligibilityHash{}
}


pub struct GetLolLobbyV2EligibilityInitialConfigurationComplete {}

impl IsApiRequest for GetLolLobbyV2EligibilityInitialConfigurationComplete {
    const METHOD: Method = Method::GET;
    type ReturnType = bool;
    fn get_url(&self) -> String {"/lol-lobby/v2/eligibility/initial-configuration-complete".to_string()}
}

pub fn get_lol_lobby_v2_eligibility_initial_configuration_complete() -> GetLolLobbyV2EligibilityInitialConfigurationComplete {
    GetLolLobbyV2EligibilityInitialConfigurationComplete{}
}


pub struct GetLolLobbyV2Lobby {}

impl IsApiRequest for GetLolLobbyV2Lobby {
    const METHOD: Method = Method::GET;
    type ReturnType = LolLobbyLobbyDto;
    fn get_url(&self) -> String {"/lol-lobby/v2/lobby".to_string()}
}

pub fn get_lol_lobby_v2_lobby() -> GetLolLobbyV2Lobby {
    GetLolLobbyV2Lobby{}
}


pub struct GetLolLobbyV2LobbyCustomAvailableBots {}

impl IsApiRequest for GetLolLobbyV2LobbyCustomAvailableBots {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<LolLobbyLobbyBotChampion>;
    fn get_url(&self) -> String {"/lol-lobby/v2/lobby/custom/available-bots".to_string()}
}

pub fn get_lol_lobby_v2_lobby_custom_available_bots() -> GetLolLobbyV2LobbyCustomAvailableBots {
    GetLolLobbyV2LobbyCustomAvailableBots{}
}


pub struct GetLolLobbyV2LobbyCustomBotsEnabled {}

impl IsApiRequest for GetLolLobbyV2LobbyCustomBotsEnabled {
    const METHOD: Method = Method::GET;
    type ReturnType = bool;
    fn get_url(&self) -> String {"/lol-lobby/v2/lobby/custom/bots-enabled".to_string()}
}

pub fn get_lol_lobby_v2_lobby_custom_bots_enabled() -> GetLolLobbyV2LobbyCustomBotsEnabled {
    GetLolLobbyV2LobbyCustomBotsEnabled{}
}


pub struct GetLolLobbyV2LobbyInvitations {}

impl IsApiRequest for GetLolLobbyV2LobbyInvitations {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<LolLobbyLobbyInvitationDto>;
    fn get_url(&self) -> String {"/lol-lobby/v2/lobby/invitations".to_string()}
}

pub fn get_lol_lobby_v2_lobby_invitations() -> GetLolLobbyV2LobbyInvitations {
    GetLolLobbyV2LobbyInvitations{}
}


pub struct GetLolLobbyV2LobbyMatchmakingSearchState {}

impl IsApiRequest for GetLolLobbyV2LobbyMatchmakingSearchState {
    const METHOD: Method = Method::GET;
    type ReturnType = LolLobbyLobbyMatchmakingSearchResource;
    fn get_url(&self) -> String {"/lol-lobby/v2/lobby/matchmaking/search-state".to_string()}
}

pub fn get_lol_lobby_v2_lobby_matchmaking_search_state() -> GetLolLobbyV2LobbyMatchmakingSearchState {
    GetLolLobbyV2LobbyMatchmakingSearchState{}
}


pub struct GetLolLobbyV2LobbyMembers {}

impl IsApiRequest for GetLolLobbyV2LobbyMembers {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<LolLobbyLobbyParticipantDto>;
    fn get_url(&self) -> String {"/lol-lobby/v2/lobby/members".to_string()}
}

pub fn get_lol_lobby_v2_lobby_members() -> GetLolLobbyV2LobbyMembers {
    GetLolLobbyV2LobbyMembers{}
}


pub struct GetLolLobbyV2Notifications {}

impl IsApiRequest for GetLolLobbyV2Notifications {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<LolLobbyLobbyNotification>;
    fn get_url(&self) -> String {"/lol-lobby/v2/notifications".to_string()}
}

pub fn get_lol_lobby_v2_notifications() -> GetLolLobbyV2Notifications {
    GetLolLobbyV2Notifications{}
}


pub struct GetLolLobbyV2PartyActive {}

impl IsApiRequest for GetLolLobbyV2PartyActive {
    const METHOD: Method = Method::GET;
    type ReturnType = bool;
    fn get_url(&self) -> String {"/lol-lobby/v2/party-active".to_string()}
}

pub fn get_lol_lobby_v2_party_active() -> GetLolLobbyV2PartyActive {
    GetLolLobbyV2PartyActive{}
}


pub struct GetLolLobbyV2PartyEogStatus {}

impl IsApiRequest for GetLolLobbyV2PartyEogStatus {
    const METHOD: Method = Method::GET;
    type ReturnType = LolLobbyPartyStatusDto;
    fn get_url(&self) -> String {"/lol-lobby/v2/party/eog-status".to_string()}
}

pub fn get_lol_lobby_v2_party_eog_status() -> GetLolLobbyV2PartyEogStatus {
    GetLolLobbyV2PartyEogStatus{}
}


pub struct GetLolLobbyV2ReceivedInvitations {}

impl IsApiRequest for GetLolLobbyV2ReceivedInvitations {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<LolLobbyReceivedInvitationDto>;
    fn get_url(&self) -> String {"/lol-lobby/v2/received-invitations".to_string()}
}

pub fn get_lol_lobby_v2_received_invitations() -> GetLolLobbyV2ReceivedInvitations {
    GetLolLobbyV2ReceivedInvitations{}
}


pub struct GetLolLobbyV2RegistrationStatus {}

impl IsApiRequest for GetLolLobbyV2RegistrationStatus {
    const METHOD: Method = Method::GET;
    type ReturnType = Value;
    fn get_url(&self) -> String {"/lol-lobby/v2/registration-status".to_string()}
}

pub fn get_lol_lobby_v2_registration_status() -> GetLolLobbyV2RegistrationStatus {
    GetLolLobbyV2RegistrationStatus{}
}


pub struct PostLolLobbyV1Clash {
    pub body: String,
}

impl IsApiRequest for PostLolLobbyV1Clash {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;
    fn get_url(&self) -> String {"/lol-lobby/v1/clash".to_string()}
    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }
}

pub fn post_lol_lobby_v1_clash(body: String) -> PostLolLobbyV1Clash {
    PostLolLobbyV1Clash{body}
}


pub struct PostLolLobbyV1CustomGamesByIdJoin {
    pub id: u64,
    pub body: LolLobbyLobbyCustomJoinParameters,
}

impl IsApiRequest for PostLolLobbyV1CustomGamesByIdJoin {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;
    fn get_url(&self) -> String {format!("/lol-lobby/v1/custom-games/{}/join", self.id)}
    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }
}

pub fn post_lol_lobby_v1_custom_games_by_id_join(id: u64, body: LolLobbyLobbyCustomJoinParameters) -> PostLolLobbyV1CustomGamesByIdJoin {
    PostLolLobbyV1CustomGamesByIdJoin{id, body}
}


pub struct PostLolLobbyV1CustomGamesRefresh {}

impl IsApiRequest for PostLolLobbyV1CustomGamesRefresh {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;
    fn get_url(&self) -> String {"/lol-lobby/v1/custom-games/refresh".to_string()}
}

pub fn post_lol_lobby_v1_custom_games_refresh() -> PostLolLobbyV1CustomGamesRefresh {
    PostLolLobbyV1CustomGamesRefresh{}
}


pub struct PostLolLobbyV1LobbyCustomBots {
    pub body: LolLobbyLobbyBotParams,
}

impl IsApiRequest for PostLolLobbyV1LobbyCustomBots {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;
    fn get_url(&self) -> String {"/lol-lobby/v1/lobby/custom/bots".to_string()}
    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }
}

pub fn post_lol_lobby_v1_lobby_custom_bots(body: LolLobbyLobbyBotParams) -> PostLolLobbyV1LobbyCustomBots {
    PostLolLobbyV1LobbyCustomBots{body}
}


pub struct PostLolLobbyV1LobbyCustomBotsBySummonerInternalNameByBotUuidToDelete {
    pub summoner_internal_name: String,
    pub bot_uuid_to_delete: String,
    pub body: LolLobbyLobbyBotParams,
}

impl IsApiRequest for PostLolLobbyV1LobbyCustomBotsBySummonerInternalNameByBotUuidToDelete {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;
    fn get_url(&self) -> String {format!("/lol-lobby/v1/lobby/custom/bots/{}/{}", self.summoner_internal_name, self.bot_uuid_to_delete)}
    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }
}

pub fn post_lol_lobby_v1_lobby_custom_bots_by_summoner_internal_name_by_bot_uuid_to_delete(summoner_internal_name: String, bot_uuid_to_delete: String, body: LolLobbyLobbyBotParams) -> PostLolLobbyV1LobbyCustomBotsBySummonerInternalNameByBotUuidToDelete {
    PostLolLobbyV1LobbyCustomBotsBySummonerInternalNameByBotUuidToDelete{summoner_internal_name, bot_uuid_to_delete, body}
}


pub struct PostLolLobbyV1LobbyCustomCancelChampSelect {}

impl IsApiRequest for PostLolLobbyV1LobbyCustomCancelChampSelect {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;
    fn get_url(&self) -> String {"/lol-lobby/v1/lobby/custom/cancel-champ-select".to_string()}
}

pub fn post_lol_lobby_v1_lobby_custom_cancel_champ_select() -> PostLolLobbyV1LobbyCustomCancelChampSelect {
    PostLolLobbyV1LobbyCustomCancelChampSelect{}
}


pub struct PostLolLobbyV1LobbyCustomStartChampSelect {}

impl IsApiRequest for PostLolLobbyV1LobbyCustomStartChampSelect {
    const METHOD: Method = Method::POST;
    type ReturnType = LolLobbyLobbyCustomChampSelectStartResponse;
    fn get_url(&self) -> String {"/lol-lobby/v1/lobby/custom/start-champ-select".to_string()}
}

pub fn post_lol_lobby_v1_lobby_custom_start_champ_select() -> PostLolLobbyV1LobbyCustomStartChampSelect {
    PostLolLobbyV1LobbyCustomStartChampSelect{}
}


pub struct PostLolLobbyV1LobbyCustomSwitchTeams {
    pub body: String,
}

impl IsApiRequest for PostLolLobbyV1LobbyCustomSwitchTeams {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;
    fn get_url(&self) -> String {"/lol-lobby/v1/lobby/custom/switch-teams".to_string()}
    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }
}

pub fn post_lol_lobby_v1_lobby_custom_switch_teams(body: String) -> PostLolLobbyV1LobbyCustomSwitchTeams {
    PostLolLobbyV1LobbyCustomSwitchTeams{body}
}


pub struct PostLolLobbyV1LobbyInvitations {
    pub body: LolLobbyLobbyInvitation,
}

impl IsApiRequest for PostLolLobbyV1LobbyInvitations {
    const METHOD: Method = Method::POST;
    type ReturnType = LolLobbyLobbyInvitation;
    fn get_url(&self) -> String {"/lol-lobby/v1/lobby/invitations".to_string()}
    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }
}

pub fn post_lol_lobby_v1_lobby_invitations(body: LolLobbyLobbyInvitation) -> PostLolLobbyV1LobbyInvitations {
    PostLolLobbyV1LobbyInvitations{body}
}


pub struct PostLolLobbyV1LobbyMembersLocalMemberPlayerSlotsBySlotsIndexByPerksString {
    pub slots_index: u64,
    pub perks_string: String,
}

impl IsApiRequest for PostLolLobbyV1LobbyMembersLocalMemberPlayerSlotsBySlotsIndexByPerksString {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;
    fn get_url(&self) -> String {format!("/lol-lobby/v1/lobby/members/localMember/player-slots/{}/{}", self.slots_index, self.perks_string)}
}

pub fn post_lol_lobby_v1_lobby_members_local_member_player_slots_by_slots_index_by_perks_string(slots_index: u64, perks_string: String) -> PostLolLobbyV1LobbyMembersLocalMemberPlayerSlotsBySlotsIndexByPerksString {
    PostLolLobbyV1LobbyMembersLocalMemberPlayerSlotsBySlotsIndexByPerksString{slots_index, perks_string}
}


pub struct PostLolLobbyV1TournamentsByIdJoin {
    pub id: String,
}

impl IsApiRequest for PostLolLobbyV1TournamentsByIdJoin {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;
    fn get_url(&self) -> String {format!("/lol-lobby/v1/tournaments/{}/join", self.id)}
}

pub fn post_lol_lobby_v1_tournaments_by_id_join(id: String) -> PostLolLobbyV1TournamentsByIdJoin {
    PostLolLobbyV1TournamentsByIdJoin{id}
}


pub struct PostLolLobbyV2EligibilityParty {}

impl IsApiRequest for PostLolLobbyV2EligibilityParty {
    const METHOD: Method = Method::POST;
    type ReturnType = Vec<LolLobbyEligibility>;
    fn get_url(&self) -> String {"/lol-lobby/v2/eligibility/party".to_string()}
}

pub fn post_lol_lobby_v2_eligibility_party() -> PostLolLobbyV2EligibilityParty {
    PostLolLobbyV2EligibilityParty{}
}


pub struct PostLolLobbyV2EligibilitySelf {}

impl IsApiRequest for PostLolLobbyV2EligibilitySelf {
    const METHOD: Method = Method::POST;
    type ReturnType = Vec<LolLobbyEligibility>;
    fn get_url(&self) -> String {"/lol-lobby/v2/eligibility/self".to_string()}
}

pub fn post_lol_lobby_v2_eligibility_self() -> PostLolLobbyV2EligibilitySelf {
    PostLolLobbyV2EligibilitySelf{}
}


pub struct PostLolLobbyV2EogInvitations {
    pub body: Vec<LolLobbyLobbyInvitationDto>,
}

impl IsApiRequest for PostLolLobbyV2EogInvitations {
    const METHOD: Method = Method::POST;
    type ReturnType = Vec<LolLobbyLobbyInvitationDto>;
    fn get_url(&self) -> String {"/lol-lobby/v2/eog-invitations".to_string()}
    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }
}

pub fn post_lol_lobby_v2_eog_invitations(body: Vec<LolLobbyLobbyInvitationDto>) -> PostLolLobbyV2EogInvitations {
    PostLolLobbyV2EogInvitations{body}
}


pub struct PostLolLobbyV2Lobby {
    pub body: LolLobbyLobbyChangeGameDto,
}

impl IsApiRequest for PostLolLobbyV2Lobby {
    const METHOD: Method = Method::POST;
    type ReturnType = LolLobbyLobbyDto;
    fn get_url(&self) -> String {"/lol-lobby/v2/lobby".to_string()}
    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }
}

pub fn post_lol_lobby_v2_lobby(body: LolLobbyLobbyChangeGameDto) -> PostLolLobbyV2Lobby {
    PostLolLobbyV2Lobby{body}
}


pub struct PostLolLobbyV2LobbyInvitations {
    pub body: Vec<LolLobbyLobbyInvitationDto>,
}

impl IsApiRequest for PostLolLobbyV2LobbyInvitations {
    const METHOD: Method = Method::POST;
    type ReturnType = Vec<LolLobbyLobbyInvitationDto>;
    fn get_url(&self) -> String {"/lol-lobby/v2/lobby/invitations".to_string()}
    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }
}

pub fn post_lol_lobby_v2_lobby_invitations(body: Vec<LolLobbyLobbyInvitationDto>) -> PostLolLobbyV2LobbyInvitations {
    PostLolLobbyV2LobbyInvitations{body}
}


pub struct PostLolLobbyV2LobbyMatchmakingSearch {}

impl IsApiRequest for PostLolLobbyV2LobbyMatchmakingSearch {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;
    fn get_url(&self) -> String {"/lol-lobby/v2/lobby/matchmaking/search".to_string()}
}

pub fn post_lol_lobby_v2_lobby_matchmaking_search() -> PostLolLobbyV2LobbyMatchmakingSearch {
    PostLolLobbyV2LobbyMatchmakingSearch{}
}


pub struct PostLolLobbyV2LobbyMembersBySummonerIdGrantInvite {
    pub summoner_id: u64,
}

impl IsApiRequest for PostLolLobbyV2LobbyMembersBySummonerIdGrantInvite {
    const METHOD: Method = Method::POST;
    type ReturnType = u64;
    fn get_url(&self) -> String {format!("/lol-lobby/v2/lobby/members/{}/grant-invite", self.summoner_id)}
}

pub fn post_lol_lobby_v2_lobby_members_by_summoner_id_grant_invite(summoner_id: u64) -> PostLolLobbyV2LobbyMembersBySummonerIdGrantInvite {
    PostLolLobbyV2LobbyMembersBySummonerIdGrantInvite{summoner_id}
}


pub struct PostLolLobbyV2LobbyMembersBySummonerIdKick {
    pub summoner_id: u64,
}

impl IsApiRequest for PostLolLobbyV2LobbyMembersBySummonerIdKick {
    const METHOD: Method = Method::POST;
    type ReturnType = u64;
    fn get_url(&self) -> String {format!("/lol-lobby/v2/lobby/members/{}/kick", self.summoner_id)}
}

pub fn post_lol_lobby_v2_lobby_members_by_summoner_id_kick(summoner_id: u64) -> PostLolLobbyV2LobbyMembersBySummonerIdKick {
    PostLolLobbyV2LobbyMembersBySummonerIdKick{summoner_id}
}


pub struct PostLolLobbyV2LobbyMembersBySummonerIdPromote {
    pub summoner_id: u64,
}

impl IsApiRequest for PostLolLobbyV2LobbyMembersBySummonerIdPromote {
    const METHOD: Method = Method::POST;
    type ReturnType = u64;
    fn get_url(&self) -> String {format!("/lol-lobby/v2/lobby/members/{}/promote", self.summoner_id)}
}

pub fn post_lol_lobby_v2_lobby_members_by_summoner_id_promote(summoner_id: u64) -> PostLolLobbyV2LobbyMembersBySummonerIdPromote {
    PostLolLobbyV2LobbyMembersBySummonerIdPromote{summoner_id}
}


pub struct PostLolLobbyV2LobbyMembersBySummonerIdRevokeInvite {
    pub summoner_id: u64,
}

impl IsApiRequest for PostLolLobbyV2LobbyMembersBySummonerIdRevokeInvite {
    const METHOD: Method = Method::POST;
    type ReturnType = u64;
    fn get_url(&self) -> String {format!("/lol-lobby/v2/lobby/members/{}/revoke-invite", self.summoner_id)}
}

pub fn post_lol_lobby_v2_lobby_members_by_summoner_id_revoke_invite(summoner_id: u64) -> PostLolLobbyV2LobbyMembersBySummonerIdRevokeInvite {
    PostLolLobbyV2LobbyMembersBySummonerIdRevokeInvite{summoner_id}
}


pub struct PostLolLobbyV2LobbyTeamByTeam {
    pub team: String,
}

impl IsApiRequest for PostLolLobbyV2LobbyTeamByTeam {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;
    fn get_url(&self) -> String {format!("/lol-lobby/v2/lobby/team/{}", self.team)}
}

pub fn post_lol_lobby_v2_lobby_team_by_team(team: String) -> PostLolLobbyV2LobbyTeamByTeam {
    PostLolLobbyV2LobbyTeamByTeam{team}
}


pub struct PostLolLobbyV2MatchmakingQuickSearch {
    pub body: LolLobbyLobbyChangeGameDto,
}

impl IsApiRequest for PostLolLobbyV2MatchmakingQuickSearch {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;
    fn get_url(&self) -> String {"/lol-lobby/v2/matchmaking/quick-search".to_string()}
    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }
}

pub fn post_lol_lobby_v2_matchmaking_quick_search(body: LolLobbyLobbyChangeGameDto) -> PostLolLobbyV2MatchmakingQuickSearch {
    PostLolLobbyV2MatchmakingQuickSearch{body}
}


pub struct PostLolLobbyV2Notifications {
    pub body: LolLobbyLobbyNotification,
}

impl IsApiRequest for PostLolLobbyV2Notifications {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;
    fn get_url(&self) -> String {"/lol-lobby/v2/notifications".to_string()}
    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }
}

pub fn post_lol_lobby_v2_notifications(body: LolLobbyLobbyNotification) -> PostLolLobbyV2Notifications {
    PostLolLobbyV2Notifications{body}
}


pub struct PostLolLobbyV2PartiesOverridesEnabledForTeamBuilderQueues {
    pub body: bool,
}

impl IsApiRequest for PostLolLobbyV2PartiesOverridesEnabledForTeamBuilderQueues {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;
    fn get_url(&self) -> String {"/lol-lobby/v2/parties/overrides/EnabledForTeamBuilderQueues".to_string()}
    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }
}

pub fn post_lol_lobby_v2_parties_overrides_enabled_for_team_builder_queues(body: bool) -> PostLolLobbyV2PartiesOverridesEnabledForTeamBuilderQueues {
    PostLolLobbyV2PartiesOverridesEnabledForTeamBuilderQueues{body}
}


pub struct PostLolLobbyV2PartyByPartyIdJoin {
    pub party_id: String,
    pub body: LolLobbyCustomJoinOptionsDto,
}

impl IsApiRequest for PostLolLobbyV2PartyByPartyIdJoin {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;
    fn get_url(&self) -> String {format!("/lol-lobby/v2/party/{}/join", self.party_id)}
    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }
}

pub fn post_lol_lobby_v2_party_by_party_id_join(party_id: String, body: LolLobbyCustomJoinOptionsDto) -> PostLolLobbyV2PartyByPartyIdJoin {
    PostLolLobbyV2PartyByPartyIdJoin{party_id, body}
}


pub struct PostLolLobbyV2PlayAgain {}

impl IsApiRequest for PostLolLobbyV2PlayAgain {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;
    fn get_url(&self) -> String {"/lol-lobby/v2/play-again".to_string()}
}

pub fn post_lol_lobby_v2_play_again() -> PostLolLobbyV2PlayAgain {
    PostLolLobbyV2PlayAgain{}
}


pub struct PostLolLobbyV2PlayAgainDecline {}

impl IsApiRequest for PostLolLobbyV2PlayAgainDecline {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;
    fn get_url(&self) -> String {"/lol-lobby/v2/play-again-decline".to_string()}
}

pub fn post_lol_lobby_v2_play_again_decline() -> PostLolLobbyV2PlayAgainDecline {
    PostLolLobbyV2PlayAgainDecline{}
}


pub struct PostLolLobbyV2ReceivedInvitationsByInvitationIdAccept {
    pub invitation_id: String,
}

impl IsApiRequest for PostLolLobbyV2ReceivedInvitationsByInvitationIdAccept {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;
    fn get_url(&self) -> String {format!("/lol-lobby/v2/received-invitations/{}/accept", self.invitation_id)}
}

pub fn post_lol_lobby_v2_received_invitations_by_invitation_id_accept(invitation_id: String) -> PostLolLobbyV2ReceivedInvitationsByInvitationIdAccept {
    PostLolLobbyV2ReceivedInvitationsByInvitationIdAccept{invitation_id}
}


pub struct PostLolLobbyV2ReceivedInvitationsByInvitationIdDecline {
    pub invitation_id: String,
}

impl IsApiRequest for PostLolLobbyV2ReceivedInvitationsByInvitationIdDecline {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;
    fn get_url(&self) -> String {format!("/lol-lobby/v2/received-invitations/{}/decline", self.invitation_id)}
}

pub fn post_lol_lobby_v2_received_invitations_by_invitation_id_decline(invitation_id: String) -> PostLolLobbyV2ReceivedInvitationsByInvitationIdDecline {
    PostLolLobbyV2ReceivedInvitationsByInvitationIdDecline{invitation_id}
}


pub struct PutLolLobbyV1AutofillDisplayed {}

impl IsApiRequest for PutLolLobbyV1AutofillDisplayed {
    const METHOD: Method = Method::PUT;
    type ReturnType = bool;
    fn get_url(&self) -> String {"/lol-lobby/v1/autofill-displayed".to_string()}
}

pub fn put_lol_lobby_v1_autofill_displayed() -> PutLolLobbyV1AutofillDisplayed {
    PutLolLobbyV1AutofillDisplayed{}
}


pub struct PutLolLobbyV1LobbyMembersLocalMemberPlayerSlots {
    pub body: Vec<LolLobbyQuickPlayPresetSlotDto>,
}

impl IsApiRequest for PutLolLobbyV1LobbyMembersLocalMemberPlayerSlots {
    const METHOD: Method = Method::PUT;
    type ReturnType = Value;
    fn get_url(&self) -> String {"/lol-lobby/v1/lobby/members/localMember/player-slots".to_string()}
    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }
}

pub fn put_lol_lobby_v1_lobby_members_local_member_player_slots(body: Vec<LolLobbyQuickPlayPresetSlotDto>) -> PutLolLobbyV1LobbyMembersLocalMemberPlayerSlots {
    PutLolLobbyV1LobbyMembersLocalMemberPlayerSlots{body}
}


pub struct PutLolLobbyV1LobbyMembersLocalMemberPositionPreferences {
    pub body: LolLobbyLobbyPositionPreferences,
}

impl IsApiRequest for PutLolLobbyV1LobbyMembersLocalMemberPositionPreferences {
    const METHOD: Method = Method::PUT;
    type ReturnType = Value;
    fn get_url(&self) -> String {"/lol-lobby/v1/lobby/members/localMember/position-preferences".to_string()}
    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }
}

pub fn put_lol_lobby_v1_lobby_members_local_member_position_preferences(body: LolLobbyLobbyPositionPreferences) -> PutLolLobbyV1LobbyMembersLocalMemberPositionPreferences {
    PutLolLobbyV1LobbyMembersLocalMemberPositionPreferences{body}
}


pub struct PutLolLobbyV1LobbyMembersLocalMemberQuickplayPlayerState {
    pub body: String,
}

impl IsApiRequest for PutLolLobbyV1LobbyMembersLocalMemberQuickplayPlayerState {
    const METHOD: Method = Method::PUT;
    type ReturnType = Value;
    fn get_url(&self) -> String {"/lol-lobby/v1/lobby/members/localMember/quickplayPlayerState".to_string()}
    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }
}

pub fn put_lol_lobby_v1_lobby_members_local_member_quickplay_player_state(body: String) -> PutLolLobbyV1LobbyMembersLocalMemberQuickplayPlayerState {
    PutLolLobbyV1LobbyMembersLocalMemberQuickplayPlayerState{body}
}


pub struct PutLolLobbyV1PartiesActive {
    pub body: i32,
}

impl IsApiRequest for PutLolLobbyV1PartiesActive {
    const METHOD: Method = Method::PUT;
    type ReturnType = Value;
    fn get_url(&self) -> String {"/lol-lobby/v1/parties/active".to_string()}
    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }
}

pub fn put_lol_lobby_v1_parties_active(body: i32) -> PutLolLobbyV1PartiesActive {
    PutLolLobbyV1PartiesActive{body}
}


pub struct PutLolLobbyV1PartiesByPartyIdMembersByPuuidRole {
    pub party_id: String,
    pub puuid: String,
    pub body: String,
}

impl IsApiRequest for PutLolLobbyV1PartiesByPartyIdMembersByPuuidRole {
    const METHOD: Method = Method::PUT;
    type ReturnType = Value;
    fn get_url(&self) -> String {format!("/lol-lobby/v1/parties/{}/members/{}/role", self.party_id, self.puuid)}
    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }
}

pub fn put_lol_lobby_v1_parties_by_party_id_members_by_puuid_role(party_id: String, puuid: String, body: String) -> PutLolLobbyV1PartiesByPartyIdMembersByPuuidRole {
    PutLolLobbyV1PartiesByPartyIdMembersByPuuidRole{party_id, puuid, body}
}


pub struct PutLolLobbyV1PartiesMetadata {
    pub body: LolLobbyPartyMemberMetadataDto,
}

impl IsApiRequest for PutLolLobbyV1PartiesMetadata {
    const METHOD: Method = Method::PUT;
    type ReturnType = Value;
    fn get_url(&self) -> String {"/lol-lobby/v1/parties/metadata".to_string()}
    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }
}

pub fn put_lol_lobby_v1_parties_metadata(body: LolLobbyPartyMemberMetadataDto) -> PutLolLobbyV1PartiesMetadata {
    PutLolLobbyV1PartiesMetadata{body}
}


pub struct PutLolLobbyV1PartiesQueue {
    pub body: i32,
}

impl IsApiRequest for PutLolLobbyV1PartiesQueue {
    const METHOD: Method = Method::PUT;
    type ReturnType = Value;
    fn get_url(&self) -> String {"/lol-lobby/v1/parties/queue".to_string()}
    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }
}

pub fn put_lol_lobby_v1_parties_queue(body: i32) -> PutLolLobbyV1PartiesQueue {
    PutLolLobbyV1PartiesQueue{body}
}


pub struct PutLolLobbyV1PartiesReady {
    pub body: i32,
}

impl IsApiRequest for PutLolLobbyV1PartiesReady {
    const METHOD: Method = Method::PUT;
    type ReturnType = Value;
    fn get_url(&self) -> String {"/lol-lobby/v1/parties/ready".to_string()}
    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }
}

pub fn put_lol_lobby_v1_parties_ready(body: i32) -> PutLolLobbyV1PartiesReady {
    PutLolLobbyV1PartiesReady{body}
}


pub struct PutLolLobbyV2LobbyMembersLocalMemberPositionPreferences {
    pub body: LolLobbyLobbyPositionPreferences,
}

impl IsApiRequest for PutLolLobbyV2LobbyMembersLocalMemberPositionPreferences {
    const METHOD: Method = Method::PUT;
    type ReturnType = Value;
    fn get_url(&self) -> String {"/lol-lobby/v2/lobby/members/localMember/position-preferences".to_string()}
    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }
}

pub fn put_lol_lobby_v2_lobby_members_local_member_position_preferences(body: LolLobbyLobbyPositionPreferences) -> PutLolLobbyV2LobbyMembersLocalMemberPositionPreferences {
    PutLolLobbyV2LobbyMembersLocalMemberPositionPreferences{body}
}


pub struct PutLolLobbyV2LobbyPartyType {
    pub body: String,
}

impl IsApiRequest for PutLolLobbyV2LobbyPartyType {
    const METHOD: Method = Method::PUT;
    type ReturnType = Value;
    fn get_url(&self) -> String {"/lol-lobby/v2/lobby/partyType".to_string()}
    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }
}

pub fn put_lol_lobby_v2_lobby_party_type(body: String) -> PutLolLobbyV2LobbyPartyType {
    PutLolLobbyV2LobbyPartyType{body}
}


pub struct PutLolLobbyV2LobbyStrawberryMapId {
    pub body: LolLobbyStrawberryMapUpdateDto,
}

impl IsApiRequest for PutLolLobbyV2LobbyStrawberryMapId {
    const METHOD: Method = Method::PUT;
    type ReturnType = Value;
    fn get_url(&self) -> String {"/lol-lobby/v2/lobby/strawberryMapId".to_string()}
    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }
}

pub fn put_lol_lobby_v2_lobby_strawberry_map_id(body: LolLobbyStrawberryMapUpdateDto) -> PutLolLobbyV2LobbyStrawberryMapId {
    PutLolLobbyV2LobbyStrawberryMapId{body}
}


pub struct PutLolLobbyV2LobbySubteamData {
    pub body: LolLobbySubteamDataDto,
}

impl IsApiRequest for PutLolLobbyV2LobbySubteamData {
    const METHOD: Method = Method::PUT;
    type ReturnType = Value;
    fn get_url(&self) -> String {"/lol-lobby/v2/lobby/subteamData".to_string()}
    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }
}

pub fn put_lol_lobby_v2_lobby_subteam_data(body: LolLobbySubteamDataDto) -> PutLolLobbyV2LobbySubteamData {
    PutLolLobbyV2LobbySubteamData{body}
}


// OBJECTS

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyBotParticipantDto {
    #[serde(rename = "championId")]
    pub champion_id: i32,
    #[serde(rename = "botSkillLevel")]
    pub bot_skill_level: i32,
    pub team: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyCustomGameSettingsDto {
    #[serde(rename = "lobbyName")]
    pub lobby_name: String,
    #[serde(rename = "lobbyPassword")]
    pub lobby_password: String,
    #[serde(rename = "gameId")]
    pub game_id: u64,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyCustomJoinOptionsDto {
    #[serde(rename = "lobbyPassword")]
    pub lobby_password: String,
    pub team: Option<String>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyEligibility {
    #[serde(rename = "queueId")]
    pub queue_id: i32,
    pub eligible: bool,
    pub restrictions: Vec<LolLobbyEligibilityRestriction>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyEligibilityRestriction {
    #[serde(rename = "restrictionCode")]
    pub restriction_code: LolLobbyEligibilityRestrictionCode,
    #[serde(rename = "restrictionArgs")]
    pub restriction_args: HashMap<String, String>,
    #[serde(rename = "expiredTimestamp")]
    pub expired_timestamp: u64,
    #[serde(rename = "summonerIds")]
    pub summoner_ids: Vec<u64>,
    #[serde(rename = "summonerIdsString")]
    pub summoner_ids_string: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyGameModeDto {
    #[serde(rename = "gameType")]
    pub game_type: String,
    #[serde(rename = "maxTeamSize")]
    pub max_team_size: i32,
    #[serde(rename = "maxPartySize")]
    pub max_party_size: i32,
    #[serde(rename = "botDifficulty")]
    pub bot_difficulty: Option<String>,
    #[serde(rename = "queueId")]
    pub queue_id: Option<i32>,
    #[serde(rename = "gameCustomization")]
    pub game_customization: Option<HashMap<String, String>>,
    #[serde(rename = "customsSettings")]
    pub customs_settings: Option<LolLobbyCustomGameSettingsDto>,
    #[serde(rename = "gameTypeConfigId")]
    pub game_type_config_id: Option<i64>,
    #[serde(rename = "mapId")]
    pub map_id: Option<i32>,
    #[serde(rename = "allowSpectators")]
    pub allow_spectators: Option<String>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyGatekeeperRestrictionDto {
    #[serde(rename = "accountId")]
    pub account_id: u64,
    pub reason: String,
    #[serde(rename = "remainingMillis")]
    pub remaining_millis: i64,
    pub payload: String,
    #[serde(rename = "queueId")]
    pub queue_id: i32,
    pub puuid: String,
    pub details: HashMap<String, String>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyLobbyBotChampion {
    pub active: bool,
    pub id: i32,
    pub name: String,
    #[serde(rename = "botDifficulties")]
    pub bot_difficulties: Vec<LolLobbyLobbyBotDifficulty>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyLobbyBotParams {
    #[serde(rename = "championId")]
    pub champion_id: i32,
    #[serde(rename = "botDifficulty")]
    pub bot_difficulty: LolLobbyLobbyBotDifficulty,
    #[serde(rename = "teamId")]
    pub team_id: String,
    pub position: String,
    #[serde(rename = "botUuid")]
    pub bot_uuid: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyLobbyChangeGameDto {
    #[serde(rename = "queueId")]
    pub queue_id: i32,
    #[serde(rename = "isCustom")]
    pub is_custom: bool,
    #[serde(rename = "customGameLobby")]
    pub custom_game_lobby: Option<LolLobbyLobbyCustomGameLobby>,
    #[serde(rename = "gameCustomization")]
    pub game_customization: Option<HashMap<String, String>>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyLobbyCustomChampSelectStartResponse {
    pub success: bool,
    #[serde(rename = "failedPlayers")]
    pub failed_players: Vec<LolLobbyLobbyCustomFailedPlayer>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyLobbyCustomFailedPlayer {
    #[serde(rename = "summonerId")]
    pub summoner_id: u64,
    #[serde(rename = "summonerName")]
    pub summoner_name: String,
    pub reason: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyLobbyCustomGame {
    pub id: u64,
    #[serde(rename = "gameType")]
    pub game_type: String,
    #[serde(rename = "mapId")]
    pub map_id: i32,
    #[serde(rename = "ownerDisplayName")]
    pub owner_display_name: String,
    #[serde(rename = "spectatorPolicy")]
    pub spectator_policy: String,
    #[serde(rename = "filledSpectatorSlots")]
    pub filled_spectator_slots: i32,
    #[serde(rename = "maxSpectatorSlots")]
    pub max_spectator_slots: u64,
    #[serde(rename = "filledPlayerSlots")]
    pub filled_player_slots: i32,
    #[serde(rename = "maxPlayerSlots")]
    pub max_player_slots: i32,
    #[serde(rename = "lobbyName")]
    pub lobby_name: String,
    #[serde(rename = "hasPassword")]
    pub has_password: bool,
    #[serde(rename = "passbackUrl")]
    pub passback_url: String,
    #[serde(rename = "partyId")]
    pub party_id: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyLobbyCustomGameConfiguration {
    #[serde(rename = "mapId")]
    pub map_id: i32,
    #[serde(rename = "gameMode")]
    pub game_mode: String,
    pub mutators: LolLobbyQueueGameTypeConfig,
    #[serde(rename = "gameTypeConfig")]
    pub game_type_config: LolLobbyQueueGameTypeConfig,
    #[serde(rename = "spectatorPolicy")]
    pub spectator_policy: LolLobbyQueueCustomGameSpectatorPolicy,
    #[serde(rename = "teamSize")]
    pub team_size: i32,
    #[serde(rename = "maxPlayerCount")]
    pub max_player_count: u32,
    #[serde(rename = "tournamentGameMode")]
    pub tournament_game_mode: String,
    #[serde(rename = "tournamentPassbackUrl")]
    pub tournament_passback_url: String,
    #[serde(rename = "tournamentPassbackDataPacket")]
    pub tournament_passback_data_packet: String,
    #[serde(rename = "gameServerRegion")]
    pub game_server_region: String,
    #[serde(rename = "spectatorDelayEnabled")]
    pub spectator_delay_enabled: bool,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyLobbyCustomGameLobby {
    #[serde(rename = "lobbyName")]
    pub lobby_name: String,
    #[serde(rename = "lobbyPassword")]
    pub lobby_password: String,
    pub configuration: LolLobbyLobbyCustomGameConfiguration,
    #[serde(rename = "teamOne")]
    pub team_one: Vec<LolLobbyLobbyMember>,
    #[serde(rename = "teamTwo")]
    pub team_two: Vec<LolLobbyLobbyMember>,
    pub spectators: Vec<LolLobbyLobbyMember>,
    #[serde(rename = "practiceGameRewardsDisabledReasons")]
    pub practice_game_rewards_disabled_reasons: Vec<String>,
    #[serde(rename = "gameId")]
    pub game_id: u64,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyLobbyCustomJoinParameters {
    pub password: Option<String>,
    #[serde(rename = "asSpectator")]
    pub as_spectator: Option<bool>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyLobbyDto {
    #[serde(rename = "partyId")]
    pub party_id: String,
    #[serde(rename = "partyType")]
    pub party_type: String,
    pub members: Vec<LolLobbyLobbyParticipantDto>,
    #[serde(rename = "localMember")]
    pub local_member: LolLobbyLobbyParticipantDto,
    pub invitations: Vec<LolLobbyLobbyInvitationDto>,
    #[serde(rename = "canStartActivity")]
    pub can_start_activity: bool,
    pub restrictions: Option<Vec<LolLobbyEligibilityRestriction>>,
    pub warnings: Option<Vec<LolLobbyEligibilityRestriction>>,
    #[serde(rename = "gameConfig")]
    pub game_config: LolLobbyLobbyGameConfigDto,
    #[serde(rename = "multiUserChatId")]
    pub multi_user_chat_id: String,
    #[serde(rename = "multiUserChatPassword")]
    pub multi_user_chat_password: String,
    #[serde(rename = "mucJwtDto")]
    pub muc_jwt_dto: LolLobbyMucJwtDto,
    #[serde(rename = "scarcePositions")]
    pub scarce_positions: Vec<String>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyLobbyGameConfigDto {
    #[serde(rename = "gameMode")]
    pub game_mode: String,
    #[serde(rename = "mapId")]
    pub map_id: i32,
    #[serde(rename = "queueId")]
    pub queue_id: i32,
    #[serde(rename = "pickType")]
    pub pick_type: String,
    #[serde(rename = "maxTeamSize")]
    pub max_team_size: i32,
    #[serde(rename = "maxLobbySize")]
    pub max_lobby_size: i32,
    #[serde(rename = "maxHumanPlayers")]
    pub max_human_players: i32,
    #[serde(rename = "allowablePremadeSizes")]
    pub allowable_premade_sizes: Vec<i32>,
    #[serde(rename = "premadeSizeAllowed")]
    pub premade_size_allowed: bool,
    #[serde(rename = "isTeamBuilderManaged")]
    pub is_team_builder_managed: bool,
    #[serde(rename = "isCustom")]
    pub is_custom: bool,
    #[serde(rename = "showPositionSelector")]
    pub show_position_selector: bool,
    #[serde(rename = "showQuickPlaySlotSelection")]
    pub show_quick_play_slot_selection: bool,
    #[serde(rename = "isLobbyFull")]
    pub is_lobby_full: bool,
    #[serde(rename = "shouldForceScarcePositionSelection")]
    pub should_force_scarce_position_selection: bool,
    #[serde(rename = "customLobbyName")]
    pub custom_lobby_name: String,
    #[serde(rename = "customMutatorName")]
    pub custom_mutator_name: String,
    #[serde(rename = "customTeam100")]
    pub custom_team_100: Vec<LolLobbyLobbyParticipantDto>,
    #[serde(rename = "customTeam200")]
    pub custom_team_200: Vec<LolLobbyLobbyParticipantDto>,
    #[serde(rename = "customSpectators")]
    pub custom_spectators: Vec<LolLobbyLobbyParticipantDto>,
    #[serde(rename = "customSpectatorPolicy")]
    pub custom_spectator_policy: LolLobbyQueueCustomGameSpectatorPolicy,
    #[serde(rename = "customRewardsDisabledReasons")]
    pub custom_rewards_disabled_reasons: Vec<String>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyLobbyInvitation {
    pub id: String,
    #[serde(rename = "fromSummonerId")]
    pub from_summoner_id: u64,
    #[serde(rename = "toSummonerId")]
    pub to_summoner_id: u64,
    pub state: LolLobbyLobbyInvitationState,
    #[serde(rename = "errorType")]
    pub error_type: String,
    pub timestamp: String,
    #[serde(rename = "invitationMetaData")]
    pub invitation_meta_data: Value,
    pub eligibility: LolLobbyEligibility,
    #[serde(rename = "fromSummonerName")]
    pub from_summoner_name: String,
    #[serde(rename = "toSummonerName")]
    pub to_summoner_name: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyLobbyInvitationDto {
    #[serde(rename = "invitationId")]
    pub invitation_id: String,
    #[serde(rename = "toSummonerId")]
    pub to_summoner_id: u64,
    pub state: LolLobbyLobbyInvitationState,
    pub timestamp: String,
    #[serde(rename = "toSummonerName")]
    pub to_summoner_name: String,
    #[serde(rename = "invitationType")]
    pub invitation_type: LolLobbyInvitationType,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyLobbyMatchmakingLowPriorityDataResource {
    #[serde(rename = "penalizedSummonerIds")]
    pub penalized_summoner_ids: Vec<u64>,
    #[serde(rename = "penaltyTime")]
    pub penalty_time: f64,
    #[serde(rename = "penaltyTimeRemaining")]
    pub penalty_time_remaining: f64,
    #[serde(rename = "bustedLeaverAccessToken")]
    pub busted_leaver_access_token: String,
    pub reason: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyLobbyMatchmakingSearchErrorResource {
    pub id: i32,
    #[serde(rename = "errorType")]
    pub error_type: String,
    #[serde(rename = "penalizedSummonerId")]
    pub penalized_summoner_id: u64,
    #[serde(rename = "penaltyTimeRemaining")]
    pub penalty_time_remaining: f64,
    pub message: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyLobbyMatchmakingSearchResource {
    #[serde(rename = "searchState")]
    pub search_state: LolLobbyLobbyMatchmakingSearchState,
    #[serde(rename = "lowPriorityData")]
    pub low_priority_data: LolLobbyLobbyMatchmakingLowPriorityDataResource,
    pub errors: Vec<LolLobbyLobbyMatchmakingSearchErrorResource>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyLobbyMember {
    pub id: u64,
    #[serde(rename = "isOwner")]
    pub is_owner: bool,
    #[serde(rename = "isSpectator")]
    pub is_spectator: bool,
    #[serde(rename = "canInviteOthers")]
    pub can_invite_others: bool,
    #[serde(rename = "positionPreferences")]
    pub position_preferences: LolLobbyLobbyPositionPreferences,
    #[serde(rename = "excludedPositionPreference")]
    pub excluded_position_preference: Option<String>,
    #[serde(rename = "summonerInternalName")]
    pub summoner_internal_name: String,
    #[serde(rename = "showPositionExcluder")]
    pub show_position_excluder: bool,
    #[serde(rename = "autoFillEligible")]
    pub auto_fill_eligible: bool,
    #[serde(rename = "autoFillProtectedForStreaking")]
    pub auto_fill_protected_for_streaking: bool,
    #[serde(rename = "autoFillProtectedForPromos")]
    pub auto_fill_protected_for_promos: bool,
    #[serde(rename = "autoFillProtectedForSoloing")]
    pub auto_fill_protected_for_soloing: bool,
    #[serde(rename = "isBot")]
    pub is_bot: bool,
    #[serde(rename = "botDifficulty")]
    pub bot_difficulty: LolLobbyLobbyBotDifficulty,
    #[serde(rename = "botChampionId")]
    pub bot_champion_id: i32,
    pub position: String,
    #[serde(rename = "botUuid")]
    pub bot_uuid: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyLobbyNotification {
    #[serde(rename = "notificationId")]
    pub notification_id: String,
    #[serde(rename = "notificationReason")]
    pub notification_reason: String,
    #[serde(rename = "summonerIds")]
    pub summoner_ids: Vec<u64>,
    pub timestamp: u64,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyLobbyParticipantDto {
    #[serde(rename = "summonerId")]
    pub summoner_id: u64,
    #[serde(rename = "summonerIconId")]
    pub summoner_icon_id: i32,
    #[serde(rename = "summonerName")]
    pub summoner_name: String,
    #[serde(rename = "summonerInternalName")]
    pub summoner_internal_name: String,
    pub puuid: String,
    #[serde(rename = "summonerLevel")]
    pub summoner_level: u32,
    #[serde(rename = "allowedStartActivity")]
    pub allowed_start_activity: bool,
    #[serde(rename = "allowedChangeActivity")]
    pub allowed_change_activity: bool,
    #[serde(rename = "allowedToggleInvite")]
    pub allowed_toggle_invite: bool,
    #[serde(rename = "allowedKickOthers")]
    pub allowed_kick_others: bool,
    #[serde(rename = "allowedInviteOthers")]
    pub allowed_invite_others: bool,
    #[serde(rename = "isLeader")]
    pub is_leader: bool,
    #[serde(rename = "isSpectator")]
    pub is_spectator: bool,
    #[serde(rename = "teamId")]
    pub team_id: i32,
    #[serde(rename = "firstPositionPreference")]
    pub first_position_preference: String,
    #[serde(rename = "secondPositionPreference")]
    pub second_position_preference: String,
    #[serde(rename = "subteamIndex")]
    pub subteam_index: Option<i8>,
    #[serde(rename = "intraSubteamPosition")]
    pub intra_subteam_position: Option<i8>,
    #[serde(rename = "tftNPEQueueBypass")]
    pub tft_npe_queue_bypass: Option<bool>,
    #[serde(rename = "quickplayPlayerState")]
    pub quickplay_player_state: Option<String>,
    #[serde(rename = "strawberryMapId")]
    pub strawberry_map_id: Option<String>,
    #[serde(rename = "playerSlots")]
    pub player_slots: Vec<LolLobbyQuickPlayPresetSlotDto>,
    pub ready: bool,
    #[serde(rename = "showGhostedBanner")]
    pub show_ghosted_banner: bool,
    #[serde(rename = "autoFillEligible")]
    pub auto_fill_eligible: bool,
    #[serde(rename = "autoFillProtectedForStreaking")]
    pub auto_fill_protected_for_streaking: bool,
    #[serde(rename = "autoFillProtectedForPromos")]
    pub auto_fill_protected_for_promos: bool,
    #[serde(rename = "autoFillProtectedForSoloing")]
    pub auto_fill_protected_for_soloing: bool,
    #[serde(rename = "autoFillProtectedForRemedy")]
    pub auto_fill_protected_for_remedy: bool,
    #[serde(rename = "isBot")]
    pub is_bot: bool,
    #[serde(rename = "botId")]
    pub bot_id: String,
    #[serde(rename = "botDifficulty")]
    pub bot_difficulty: LolLobbyLobbyBotDifficulty,
    #[serde(rename = "botChampionId")]
    pub bot_champion_id: i32,
    #[serde(rename = "botPosition")]
    pub bot_position: String,
    #[serde(rename = "botUuid")]
    pub bot_uuid: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyLobbyPartyRewards {
    #[serde(rename = "isEnabled")]
    pub is_enabled: bool,
    #[serde(rename = "queueId")]
    pub queue_id: i32,
    #[serde(rename = "isCustom")]
    pub is_custom: bool,
    #[serde(rename = "partyRewards")]
    pub party_rewards: Vec<LolLobbyPartyReward>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyLobbyPositionPreferences {
    #[serde(rename = "firstPreference")]
    pub first_preference: String,
    #[serde(rename = "secondPreference")]
    pub second_preference: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyMucJwtDto {
    pub jwt: String,
    #[serde(rename = "channelClaim")]
    pub channel_claim: String,
    pub domain: String,
    #[serde(rename = "targetRegion")]
    pub target_region: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyPartyChatDto {
    pub jid: String,
    #[serde(rename = "mucJwtDto")]
    pub muc_jwt_dto: LolLobbyMucJwtDto,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyPartyDto {
    #[serde(rename = "partyId")]
    pub party_id: String,
    #[serde(rename = "platformId")]
    pub platform_id: String,
    pub players: Vec<LolLobbyPartyMemberDto>,
    pub chat: LolLobbyPartyChatDto,
    #[serde(rename = "maxPartySize")]
    pub max_party_size: i32,
    #[serde(rename = "partyType")]
    pub party_type: String,
    #[serde(rename = "gameMode")]
    pub game_mode: LolLobbyGameModeDto,
    #[serde(rename = "activityLocked")]
    pub activity_locked: bool,
    pub version: u64,
    #[serde(rename = "activityStartedUtcMillis")]
    pub activity_started_utc_millis: u64,
    #[serde(rename = "activityResumeUtcMillis")]
    pub activity_resume_utc_millis: u64,
    #[serde(rename = "activeRestrictions")]
    pub active_restrictions: LolLobbyQueueRestrictionDto,
    #[serde(rename = "eligibilityHash")]
    pub eligibility_hash: i64,
    #[serde(rename = "eligibilityRestrictions")]
    pub eligibility_restrictions: Option<Vec<LolLobbyGatekeeperRestrictionDto>>,
    #[serde(rename = "eligibilityWarnings")]
    pub eligibility_warnings: Option<Vec<LolLobbyGatekeeperRestrictionDto>>,
    #[serde(rename = "botParticipants")]
    pub bot_participants: Option<Vec<LolLobbyBotParticipantDto>>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyPartyMemberDto {
    #[serde(rename = "platformId")]
    pub platform_id: String,
    pub puuid: String,
    #[serde(rename = "accountId")]
    pub account_id: u64,
    #[serde(rename = "summonerId")]
    pub summoner_id: u64,
    #[serde(rename = "partyId")]
    pub party_id: String,
    #[serde(rename = "partyVersion")]
    pub party_version: i64,
    pub role: LolLobbyPartyMemberRoleEnum,
    #[serde(rename = "gameMode")]
    pub game_mode: Option<LolLobbyGameModeDto>,
    pub ready: Option<bool>,
    pub metadata: LolLobbyPartyMemberMetadataDto,
    #[serde(rename = "invitedBySummonerId")]
    pub invited_by_summoner_id: Option<u64>,
    #[serde(rename = "inviteTimestamp")]
    pub invite_timestamp: Option<u64>,
    #[serde(rename = "canInvite")]
    pub can_invite: Option<bool>,
    pub team: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyPartyMemberMetadataDto {
    #[serde(rename = "positionPref")]
    pub position_pref: Vec<String>,
    pub properties: Option<Value>,
    #[serde(rename = "playerSlots")]
    pub player_slots: Vec<LolLobbyQuickPlayPresetSlotDto>,
    #[serde(rename = "subteamData")]
    pub subteam_data: Option<LolLobbySubteamDataDto>,
    #[serde(rename = "tftNPEQueueBypass")]
    pub tft_npe_queue_bypass: Option<bool>,
    #[serde(rename = "quickplayPlayerState")]
    pub quickplay_player_state: Option<String>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyPartyReward {
    #[serde(rename = "premadeSize")]
    pub premade_size: i32,
    #[serde(rename = "type")]
    pub type_: LolLobbyLobbyPartyRewardType,
    pub value: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyPartyStatusDto {
    #[serde(rename = "readyPlayers")]
    pub ready_players: Vec<String>,
    #[serde(rename = "leftPlayers")]
    pub left_players: Vec<String>,
    #[serde(rename = "eogPlayers")]
    pub eog_players: Vec<String>,
    #[serde(rename = "partySize")]
    pub party_size: i32,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyPlayerDto {
    pub puuid: String,
    #[serde(rename = "platformId")]
    pub platform_id: String,
    #[serde(rename = "accountId")]
    pub account_id: u64,
    #[serde(rename = "summonerId")]
    pub summoner_id: u64,
    #[serde(rename = "eligibilityHash")]
    pub eligibility_hash: i64,
    #[serde(rename = "serverUtcMillis")]
    pub server_utc_millis: i64,
    pub parties: Option<Vec<LolLobbyPartyMemberDto>>,
    #[serde(rename = "currentParty")]
    pub current_party: Option<LolLobbyPartyDto>,
    #[serde(rename = "tftGamesPlayed")]
    pub tft_games_played: i64,
    #[serde(rename = "tftGamesWon")]
    pub tft_games_won: i64,
    pub registration: LolLobbyRegistrationCredentials,
    #[serde(rename = "createdAt")]
    pub created_at: u64,
    pub version: u64,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyPremadeMemberDto {
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(rename = "gameName")]
    pub game_name: String,
    #[serde(rename = "tagLine")]
    pub tag_line: String,
    pub puuid: String,
    #[serde(rename = "partyId")]
    pub party_id: String,
    #[serde(rename = "summonerId")]
    pub summoner_id: u64,
    pub role: LolLobbyPartyMemberRoleEnum,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyPremadePartyDto {
    #[serde(rename = "partyId")]
    pub party_id: String,
    #[serde(rename = "commsEnabled")]
    pub comms_enabled: bool,
    pub players: HashMap<String, LolLobbyPremadeMemberDto>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyQueueGameTypeConfig {
    pub id: i64,
    pub name: String,
    #[serde(rename = "maxAllowableBans")]
    pub max_allowable_bans: i32,
    #[serde(rename = "allowTrades")]
    pub allow_trades: bool,
    #[serde(rename = "exclusivePick")]
    pub exclusive_pick: bool,
    #[serde(rename = "duplicatePick")]
    pub duplicate_pick: bool,
    #[serde(rename = "teamChampionPool")]
    pub team_champion_pool: bool,
    #[serde(rename = "crossTeamChampionPool")]
    pub cross_team_champion_pool: bool,
    #[serde(rename = "advancedLearningQuests")]
    pub advanced_learning_quests: bool,
    #[serde(rename = "battleBoost")]
    pub battle_boost: bool,
    #[serde(rename = "deathMatch")]
    pub death_match: bool,
    #[serde(rename = "doNotRemove")]
    pub do_not_remove: bool,
    #[serde(rename = "learningQuests")]
    pub learning_quests: bool,
    #[serde(rename = "onboardCoopBeginner")]
    pub onboard_coop_beginner: bool,
    pub reroll: bool,
    #[serde(rename = "mainPickTimerDuration")]
    pub main_pick_timer_duration: i32,
    #[serde(rename = "postPickTimerDuration")]
    pub post_pick_timer_duration: i32,
    #[serde(rename = "banTimerDuration")]
    pub ban_timer_duration: i32,
    #[serde(rename = "pickMode")]
    pub pick_mode: String,
    #[serde(rename = "banMode")]
    pub ban_mode: String,
    #[serde(rename = "gameModeOverride")]
    pub game_mode_override: Option<String>,
    #[serde(rename = "numPlayersPerTeamOverride")]
    pub num_players_per_team_override: Option<i32>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyQueueRestrictionDto {
    #[serde(rename = "gatekeeperRestrictions")]
    pub gatekeeper_restrictions: Vec<LolLobbyGatekeeperRestrictionDto>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyQuickPlayPresetSlotDto {
    #[serde(rename = "championId")]
    pub champion_id: i32,
    #[serde(rename = "skinId")]
    pub skin_id: i32,
    #[serde(rename = "positionPreference")]
    pub position_preference: String,
    pub perks: String,
    #[serde(rename = "spell1")]
    pub spell_1: u64,
    #[serde(rename = "spell2")]
    pub spell_2: u64,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyReceivedInvitationDto {
    #[serde(rename = "invitationId")]
    pub invitation_id: String,
    #[serde(rename = "fromSummonerId")]
    pub from_summoner_id: u64,
    pub state: LolLobbyLobbyInvitationState,
    pub timestamp: String,
    #[serde(rename = "fromSummonerName")]
    pub from_summoner_name: String,
    #[serde(rename = "canAcceptInvitation")]
    pub can_accept_invitation: bool,
    pub restrictions: Vec<LolLobbyEligibilityRestriction>,
    #[serde(rename = "gameConfig")]
    pub game_config: LolLobbyReceivedInvitationGameConfigDto,
    #[serde(rename = "invitationType")]
    pub invitation_type: LolLobbyInvitationType,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyReceivedInvitationGameConfigDto {
    #[serde(rename = "gameMode")]
    pub game_mode: String,
    #[serde(rename = "queueId")]
    pub queue_id: i32,
    #[serde(rename = "mapId")]
    pub map_id: i32,
    #[serde(rename = "inviteGameType")]
    pub invite_game_type: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyRegistrationCredentials {
    #[serde(rename = "summonerToken")]
    pub summoner_token: Option<String>,
    #[serde(rename = "inventoryToken")]
    pub inventory_token: Option<String>,
    #[serde(rename = "inventoryTokens")]
    pub inventory_tokens: Option<Vec<String>>,
    #[serde(rename = "simpleInventoryToken")]
    pub simple_inventory_token: Option<String>,
    #[serde(rename = "rankedOverviewToken")]
    pub ranked_overview_token: Option<String>,
    #[serde(rename = "gameClientVersion")]
    pub game_client_version: Option<String>,
    #[serde(rename = "playerTokens")]
    pub player_tokens: Option<HashMap<String, String>>,
    pub experiments: Option<HashMap<String, String>>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyStrawberryMapUpdateDto {
    #[serde(rename = "contentId")]
    pub content_id: String,
    #[serde(rename = "itemId")]
    pub item_id: i32,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbySubteamDataDto {
    #[serde(rename = "subteamIndex")]
    pub subteam_index: i8,
    #[serde(rename = "intraSubteamPosition")]
    pub intra_subteam_position: i8,
}


// ENUMS

#[derive(Serialize, Deserialize, Clone, PartialEq, Default, Debug)]
pub enum LolLobbyEligibilityRestrictionCode {
    #[default]
    MmrStandardDeviationTooLarge,
    UserInfoNotAvailable,
    InventoryQueuesInfoNotAvailable,
    InventoryChampsInfoNotAvailable,
    LeaguesInfoNotAvailable,
    SummonerInfoNotAvailable,
    MinorInfoNotAvailable,
    BanInfoNotAvailable,
    TooManyIncompleteSubteamsRestriction,
    #[serde(rename = "QPScarcePositionsNotAvailableRestriction")]
    QpScarcePositionsNotAvailableRestriction,
    #[serde(rename = "QPNonUniquePrimarySlotRestriction")]
    QpNonUniquePrimarySlotRestriction,
    #[serde(rename = "QPInvalidChampionSelectionRestriction")]
    QpInvalidChampionSelectionRestriction,
    #[serde(rename = "QPInvalidPositionSelectionRestriction")]
    QpInvalidPositionSelectionRestriction,
    #[serde(rename = "QPInvalidNumberOfPlayerSlotsRestriction")]
    QpInvalidNumberOfPlayerSlotsRestriction,
    #[serde(rename = "QPPlayerChampionCoverageRestriction")]
    QpPlayerChampionCoverageRestriction,
    #[serde(rename = "QPPartyChampionCoverageRestriction")]
    QpPartyChampionCoverageRestriction,
    #[serde(rename = "QPPlayerPositionCoverageRestriction")]
    QpPlayerPositionCoverageRestriction,
    #[serde(rename = "QPPartyPositionCoverageRestriction")]
    QpPartyPositionCoverageRestriction,
    #[serde(rename = "QPPlayerScarcePositionCoverageRestriction")]
    QpPlayerScarcePositionCoverageRestriction,
    MinNormalGamesForRankedRestriction,
    #[serde(rename = "LOLNewPlayerRestriction")]
    LolNewPlayerRestriction,
    UnknownRestriction,
    SeasonVersionLockout,
    #[serde(rename = "TFTNewPlayerRestriction")]
    TftNewPlayerRestriction,
    QueueEntryNotEntitledRestriction,
    GameVersionNotSupported,
    GameVersionMissing,
    GameVersionMismatch,
    PrerequisiteQueuesNotPlayedRestriction,
    TeamSizeRestriction,
    #[serde(rename = "TeamHighMMRMaxSizeRestriction")]
    TeamHighMmrMaxSizeRestriction,
    PlayerRankedSuspensionRestriction,
    PlayerRankSoloOnlyRestriction,
    PlayerTimePlayedRestriction,
    PlayerMinorRestriction,
    PlayerMinLevelRestriction,
    PlayerMaxLevelRestriction,
    PlayerTimeBasedRankRestriction,
    PlayerGameBasedRankRestriction,
    PlayerLeaverTaintedWarningRestriction,
    PlayerLeaverQueueLockoutRestriction,
    PlayerLeaverBustedRestriction,
    PlayerInGameRestriction,
    PlayerDodgeRestriction,
    PlayerBingeRestriction,
    TeamMinSizeRestriction,
    TeamMaxSizeRestriction,
    TeamSkillRestriction,
    TeamDivisionRestriction,
    PlayerAvailableChampionRestriction,
    PlayerBannedRestriction,
    PlayerTimedRestriction,
    PlayerLevelRestriction,
    QueueUnsupported,
    QueueDisabled,
}


#[derive(Serialize, Deserialize, Clone, PartialEq, Default, Debug)]
pub enum LolLobbyInvitationType {
    #[default]
    #[serde(rename = "party")]
    Party,
    #[serde(rename = "lobby")]
    Lobby,
    #[serde(rename = "invalid")]
    Invalid,
}


#[derive(Serialize, Deserialize, Clone, PartialEq, Default, Debug)]
pub enum LolLobbyLobbyBotDifficulty {
    #[default]
    #[serde(rename = "RSINTERMEDIATE")]
    Rsintermediate,
    #[serde(rename = "RSBEGINNER")]
    Rsbeginner,
    #[serde(rename = "RSINTRO")]
    Rsintro,
    #[serde(rename = "INTRO")]
    Intro,
    #[serde(rename = "TUTORIAL")]
    Tutorial,
    #[serde(rename = "UBER")]
    Uber,
    #[serde(rename = "HARD")]
    Hard,
    #[serde(rename = "MEDIUM")]
    Medium,
    #[serde(rename = "EASY")]
    Easy,
    #[serde(rename = "NONE")]
    None,
}


#[derive(Serialize, Deserialize, Clone, PartialEq, Default, Debug)]
pub enum LolLobbyLobbyInvitationState {
    #[default]
    Error,
    OnHold,
    Kicked,
    Declined,
    Joined,
    Accepted,
    Pending,
    Requested,
}


#[derive(Serialize, Deserialize, Clone, PartialEq, Default, Debug)]
pub enum LolLobbyLobbyMatchmakingSearchState {
    #[default]
    ServiceShutdown,
    ServiceError,
    Error,
    Found,
    Searching,
    Canceled,
    AbandonedLowPriorityQueue,
    Invalid,
}


#[derive(Serialize, Deserialize, Clone, PartialEq, Default, Debug)]
pub enum LolLobbyLobbyPartyRewardType {
    #[default]
    None,
    Icon,
    Ip,
}


#[derive(Serialize, Deserialize, Clone, PartialEq, Default, Debug)]
pub enum LolLobbyPartyMemberRoleEnum {
    #[default]
    #[serde(rename = "NONE")]
    None,
    #[serde(rename = "DECLINED")]
    Declined,
    #[serde(rename = "KICKED")]
    Kicked,
    #[serde(rename = "HOLD")]
    Hold,
    #[serde(rename = "INVITED")]
    Invited,
    #[serde(rename = "MEMBER")]
    Member,
    #[serde(rename = "LEADER")]
    Leader,
}


#[derive(Serialize, Deserialize, Clone, PartialEq, Default, Debug)]
pub enum LolLobbyQueueAvailability {
    #[default]
    DoesntMeetRequirements,
    PlatformDisabled,
    Available,
}


#[derive(Serialize, Deserialize, Clone, PartialEq, Default, Debug)]
pub enum LolLobbyQueueCustomGameSpectatorPolicy {
    #[default]
    AllAllowed,
    FriendsAllowed,
    LobbyAllowed,
    NotAllowed,
}

