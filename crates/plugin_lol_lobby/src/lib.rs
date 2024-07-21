mod enum_impl;

use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use serde_json::{json, Value, to_value};
use reqwest::Method;
use common::IsApiRequest;

// ENDPOINTS

pub struct PostLolLobbyV1Clash {

    pub body: String,
}

impl IsApiRequest for PostLolLobbyV1Clash {
    const METHOD: Method = Method::POST;
    type ReturnType = HashMap<String, String>;

    fn get_url(&self) -> String {
        "/lol-lobby/v1/clash".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn post_lol_lobby_v_1_clash(body: String) -> PostLolLobbyV1Clash {
    PostLolLobbyV1Clash {
        body
    }
}


pub struct DeleteLolLobbyV1Clash {

}

impl IsApiRequest for DeleteLolLobbyV1Clash {
    const METHOD: Method = Method::DELETE;
    type ReturnType = HashMap<String, String>;

    fn get_url(&self) -> String {
        "/lol-lobby/v1/clash".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn delete_lol_lobby_v_1_clash() -> DeleteLolLobbyV1Clash {
    DeleteLolLobbyV1Clash {
        
    }
}


pub struct DeleteLolLobbyV1LobbyCustomBotsBySummonerInternalNameByBotUuidToDeleteByTeamId {

    pub summoner_internal_name: String,
    pub bot_uuid_to_delete: String,
    pub team_id: String,
}

impl IsApiRequest for DeleteLolLobbyV1LobbyCustomBotsBySummonerInternalNameByBotUuidToDeleteByTeamId {
    const METHOD: Method = Method::DELETE;
    type ReturnType = HashMap<String, String>;

    fn get_url(&self) -> String {
        format!("/lol-lobby/v1/lobby/custom/bots/{}/{}/{}", self.summoner_internal_name, self.bot_uuid_to_delete, self.team_id)
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn delete_lol_lobby_v_1_lobby_custom_bots_by_summoner_internal_name_by_bot_uuid_to_delete_by_team_id(summoner_internal_name: String, bot_uuid_to_delete: String, team_id: String) -> DeleteLolLobbyV1LobbyCustomBotsBySummonerInternalNameByBotUuidToDeleteByTeamId {
    DeleteLolLobbyV1LobbyCustomBotsBySummonerInternalNameByBotUuidToDeleteByTeamId {
        summoner_internal_name, bot_uuid_to_delete, team_id
    }
}


pub struct GetLolLobbyV2Lobby {

}

impl IsApiRequest for GetLolLobbyV2Lobby {
    const METHOD: Method = Method::GET;
    type ReturnType = LolLobbyLobbyDto;

    fn get_url(&self) -> String {
        "/lol-lobby/v2/lobby".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_lobby_v_2_lobby() -> GetLolLobbyV2Lobby {
    GetLolLobbyV2Lobby {
        
    }
}


pub struct PostLolLobbyV2Lobby {

    pub body: LolLobbyLobbyChangeGameDto,
}

impl IsApiRequest for PostLolLobbyV2Lobby {
    const METHOD: Method = Method::POST;
    type ReturnType = LolLobbyLobbyDto;

    fn get_url(&self) -> String {
        "/lol-lobby/v2/lobby".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn post_lol_lobby_v_2_lobby(body: LolLobbyLobbyChangeGameDto) -> PostLolLobbyV2Lobby {
    PostLolLobbyV2Lobby {
        body
    }
}


pub struct DeleteLolLobbyV2Lobby {

}

impl IsApiRequest for DeleteLolLobbyV2Lobby {
    const METHOD: Method = Method::DELETE;
    type ReturnType = Value;

    fn get_url(&self) -> String {
        "/lol-lobby/v2/lobby".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn delete_lol_lobby_v_2_lobby() -> DeleteLolLobbyV2Lobby {
    DeleteLolLobbyV2Lobby {
        
    }
}


pub struct PostLolLobbyV2LobbyMatchmakingSearch {

}

impl IsApiRequest for PostLolLobbyV2LobbyMatchmakingSearch {
    const METHOD: Method = Method::POST;
    type ReturnType = HashMap<String, String>;

    fn get_url(&self) -> String {
        "/lol-lobby/v2/lobby/matchmaking/search".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn post_lol_lobby_v_2_lobby_matchmaking_search() -> PostLolLobbyV2LobbyMatchmakingSearch {
    PostLolLobbyV2LobbyMatchmakingSearch {
        
    }
}


pub struct DeleteLolLobbyV2LobbyMatchmakingSearch {

}

impl IsApiRequest for DeleteLolLobbyV2LobbyMatchmakingSearch {
    const METHOD: Method = Method::DELETE;
    type ReturnType = HashMap<String, String>;

    fn get_url(&self) -> String {
        "/lol-lobby/v2/lobby/matchmaking/search".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn delete_lol_lobby_v_2_lobby_matchmaking_search() -> DeleteLolLobbyV2LobbyMatchmakingSearch {
    DeleteLolLobbyV2LobbyMatchmakingSearch {
        
    }
}


pub struct DeleteLolLobbyV2NotificationsByNotificationId {

    pub notification_id: String,
}

impl IsApiRequest for DeleteLolLobbyV2NotificationsByNotificationId {
    const METHOD: Method = Method::DELETE;
    type ReturnType = HashMap<String, String>;

    fn get_url(&self) -> String {
        format!("/lol-lobby/v2/notifications/{}", self.notification_id)
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn delete_lol_lobby_v_2_notifications_by_notification_id(notification_id: String) -> DeleteLolLobbyV2NotificationsByNotificationId {
    DeleteLolLobbyV2NotificationsByNotificationId {
        notification_id
    }
}


pub struct GetLolLobbyV1AutofillDisplayed {

}

impl IsApiRequest for GetLolLobbyV1AutofillDisplayed {
    const METHOD: Method = Method::GET;
    type ReturnType = bool;

    fn get_url(&self) -> String {
        "/lol-lobby/v1/autofill-displayed".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_lobby_v_1_autofill_displayed() -> GetLolLobbyV1AutofillDisplayed {
    GetLolLobbyV1AutofillDisplayed {
        
    }
}


pub struct PutLolLobbyV1AutofillDisplayed {

}

impl IsApiRequest for PutLolLobbyV1AutofillDisplayed {
    const METHOD: Method = Method::PUT;
    type ReturnType = bool;

    fn get_url(&self) -> String {
        "/lol-lobby/v1/autofill-displayed".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn put_lol_lobby_v_1_autofill_displayed() -> PutLolLobbyV1AutofillDisplayed {
    PutLolLobbyV1AutofillDisplayed {
        
    }
}


pub struct GetLolLobbyV1CustomGames {

}

impl IsApiRequest for GetLolLobbyV1CustomGames {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<LolLobbyLobbyCustomGame>;

    fn get_url(&self) -> String {
        "/lol-lobby/v1/custom-games".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_lobby_v_1_custom_games() -> GetLolLobbyV1CustomGames {
    GetLolLobbyV1CustomGames {
        
    }
}


pub struct GetLolLobbyV1CustomGamesById {

    pub id: i32,
}

impl IsApiRequest for GetLolLobbyV1CustomGamesById {
    const METHOD: Method = Method::GET;
    type ReturnType = LolLobbyLobbyCustomGame;

    fn get_url(&self) -> String {
        format!("/lol-lobby/v1/custom-games/{}", self.id)
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_lobby_v_1_custom_games_by_id(id: i32) -> GetLolLobbyV1CustomGamesById {
    GetLolLobbyV1CustomGamesById {
        id
    }
}


pub struct GetLolLobbyV1LobbyAvailability {

}

impl IsApiRequest for GetLolLobbyV1LobbyAvailability {
    const METHOD: Method = Method::GET;
    type ReturnType = LolLobbyQueueAvailability;

    fn get_url(&self) -> String {
        "/lol-lobby/v1/lobby/availability".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_lobby_v_1_lobby_availability() -> GetLolLobbyV1LobbyAvailability {
    GetLolLobbyV1LobbyAvailability {
        
    }
}


pub struct GetLolLobbyV1LobbyCountdown {

}

impl IsApiRequest for GetLolLobbyV1LobbyCountdown {
    const METHOD: Method = Method::GET;
    type ReturnType = i64;

    fn get_url(&self) -> String {
        "/lol-lobby/v1/lobby/countdown".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_lobby_v_1_lobby_countdown() -> GetLolLobbyV1LobbyCountdown {
    GetLolLobbyV1LobbyCountdown {
        
    }
}


pub struct GetLolLobbyV1LobbyInvitations {

}

impl IsApiRequest for GetLolLobbyV1LobbyInvitations {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<LolLobbyLobbyInvitation>;

    fn get_url(&self) -> String {
        "/lol-lobby/v1/lobby/invitations".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_lobby_v_1_lobby_invitations() -> GetLolLobbyV1LobbyInvitations {
    GetLolLobbyV1LobbyInvitations {
        
    }
}


pub struct PostLolLobbyV1LobbyInvitations {

    pub body: LolLobbyLobbyInvitation,
}

impl IsApiRequest for PostLolLobbyV1LobbyInvitations {
    const METHOD: Method = Method::POST;
    type ReturnType = LolLobbyLobbyInvitation;

    fn get_url(&self) -> String {
        "/lol-lobby/v1/lobby/invitations".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn post_lol_lobby_v_1_lobby_invitations(body: LolLobbyLobbyInvitation) -> PostLolLobbyV1LobbyInvitations {
    PostLolLobbyV1LobbyInvitations {
        body
    }
}


pub struct GetLolLobbyV1LobbyInvitationsById {

    pub id: String,
}

impl IsApiRequest for GetLolLobbyV1LobbyInvitationsById {
    const METHOD: Method = Method::GET;
    type ReturnType = LolLobbyLobbyInvitation;

    fn get_url(&self) -> String {
        format!("/lol-lobby/v1/lobby/invitations/{}", self.id)
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_lobby_v_1_lobby_invitations_by_id(id: String) -> GetLolLobbyV1LobbyInvitationsById {
    GetLolLobbyV1LobbyInvitationsById {
        id
    }
}


pub struct GetLolLobbyV1LobbyMembersLocalMemberPlayerSlots {

}

impl IsApiRequest for GetLolLobbyV1LobbyMembersLocalMemberPlayerSlots {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<LolLobbyQuickPlayPresetSlotDto>;

    fn get_url(&self) -> String {
        "/lol-lobby/v1/lobby/members/localMember/player-slots".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_lobby_v_1_lobby_members_local_member_player_slots() -> GetLolLobbyV1LobbyMembersLocalMemberPlayerSlots {
    GetLolLobbyV1LobbyMembersLocalMemberPlayerSlots {
        
    }
}


pub struct PutLolLobbyV1LobbyMembersLocalMemberPlayerSlots {

    pub body: Vec<LolLobbyQuickPlayPresetSlotDto>,
}

impl IsApiRequest for PutLolLobbyV1LobbyMembersLocalMemberPlayerSlots {
    const METHOD: Method = Method::PUT;
    type ReturnType = HashMap<String, String>;

    fn get_url(&self) -> String {
        "/lol-lobby/v1/lobby/members/localMember/player-slots".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn put_lol_lobby_v_1_lobby_members_local_member_player_slots(body: Vec<LolLobbyQuickPlayPresetSlotDto>) -> PutLolLobbyV1LobbyMembersLocalMemberPlayerSlots {
    PutLolLobbyV1LobbyMembersLocalMemberPlayerSlots {
        body
    }
}


pub struct GetLolLobbyV1LobbyTftRankedHistory {

}

impl IsApiRequest for GetLolLobbyV1LobbyTftRankedHistory {
    const METHOD: Method = Method::GET;
    type ReturnType = bool;

    fn get_url(&self) -> String {
        "/lol-lobby/v1/lobby/tft-ranked-history".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_lobby_v_1_lobby_tft_ranked_history() -> GetLolLobbyV1LobbyTftRankedHistory {
    GetLolLobbyV1LobbyTftRankedHistory {
        
    }
}


pub struct GetLolLobbyV1PartiesGamemode {

}

impl IsApiRequest for GetLolLobbyV1PartiesGamemode {
    const METHOD: Method = Method::GET;
    type ReturnType = LolLobbyGameModeDto;

    fn get_url(&self) -> String {
        "/lol-lobby/v1/parties/gamemode".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_lobby_v_1_parties_gamemode() -> GetLolLobbyV1PartiesGamemode {
    GetLolLobbyV1PartiesGamemode {
        
    }
}


pub struct GetLolLobbyV1PartiesPlayer {

}

impl IsApiRequest for GetLolLobbyV1PartiesPlayer {
    const METHOD: Method = Method::GET;
    type ReturnType = LolLobbyPlayerDto;

    fn get_url(&self) -> String {
        "/lol-lobby/v1/parties/player".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_lobby_v_1_parties_player() -> GetLolLobbyV1PartiesPlayer {
    GetLolLobbyV1PartiesPlayer {
        
    }
}


pub struct GetLolLobbyV1PartyRewards {

}

impl IsApiRequest for GetLolLobbyV1PartyRewards {
    const METHOD: Method = Method::GET;
    type ReturnType = LolLobbyLobbyPartyRewards;

    fn get_url(&self) -> String {
        "/lol-lobby/v1/party-rewards".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_lobby_v_1_party_rewards() -> GetLolLobbyV1PartyRewards {
    GetLolLobbyV1PartyRewards {
        
    }
}


pub struct GetLolLobbyV2CommsMembers {

}

impl IsApiRequest for GetLolLobbyV2CommsMembers {
    const METHOD: Method = Method::GET;
    type ReturnType = LolLobbyPremadePartyDto;

    fn get_url(&self) -> String {
        "/lol-lobby/v2/comms/members".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_lobby_v_2_comms_members() -> GetLolLobbyV2CommsMembers {
    GetLolLobbyV2CommsMembers {
        
    }
}


pub struct GetLolLobbyV2CommsToken {

}

impl IsApiRequest for GetLolLobbyV2CommsToken {
    const METHOD: Method = Method::GET;
    type ReturnType = String;

    fn get_url(&self) -> String {
        "/lol-lobby/v2/comms/token".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_lobby_v_2_comms_token() -> GetLolLobbyV2CommsToken {
    GetLolLobbyV2CommsToken {
        
    }
}


pub struct GetLolLobbyV2EligibilityGameSelectEligibilityHash {

}

impl IsApiRequest for GetLolLobbyV2EligibilityGameSelectEligibilityHash {
    const METHOD: Method = Method::GET;
    type ReturnType = i64;

    fn get_url(&self) -> String {
        "/lol-lobby/v2/eligibility/game-select-eligibility-hash".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_lobby_v_2_eligibility_game_select_eligibility_hash() -> GetLolLobbyV2EligibilityGameSelectEligibilityHash {
    GetLolLobbyV2EligibilityGameSelectEligibilityHash {
        
    }
}


pub struct GetLolLobbyV2EligibilityInitialConfigurationComplete {

}

impl IsApiRequest for GetLolLobbyV2EligibilityInitialConfigurationComplete {
    const METHOD: Method = Method::GET;
    type ReturnType = bool;

    fn get_url(&self) -> String {
        "/lol-lobby/v2/eligibility/initial-configuration-complete".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_lobby_v_2_eligibility_initial_configuration_complete() -> GetLolLobbyV2EligibilityInitialConfigurationComplete {
    GetLolLobbyV2EligibilityInitialConfigurationComplete {
        
    }
}


pub struct GetLolLobbyV2LobbyCustomAvailableBots {

}

impl IsApiRequest for GetLolLobbyV2LobbyCustomAvailableBots {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<LolLobbyLobbyBotChampion>;

    fn get_url(&self) -> String {
        "/lol-lobby/v2/lobby/custom/available-bots".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_lobby_v_2_lobby_custom_available_bots() -> GetLolLobbyV2LobbyCustomAvailableBots {
    GetLolLobbyV2LobbyCustomAvailableBots {
        
    }
}


pub struct GetLolLobbyV2LobbyCustomBotsEnabled {

}

impl IsApiRequest for GetLolLobbyV2LobbyCustomBotsEnabled {
    const METHOD: Method = Method::GET;
    type ReturnType = bool;

    fn get_url(&self) -> String {
        "/lol-lobby/v2/lobby/custom/bots-enabled".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_lobby_v_2_lobby_custom_bots_enabled() -> GetLolLobbyV2LobbyCustomBotsEnabled {
    GetLolLobbyV2LobbyCustomBotsEnabled {
        
    }
}


pub struct GetLolLobbyV2LobbyInvitations {

}

impl IsApiRequest for GetLolLobbyV2LobbyInvitations {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<LolLobbyLobbyInvitationDto>;

    fn get_url(&self) -> String {
        "/lol-lobby/v2/lobby/invitations".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_lobby_v_2_lobby_invitations() -> GetLolLobbyV2LobbyInvitations {
    GetLolLobbyV2LobbyInvitations {
        
    }
}


pub struct PostLolLobbyV2LobbyInvitations {

    pub body: Vec<LolLobbyLobbyInvitationDto>,
}

impl IsApiRequest for PostLolLobbyV2LobbyInvitations {
    const METHOD: Method = Method::POST;
    type ReturnType = Vec<LolLobbyLobbyInvitationDto>;

    fn get_url(&self) -> String {
        "/lol-lobby/v2/lobby/invitations".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn post_lol_lobby_v_2_lobby_invitations(body: Vec<LolLobbyLobbyInvitationDto>) -> PostLolLobbyV2LobbyInvitations {
    PostLolLobbyV2LobbyInvitations {
        body
    }
}


pub struct GetLolLobbyV2LobbyMatchmakingSearchState {

}

impl IsApiRequest for GetLolLobbyV2LobbyMatchmakingSearchState {
    const METHOD: Method = Method::GET;
    type ReturnType = LolLobbyLobbyMatchmakingSearchResource;

    fn get_url(&self) -> String {
        "/lol-lobby/v2/lobby/matchmaking/search-state".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_lobby_v_2_lobby_matchmaking_search_state() -> GetLolLobbyV2LobbyMatchmakingSearchState {
    GetLolLobbyV2LobbyMatchmakingSearchState {
        
    }
}


pub struct GetLolLobbyV2LobbyMembers {

}

impl IsApiRequest for GetLolLobbyV2LobbyMembers {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<LolLobbyLobbyParticipantDto>;

    fn get_url(&self) -> String {
        "/lol-lobby/v2/lobby/members".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_lobby_v_2_lobby_members() -> GetLolLobbyV2LobbyMembers {
    GetLolLobbyV2LobbyMembers {
        
    }
}


pub struct GetLolLobbyV2Notifications {

}

impl IsApiRequest for GetLolLobbyV2Notifications {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<LolLobbyLobbyNotification>;

    fn get_url(&self) -> String {
        "/lol-lobby/v2/notifications".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_lobby_v_2_notifications() -> GetLolLobbyV2Notifications {
    GetLolLobbyV2Notifications {
        
    }
}


pub struct PostLolLobbyV2Notifications {

    pub body: LolLobbyLobbyNotification,
}

impl IsApiRequest for PostLolLobbyV2Notifications {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;

    fn get_url(&self) -> String {
        "/lol-lobby/v2/notifications".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn post_lol_lobby_v_2_notifications(body: LolLobbyLobbyNotification) -> PostLolLobbyV2Notifications {
    PostLolLobbyV2Notifications {
        body
    }
}


pub struct GetLolLobbyV2PartyActive {

}

impl IsApiRequest for GetLolLobbyV2PartyActive {
    const METHOD: Method = Method::GET;
    type ReturnType = bool;

    fn get_url(&self) -> String {
        "/lol-lobby/v2/party-active".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_lobby_v_2_party_active() -> GetLolLobbyV2PartyActive {
    GetLolLobbyV2PartyActive {
        
    }
}


pub struct GetLolLobbyV2PartyEogStatus {

}

impl IsApiRequest for GetLolLobbyV2PartyEogStatus {
    const METHOD: Method = Method::GET;
    type ReturnType = LolLobbyPartyStatusDto;

    fn get_url(&self) -> String {
        "/lol-lobby/v2/party/eog-status".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_lobby_v_2_party_eog_status() -> GetLolLobbyV2PartyEogStatus {
    GetLolLobbyV2PartyEogStatus {
        
    }
}


pub struct GetLolLobbyV2ReceivedInvitations {

}

impl IsApiRequest for GetLolLobbyV2ReceivedInvitations {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<LolLobbyReceivedInvitationDto>;

    fn get_url(&self) -> String {
        "/lol-lobby/v2/received-invitations".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_lobby_v_2_received_invitations() -> GetLolLobbyV2ReceivedInvitations {
    GetLolLobbyV2ReceivedInvitations {
        
    }
}


pub struct GetLolLobbyV2RegistrationStatus {

}

impl IsApiRequest for GetLolLobbyV2RegistrationStatus {
    const METHOD: Method = Method::GET;
    type ReturnType = HashMap<String, String>;

    fn get_url(&self) -> String {
        "/lol-lobby/v2/registration-status".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_lobby_v_2_registration_status() -> GetLolLobbyV2RegistrationStatus {
    GetLolLobbyV2RegistrationStatus {
        
    }
}


pub struct PostLolLobbyV1CustomGamesByIdJoin {

    pub id: u64,
    pub body: LolLobbyLobbyCustomJoinParameters,
}

impl IsApiRequest for PostLolLobbyV1CustomGamesByIdJoin {
    const METHOD: Method = Method::POST;
    type ReturnType = HashMap<String, String>;

    fn get_url(&self) -> String {
        format!("/lol-lobby/v1/custom-games/{}/join", self.id)
    }

    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn post_lol_lobby_v_1_custom_games_by_id_join(id: u64, body: LolLobbyLobbyCustomJoinParameters) -> PostLolLobbyV1CustomGamesByIdJoin {
    PostLolLobbyV1CustomGamesByIdJoin {
        id, body
    }
}


pub struct PostLolLobbyV1CustomGamesRefresh {

}

impl IsApiRequest for PostLolLobbyV1CustomGamesRefresh {
    const METHOD: Method = Method::POST;
    type ReturnType = HashMap<String, String>;

    fn get_url(&self) -> String {
        "/lol-lobby/v1/custom-games/refresh".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn post_lol_lobby_v_1_custom_games_refresh() -> PostLolLobbyV1CustomGamesRefresh {
    PostLolLobbyV1CustomGamesRefresh {
        
    }
}


pub struct PostLolLobbyV1LobbyCustomBots {

    pub body: LolLobbyLobbyBotParams,
}

impl IsApiRequest for PostLolLobbyV1LobbyCustomBots {
    const METHOD: Method = Method::POST;
    type ReturnType = HashMap<String, String>;

    fn get_url(&self) -> String {
        "/lol-lobby/v1/lobby/custom/bots".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn post_lol_lobby_v_1_lobby_custom_bots(body: LolLobbyLobbyBotParams) -> PostLolLobbyV1LobbyCustomBots {
    PostLolLobbyV1LobbyCustomBots {
        body
    }
}


pub struct PostLolLobbyV1LobbyCustomBotsBySummonerInternalNameByBotUuidToDelete {

    pub summoner_internal_name: String,
    pub bot_uuid_to_delete: String,
    pub body: LolLobbyLobbyBotParams,
}

impl IsApiRequest for PostLolLobbyV1LobbyCustomBotsBySummonerInternalNameByBotUuidToDelete {
    const METHOD: Method = Method::POST;
    type ReturnType = HashMap<String, String>;

    fn get_url(&self) -> String {
        format!("/lol-lobby/v1/lobby/custom/bots/{}/{}", self.summoner_internal_name, self.bot_uuid_to_delete)
    }

    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn post_lol_lobby_v_1_lobby_custom_bots_by_summoner_internal_name_by_bot_uuid_to_delete(summoner_internal_name: String, bot_uuid_to_delete: String, body: LolLobbyLobbyBotParams) -> PostLolLobbyV1LobbyCustomBotsBySummonerInternalNameByBotUuidToDelete {
    PostLolLobbyV1LobbyCustomBotsBySummonerInternalNameByBotUuidToDelete {
        summoner_internal_name, bot_uuid_to_delete, body
    }
}


pub struct PostLolLobbyV1LobbyCustomCancelChampSelect {

}

impl IsApiRequest for PostLolLobbyV1LobbyCustomCancelChampSelect {
    const METHOD: Method = Method::POST;
    type ReturnType = HashMap<String, String>;

    fn get_url(&self) -> String {
        "/lol-lobby/v1/lobby/custom/cancel-champ-select".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn post_lol_lobby_v_1_lobby_custom_cancel_champ_select() -> PostLolLobbyV1LobbyCustomCancelChampSelect {
    PostLolLobbyV1LobbyCustomCancelChampSelect {
        
    }
}


pub struct PostLolLobbyV1LobbyCustomStartChampSelect {

}

impl IsApiRequest for PostLolLobbyV1LobbyCustomStartChampSelect {
    const METHOD: Method = Method::POST;
    type ReturnType = LolLobbyLobbyCustomChampSelectStartResponse;

    fn get_url(&self) -> String {
        "/lol-lobby/v1/lobby/custom/start-champ-select".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn post_lol_lobby_v_1_lobby_custom_start_champ_select() -> PostLolLobbyV1LobbyCustomStartChampSelect {
    PostLolLobbyV1LobbyCustomStartChampSelect {
        
    }
}


pub struct PostLolLobbyV1LobbyCustomSwitchTeams {

    pub body: String,
}

impl IsApiRequest for PostLolLobbyV1LobbyCustomSwitchTeams {
    const METHOD: Method = Method::POST;
    type ReturnType = HashMap<String, String>;

    fn get_url(&self) -> String {
        "/lol-lobby/v1/lobby/custom/switch-teams".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn post_lol_lobby_v_1_lobby_custom_switch_teams(body: String) -> PostLolLobbyV1LobbyCustomSwitchTeams {
    PostLolLobbyV1LobbyCustomSwitchTeams {
        body
    }
}


pub struct PostLolLobbyV1LobbyMembersLocalMemberPlayerSlotsBySlotsIndexByPerksString {

    pub slots_index: u64,
    pub perks_string: String,
}

impl IsApiRequest for PostLolLobbyV1LobbyMembersLocalMemberPlayerSlotsBySlotsIndexByPerksString {
    const METHOD: Method = Method::POST;
    type ReturnType = HashMap<String, String>;

    fn get_url(&self) -> String {
        format!("/lol-lobby/v1/lobby/members/localMember/player-slots/{}/{}", self.slots_index, self.perks_string)
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn post_lol_lobby_v_1_lobby_members_local_member_player_slots_by_slots_index_by_perks_string(slots_index: u64, perks_string: String) -> PostLolLobbyV1LobbyMembersLocalMemberPlayerSlotsBySlotsIndexByPerksString {
    PostLolLobbyV1LobbyMembersLocalMemberPlayerSlotsBySlotsIndexByPerksString {
        slots_index, perks_string
    }
}


pub struct PostLolLobbyV1TournamentsByIdJoin {

    pub id: String,
}

impl IsApiRequest for PostLolLobbyV1TournamentsByIdJoin {
    const METHOD: Method = Method::POST;
    type ReturnType = HashMap<String, String>;

    fn get_url(&self) -> String {
        format!("/lol-lobby/v1/tournaments/{}/join", self.id)
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn post_lol_lobby_v_1_tournaments_by_id_join(id: String) -> PostLolLobbyV1TournamentsByIdJoin {
    PostLolLobbyV1TournamentsByIdJoin {
        id
    }
}


pub struct PostLolLobbyV2EligibilityParty {

}

impl IsApiRequest for PostLolLobbyV2EligibilityParty {
    const METHOD: Method = Method::POST;
    type ReturnType = Vec<LolLobbyEligibility>;

    fn get_url(&self) -> String {
        "/lol-lobby/v2/eligibility/party".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn post_lol_lobby_v_2_eligibility_party() -> PostLolLobbyV2EligibilityParty {
    PostLolLobbyV2EligibilityParty {
        
    }
}


pub struct PostLolLobbyV2EligibilitySelf {

}

impl IsApiRequest for PostLolLobbyV2EligibilitySelf {
    const METHOD: Method = Method::POST;
    type ReturnType = Vec<LolLobbyEligibility>;

    fn get_url(&self) -> String {
        "/lol-lobby/v2/eligibility/self".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn post_lol_lobby_v_2_eligibility_self() -> PostLolLobbyV2EligibilitySelf {
    PostLolLobbyV2EligibilitySelf {
        
    }
}


pub struct PostLolLobbyV2EogInvitations {

    pub body: Vec<LolLobbyLobbyInvitationDto>,
}

impl IsApiRequest for PostLolLobbyV2EogInvitations {
    const METHOD: Method = Method::POST;
    type ReturnType = Vec<LolLobbyLobbyInvitationDto>;

    fn get_url(&self) -> String {
        "/lol-lobby/v2/eog-invitations".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn post_lol_lobby_v_2_eog_invitations(body: Vec<LolLobbyLobbyInvitationDto>) -> PostLolLobbyV2EogInvitations {
    PostLolLobbyV2EogInvitations {
        body
    }
}


pub struct PostLolLobbyV2LobbyMembersBySummonerIdGrantInvite {

    pub summoner_id: u64,
}

impl IsApiRequest for PostLolLobbyV2LobbyMembersBySummonerIdGrantInvite {
    const METHOD: Method = Method::POST;
    type ReturnType = u64;

    fn get_url(&self) -> String {
        format!("/lol-lobby/v2/lobby/members/{}/grant-invite", self.summoner_id)
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn post_lol_lobby_v_2_lobby_members_by_summoner_id_grant_invite(summoner_id: u64) -> PostLolLobbyV2LobbyMembersBySummonerIdGrantInvite {
    PostLolLobbyV2LobbyMembersBySummonerIdGrantInvite {
        summoner_id
    }
}


pub struct PostLolLobbyV2LobbyMembersBySummonerIdKick {

    pub summoner_id: u64,
}

impl IsApiRequest for PostLolLobbyV2LobbyMembersBySummonerIdKick {
    const METHOD: Method = Method::POST;
    type ReturnType = u64;

    fn get_url(&self) -> String {
        format!("/lol-lobby/v2/lobby/members/{}/kick", self.summoner_id)
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn post_lol_lobby_v_2_lobby_members_by_summoner_id_kick(summoner_id: u64) -> PostLolLobbyV2LobbyMembersBySummonerIdKick {
    PostLolLobbyV2LobbyMembersBySummonerIdKick {
        summoner_id
    }
}


pub struct PostLolLobbyV2LobbyMembersBySummonerIdPromote {

    pub summoner_id: u64,
}

impl IsApiRequest for PostLolLobbyV2LobbyMembersBySummonerIdPromote {
    const METHOD: Method = Method::POST;
    type ReturnType = u64;

    fn get_url(&self) -> String {
        format!("/lol-lobby/v2/lobby/members/{}/promote", self.summoner_id)
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn post_lol_lobby_v_2_lobby_members_by_summoner_id_promote(summoner_id: u64) -> PostLolLobbyV2LobbyMembersBySummonerIdPromote {
    PostLolLobbyV2LobbyMembersBySummonerIdPromote {
        summoner_id
    }
}


pub struct PostLolLobbyV2LobbyMembersBySummonerIdRevokeInvite {

    pub summoner_id: u64,
}

impl IsApiRequest for PostLolLobbyV2LobbyMembersBySummonerIdRevokeInvite {
    const METHOD: Method = Method::POST;
    type ReturnType = u64;

    fn get_url(&self) -> String {
        format!("/lol-lobby/v2/lobby/members/{}/revoke-invite", self.summoner_id)
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn post_lol_lobby_v_2_lobby_members_by_summoner_id_revoke_invite(summoner_id: u64) -> PostLolLobbyV2LobbyMembersBySummonerIdRevokeInvite {
    PostLolLobbyV2LobbyMembersBySummonerIdRevokeInvite {
        summoner_id
    }
}


pub struct PostLolLobbyV2LobbyTeamByTeam {

    pub team: String,
}

impl IsApiRequest for PostLolLobbyV2LobbyTeamByTeam {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;

    fn get_url(&self) -> String {
        format!("/lol-lobby/v2/lobby/team/{}", self.team)
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn post_lol_lobby_v_2_lobby_team_by_team(team: String) -> PostLolLobbyV2LobbyTeamByTeam {
    PostLolLobbyV2LobbyTeamByTeam {
        team
    }
}


pub struct PostLolLobbyV2MatchmakingQuickSearch {

    pub body: LolLobbyLobbyChangeGameDto,
}

impl IsApiRequest for PostLolLobbyV2MatchmakingQuickSearch {
    const METHOD: Method = Method::POST;
    type ReturnType = HashMap<String, String>;

    fn get_url(&self) -> String {
        "/lol-lobby/v2/matchmaking/quick-search".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn post_lol_lobby_v_2_matchmaking_quick_search(body: LolLobbyLobbyChangeGameDto) -> PostLolLobbyV2MatchmakingQuickSearch {
    PostLolLobbyV2MatchmakingQuickSearch {
        body
    }
}


pub struct PostLolLobbyV2PartiesOverridesEnabledForTeamBuilderQueues {

    pub body: bool,
}

impl IsApiRequest for PostLolLobbyV2PartiesOverridesEnabledForTeamBuilderQueues {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;

    fn get_url(&self) -> String {
        "/lol-lobby/v2/parties/overrides/EnabledForTeamBuilderQueues".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn post_lol_lobby_v_2_parties_overrides_enabled_for_team_builder_queues(body: bool) -> PostLolLobbyV2PartiesOverridesEnabledForTeamBuilderQueues {
    PostLolLobbyV2PartiesOverridesEnabledForTeamBuilderQueues {
        body
    }
}


pub struct PostLolLobbyV2PartyByPartyIdJoin {

    pub party_id: String,
    pub body: LolLobbyCustomJoinOptionsDto,
}

impl IsApiRequest for PostLolLobbyV2PartyByPartyIdJoin {
    const METHOD: Method = Method::POST;
    type ReturnType = HashMap<String, String>;

    fn get_url(&self) -> String {
        format!("/lol-lobby/v2/party/{}/join", self.party_id)
    }

    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn post_lol_lobby_v_2_party_by_party_id_join(party_id: String, body: LolLobbyCustomJoinOptionsDto) -> PostLolLobbyV2PartyByPartyIdJoin {
    PostLolLobbyV2PartyByPartyIdJoin {
        party_id, body
    }
}


pub struct PostLolLobbyV2PlayAgain {

}

impl IsApiRequest for PostLolLobbyV2PlayAgain {
    const METHOD: Method = Method::POST;
    type ReturnType = HashMap<String, String>;

    fn get_url(&self) -> String {
        "/lol-lobby/v2/play-again".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn post_lol_lobby_v_2_play_again() -> PostLolLobbyV2PlayAgain {
    PostLolLobbyV2PlayAgain {
        
    }
}


pub struct PostLolLobbyV2PlayAgainDecline {

}

impl IsApiRequest for PostLolLobbyV2PlayAgainDecline {
    const METHOD: Method = Method::POST;
    type ReturnType = HashMap<String, String>;

    fn get_url(&self) -> String {
        "/lol-lobby/v2/play-again-decline".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn post_lol_lobby_v_2_play_again_decline() -> PostLolLobbyV2PlayAgainDecline {
    PostLolLobbyV2PlayAgainDecline {
        
    }
}


pub struct PostLolLobbyV2ReceivedInvitationsByInvitationIdAccept {

    pub invitation_id: String,
}

impl IsApiRequest for PostLolLobbyV2ReceivedInvitationsByInvitationIdAccept {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;

    fn get_url(&self) -> String {
        format!("/lol-lobby/v2/received-invitations/{}/accept", self.invitation_id)
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn post_lol_lobby_v_2_received_invitations_by_invitation_id_accept(invitation_id: String) -> PostLolLobbyV2ReceivedInvitationsByInvitationIdAccept {
    PostLolLobbyV2ReceivedInvitationsByInvitationIdAccept {
        invitation_id
    }
}


pub struct PostLolLobbyV2ReceivedInvitationsByInvitationIdDecline {

    pub invitation_id: String,
}

impl IsApiRequest for PostLolLobbyV2ReceivedInvitationsByInvitationIdDecline {
    const METHOD: Method = Method::POST;
    type ReturnType = Value;

    fn get_url(&self) -> String {
        format!("/lol-lobby/v2/received-invitations/{}/decline", self.invitation_id)
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn post_lol_lobby_v_2_received_invitations_by_invitation_id_decline(invitation_id: String) -> PostLolLobbyV2ReceivedInvitationsByInvitationIdDecline {
    PostLolLobbyV2ReceivedInvitationsByInvitationIdDecline {
        invitation_id
    }
}


pub struct PutLolLobbyV1LobbyMembersLocalMemberPositionPreferences {

    pub body: LolLobbyLobbyPositionPreferences,
}

impl IsApiRequest for PutLolLobbyV1LobbyMembersLocalMemberPositionPreferences {
    const METHOD: Method = Method::PUT;
    type ReturnType = HashMap<String, String>;

    fn get_url(&self) -> String {
        "/lol-lobby/v1/lobby/members/localMember/position-preferences".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn put_lol_lobby_v_1_lobby_members_local_member_position_preferences(body: LolLobbyLobbyPositionPreferences) -> PutLolLobbyV1LobbyMembersLocalMemberPositionPreferences {
    PutLolLobbyV1LobbyMembersLocalMemberPositionPreferences {
        body
    }
}


pub struct PutLolLobbyV1LobbyMembersLocalMemberQuickplayPlayerState {

    pub body: String,
}

impl IsApiRequest for PutLolLobbyV1LobbyMembersLocalMemberQuickplayPlayerState {
    const METHOD: Method = Method::PUT;
    type ReturnType = HashMap<String, String>;

    fn get_url(&self) -> String {
        "/lol-lobby/v1/lobby/members/localMember/quickplayPlayerState".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn put_lol_lobby_v_1_lobby_members_local_member_quickplay_player_state(body: String) -> PutLolLobbyV1LobbyMembersLocalMemberQuickplayPlayerState {
    PutLolLobbyV1LobbyMembersLocalMemberQuickplayPlayerState {
        body
    }
}


pub struct PutLolLobbyV1PartiesActive {

    pub body: i32,
}

impl IsApiRequest for PutLolLobbyV1PartiesActive {
    const METHOD: Method = Method::PUT;
    type ReturnType = Value;

    fn get_url(&self) -> String {
        "/lol-lobby/v1/parties/active".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn put_lol_lobby_v_1_parties_active(body: i32) -> PutLolLobbyV1PartiesActive {
    PutLolLobbyV1PartiesActive {
        body
    }
}


pub struct PutLolLobbyV1PartiesByPartyIdMembersByPuuidRole {

    pub party_id: String,
    pub puuid: String,
    pub body: String,
}

impl IsApiRequest for PutLolLobbyV1PartiesByPartyIdMembersByPuuidRole {
    const METHOD: Method = Method::PUT;
    type ReturnType = Value;

    fn get_url(&self) -> String {
        format!("/lol-lobby/v1/parties/{}/members/{}/role", self.party_id, self.puuid)
    }

    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn put_lol_lobby_v_1_parties_by_party_id_members_by_puuid_role(party_id: String, puuid: String, body: String) -> PutLolLobbyV1PartiesByPartyIdMembersByPuuidRole {
    PutLolLobbyV1PartiesByPartyIdMembersByPuuidRole {
        party_id, puuid, body
    }
}


pub struct PutLolLobbyV1PartiesMetadata {

    pub body: LolLobbyPartyMemberMetadataDto,
}

impl IsApiRequest for PutLolLobbyV1PartiesMetadata {
    const METHOD: Method = Method::PUT;
    type ReturnType = Value;

    fn get_url(&self) -> String {
        "/lol-lobby/v1/parties/metadata".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn put_lol_lobby_v_1_parties_metadata(body: LolLobbyPartyMemberMetadataDto) -> PutLolLobbyV1PartiesMetadata {
    PutLolLobbyV1PartiesMetadata {
        body
    }
}


pub struct PutLolLobbyV1PartiesQueue {

    pub body: i32,
}

impl IsApiRequest for PutLolLobbyV1PartiesQueue {
    const METHOD: Method = Method::PUT;
    type ReturnType = Value;

    fn get_url(&self) -> String {
        "/lol-lobby/v1/parties/queue".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn put_lol_lobby_v_1_parties_queue(body: i32) -> PutLolLobbyV1PartiesQueue {
    PutLolLobbyV1PartiesQueue {
        body
    }
}


pub struct PutLolLobbyV1PartiesReady {

    pub body: i32,
}

impl IsApiRequest for PutLolLobbyV1PartiesReady {
    const METHOD: Method = Method::PUT;
    type ReturnType = Value;

    fn get_url(&self) -> String {
        "/lol-lobby/v1/parties/ready".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn put_lol_lobby_v_1_parties_ready(body: i32) -> PutLolLobbyV1PartiesReady {
    PutLolLobbyV1PartiesReady {
        body
    }
}


pub struct PutLolLobbyV2LobbyMembersLocalMemberPositionPreferences {

    pub body: LolLobbyLobbyPositionPreferences,
}

impl IsApiRequest for PutLolLobbyV2LobbyMembersLocalMemberPositionPreferences {
    const METHOD: Method = Method::PUT;
    type ReturnType = HashMap<String, String>;

    fn get_url(&self) -> String {
        "/lol-lobby/v2/lobby/members/localMember/position-preferences".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn put_lol_lobby_v_2_lobby_members_local_member_position_preferences(body: LolLobbyLobbyPositionPreferences) -> PutLolLobbyV2LobbyMembersLocalMemberPositionPreferences {
    PutLolLobbyV2LobbyMembersLocalMemberPositionPreferences {
        body
    }
}


pub struct PutLolLobbyV2LobbyPartyType {

    pub body: String,
}

impl IsApiRequest for PutLolLobbyV2LobbyPartyType {
    const METHOD: Method = Method::PUT;
    type ReturnType = HashMap<String, String>;

    fn get_url(&self) -> String {
        "/lol-lobby/v2/lobby/partyType".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn put_lol_lobby_v_2_lobby_party_type(body: String) -> PutLolLobbyV2LobbyPartyType {
    PutLolLobbyV2LobbyPartyType {
        body
    }
}


pub struct PutLolLobbyV2LobbyStrawberryMapId {

    pub body: LolLobbyStrawberryMapUpdateDto,
}

impl IsApiRequest for PutLolLobbyV2LobbyStrawberryMapId {
    const METHOD: Method = Method::PUT;
    type ReturnType = Value;

    fn get_url(&self) -> String {
        "/lol-lobby/v2/lobby/strawberryMapId".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn put_lol_lobby_v_2_lobby_strawberry_map_id(body: LolLobbyStrawberryMapUpdateDto) -> PutLolLobbyV2LobbyStrawberryMapId {
    PutLolLobbyV2LobbyStrawberryMapId {
        body
    }
}


pub struct PutLolLobbyV2LobbySubteamData {

    pub body: LolLobbySubteamDataDto,
}

impl IsApiRequest for PutLolLobbyV2LobbySubteamData {
    const METHOD: Method = Method::PUT;
    type ReturnType = Value;

    fn get_url(&self) -> String {
        "/lol-lobby/v2/lobby/subteamData".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn put_lol_lobby_v_2_lobby_subteam_data(body: LolLobbySubteamDataDto) -> PutLolLobbyV2LobbySubteamData {
    PutLolLobbyV2LobbySubteamData {
        body
    }
}


// OBJECTS

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyQueueGameTypeConfig {
    pub id: i64,
    pub name: String,
    pub max_allowable_bans: i32,
    pub allow_trades: bool,
    pub exclusive_pick: bool,
    pub duplicate_pick: bool,
    pub team_champion_pool: bool,
    pub cross_team_champion_pool: bool,
    pub advanced_learning_quests: bool,
    pub battle_boost: bool,
    pub death_match: bool,
    pub do_not_remove: bool,
    pub learning_quests: bool,
    pub onboard_coop_beginner: bool,
    pub reroll: bool,
    pub main_pick_timer_duration: i32,
    pub post_pick_timer_duration: i32,
    pub ban_timer_duration: i32,
    pub pick_mode: String,
    pub ban_mode: String,
    pub game_mode_override: String,
    pub num_players_per_team_override: i32,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyLobbyNotification {
    pub notification_id: String,
    pub notification_reason: String,
    pub summoner_ids: Vec<u64>,
    pub timestamp: u64,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyQuickPlayPresetSlotDto {
    pub champion_id: i32,
    pub skin_id: i32,
    pub position_preference: String,
    pub perks: String,
    pub spell_1: u64,
    pub spell_2: u64,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyEligibility {
    pub queue_id: i32,
    pub eligible: bool,
    pub restrictions: Vec<LolLobbyEligibilityRestriction>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyPlayerDto {
    pub puuid: String,
    pub platform_id: String,
    pub account_id: u64,
    pub summoner_id: u64,
    pub eligibility_hash: i64,
    pub server_utc_millis: i64,
    pub parties: Vec<LolLobbyPartyMemberDto>,
    pub current_party: LolLobbyPartyDto,
    pub tft_games_played: i64,
    pub tft_games_won: i64,
    pub registration: LolLobbyRegistrationCredentials,
    pub created_at: u64,
    pub version: u64,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyLobbyDto {
    pub party_id: String,
    pub party_type: String,
    pub members: Vec<LolLobbyLobbyParticipantDto>,
    pub local_member: LolLobbyLobbyParticipantDto,
    pub invitations: Vec<LolLobbyLobbyInvitationDto>,
    pub can_start_activity: bool,
    pub restrictions: Vec<LolLobbyEligibilityRestriction>,
    pub warnings: Vec<LolLobbyEligibilityRestriction>,
    pub game_config: LolLobbyLobbyGameConfigDto,
    pub multi_user_chat_id: String,
    pub multi_user_chat_password: String,
    pub muc_jwt_dto: LolLobbyMucJwtDto,
    pub scarce_positions: Vec<String>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyLobbyCustomGameConfiguration {
    pub map_id: i32,
    pub game_mode: String,
    pub mutators: LolLobbyQueueGameTypeConfig,
    pub game_type_config: LolLobbyQueueGameTypeConfig,
    pub spectator_policy: LolLobbyQueueCustomGameSpectatorPolicy,
    pub team_size: i32,
    pub max_player_count: u32,
    pub tournament_game_mode: String,
    pub tournament_passback_url: String,
    pub tournament_passback_data_packet: String,
    pub game_server_region: String,
    pub spectator_delay_enabled: bool,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyPartyChatDto {
    pub jid: String,
    pub muc_jwt_dto: LolLobbyMucJwtDto,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyLobbyPartyRewards {
    pub is_enabled: bool,
    pub queue_id: i32,
    pub is_custom: bool,
    pub party_rewards: Vec<LolLobbyPartyReward>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyLobbyGameConfigDto {
    pub game_mode: String,
    pub map_id: i32,
    pub queue_id: i32,
    pub pick_type: String,
    pub max_team_size: i32,
    pub max_lobby_size: i32,
    pub max_human_players: i32,
    pub allowable_premade_sizes: Vec<i32>,
    pub premade_size_allowed: bool,
    pub is_team_builder_managed: bool,
    pub is_custom: bool,
    pub show_position_selector: bool,
    pub show_quick_play_slot_selection: bool,
    pub is_lobby_full: bool,
    pub should_force_scarce_position_selection: bool,
    pub custom_lobby_name: String,
    pub custom_mutator_name: String,
    pub custom_team_100: Vec<LolLobbyLobbyParticipantDto>,
    pub custom_team_200: Vec<LolLobbyLobbyParticipantDto>,
    pub custom_spectators: Vec<LolLobbyLobbyParticipantDto>,
    pub custom_spectator_policy: LolLobbyQueueCustomGameSpectatorPolicy,
    pub custom_rewards_disabled_reasons: Vec<String>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyLobbyCustomGameLobby {
    pub lobby_name: String,
    pub lobby_password: String,
    pub configuration: LolLobbyLobbyCustomGameConfiguration,
    pub team_one: Vec<LolLobbyLobbyMember>,
    pub team_two: Vec<LolLobbyLobbyMember>,
    pub spectators: Vec<LolLobbyLobbyMember>,
    pub practice_game_rewards_disabled_reasons: Vec<String>,
    pub game_id: u64,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyPartyReward {
    pub premade_size: i32,
    pub type_: LolLobbyLobbyPartyRewardType,
    pub value: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyLobbyBotChampion {
    pub active: bool,
    pub id: i32,
    pub name: String,
    pub bot_difficulties: Vec<LolLobbyLobbyBotDifficulty>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyLobbyMatchmakingSearchResource {
    pub search_state: LolLobbyLobbyMatchmakingSearchState,
    pub low_priority_data: LolLobbyLobbyMatchmakingLowPriorityDataResource,
    pub errors: Vec<LolLobbyLobbyMatchmakingSearchErrorResource>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyLobbyPositionPreferences {
    pub first_preference: String,
    pub second_preference: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbySubteamDataDto {
    pub subteam_index: i8,
    pub intra_subteam_position: i8,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyPartyMemberDto {
    pub platform_id: String,
    pub puuid: String,
    pub account_id: u64,
    pub summoner_id: u64,
    pub party_id: String,
    pub party_version: i64,
    pub role: LolLobbyPartyMemberRoleEnum,
    pub game_mode: LolLobbyGameModeDto,
    pub ready: bool,
    pub metadata: LolLobbyPartyMemberMetadataDto,
    pub invited_by_summoner_id: u64,
    pub invite_timestamp: u64,
    pub can_invite: bool,
    pub team: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyPartyDto {
    pub party_id: String,
    pub platform_id: String,
    pub players: Vec<LolLobbyPartyMemberDto>,
    pub chat: LolLobbyPartyChatDto,
    pub max_party_size: i32,
    pub party_type: String,
    pub game_mode: LolLobbyGameModeDto,
    pub activity_locked: bool,
    pub version: u64,
    pub activity_started_utc_millis: u64,
    pub activity_resume_utc_millis: u64,
    pub active_restrictions: LolLobbyQueueRestrictionDto,
    pub eligibility_hash: i64,
    pub eligibility_restrictions: Vec<LolLobbyGatekeeperRestrictionDto>,
    pub eligibility_warnings: Vec<LolLobbyGatekeeperRestrictionDto>,
    pub bot_participants: Vec<LolLobbyBotParticipantDto>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyRegistrationCredentials {
    pub summoner_token: String,
    pub inventory_token: String,
    pub inventory_tokens: Vec<String>,
    pub simple_inventory_token: String,
    pub ranked_overview_token: String,
    pub game_client_version: String,
    pub player_tokens: HashMap<String, String>,
    pub experiments: HashMap<String, String>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyPremadeMemberDto {
    pub display_name: String,
    pub game_name: String,
    pub tag_line: String,
    pub puuid: String,
    pub party_id: String,
    pub summoner_id: u64,
    pub role: LolLobbyPartyMemberRoleEnum,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyLobbyCustomJoinParameters {
    pub password: String,
    pub as_spectator: bool,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyLobbyCustomFailedPlayer {
    pub summoner_id: u64,
    pub summoner_name: String,
    pub reason: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyPartyStatusDto {
    pub ready_players: Vec<String>,
    pub left_players: Vec<String>,
    pub eog_players: Vec<String>,
    pub party_size: i32,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyLobbyMember {
    pub id: u64,
    pub is_owner: bool,
    pub is_spectator: bool,
    pub can_invite_others: bool,
    pub position_preferences: LolLobbyLobbyPositionPreferences,
    pub excluded_position_preference: String,
    pub summoner_internal_name: String,
    pub show_position_excluder: bool,
    pub auto_fill_eligible: bool,
    pub auto_fill_protected_for_streaking: bool,
    pub auto_fill_protected_for_promos: bool,
    pub auto_fill_protected_for_soloing: bool,
    pub is_bot: bool,
    pub bot_difficulty: LolLobbyLobbyBotDifficulty,
    pub bot_champion_id: i32,
    pub position: String,
    pub bot_uuid: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyLobbyInvitationDto {
    pub invitation_id: String,
    pub to_summoner_id: u64,
    pub state: LolLobbyLobbyInvitationState,
    pub timestamp: String,
    pub to_summoner_name: String,
    pub invitation_type: LolLobbyInvitationType,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyMucJwtDto {
    pub jwt: String,
    pub channel_claim: String,
    pub domain: String,
    pub target_region: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyLobbyBotParams {
    pub champion_id: i32,
    pub bot_difficulty: LolLobbyLobbyBotDifficulty,
    pub team_id: String,
    pub position: String,
    pub bot_uuid: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyLobbyInvitation {
    pub id: String,
    pub from_summoner_id: u64,
    pub to_summoner_id: u64,
    pub state: LolLobbyLobbyInvitationState,
    pub error_type: String,
    pub timestamp: String,
    pub invitation_meta_data: HashMap<String, String>,
    pub eligibility: LolLobbyEligibility,
    pub from_summoner_name: String,
    pub to_summoner_name: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyGatekeeperRestrictionDto {
    pub account_id: u64,
    pub reason: String,
    pub remaining_millis: i64,
    pub payload: String,
    pub queue_id: i32,
    pub puuid: String,
    pub details: HashMap<String, String>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyLobbyCustomGame {
    pub id: u64,
    pub game_type: String,
    pub map_id: i32,
    pub owner_display_name: String,
    pub spectator_policy: String,
    pub filled_spectator_slots: i32,
    pub max_spectator_slots: u64,
    pub filled_player_slots: i32,
    pub max_player_slots: i32,
    pub lobby_name: String,
    pub has_password: bool,
    pub passback_url: String,
    pub party_id: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyReceivedInvitationGameConfigDto {
    pub game_mode: String,
    pub queue_id: i32,
    pub map_id: i32,
    pub invite_game_type: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyLobbyMatchmakingLowPriorityDataResource {
    pub penalized_summoner_ids: Vec<u64>,
    pub penalty_time: f64,
    pub penalty_time_remaining: f64,
    pub busted_leaver_access_token: String,
    pub reason: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyEligibilityRestriction {
    pub restriction_code: LolLobbyEligibilityRestrictionCode,
    pub restriction_args: HashMap<String, String>,
    pub expired_timestamp: u64,
    pub summoner_ids: Vec<u64>,
    pub summoner_ids_string: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyGameModeDto {
    pub game_type: String,
    pub max_team_size: i32,
    pub max_party_size: i32,
    pub bot_difficulty: String,
    pub queue_id: i32,
    pub game_customization: HashMap<String, String>,
    pub customs_settings: LolLobbyCustomGameSettingsDto,
    pub game_type_config_id: i64,
    pub map_id: i32,
    pub allow_spectators: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyQueueRestrictionDto {
    pub gatekeeper_restrictions: Vec<LolLobbyGatekeeperRestrictionDto>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyPartyMemberMetadataDto {
    pub position_pref: Vec<String>,
    pub properties: HashMap<String, String>,
    pub player_slots: Vec<LolLobbyQuickPlayPresetSlotDto>,
    pub subteam_data: LolLobbySubteamDataDto,
    pub tft_npe_queue_bypass: bool,
    pub quickplay_player_state: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyBotParticipantDto {
    pub champion_id: i32,
    pub bot_skill_level: i32,
    pub team: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyLobbyMatchmakingSearchErrorResource {
    pub id: i32,
    pub error_type: String,
    pub penalized_summoner_id: u64,
    pub penalty_time_remaining: f64,
    pub message: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyCustomJoinOptionsDto {
    pub lobby_password: String,
    pub team: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyLobbyChangeGameDto {
    pub queue_id: i32,
    pub is_custom: bool,
    pub custom_game_lobby: LolLobbyLobbyCustomGameLobby,
    pub game_customization: HashMap<String, String>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyPremadePartyDto {
    pub party_id: String,
    pub comms_enabled: bool,
    pub players: LolLobbyPremadeMemberDto,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyStrawberryMapUpdateDto {
    pub content_id: String,
    pub item_id: i32,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyLobbyParticipantDto {
    pub summoner_id: u64,
    pub summoner_icon_id: i32,
    pub summoner_name: String,
    pub summoner_internal_name: String,
    pub puuid: String,
    pub summoner_level: u32,
    pub allowed_start_activity: bool,
    pub allowed_change_activity: bool,
    pub allowed_toggle_invite: bool,
    pub allowed_kick_others: bool,
    pub allowed_invite_others: bool,
    pub is_leader: bool,
    pub is_spectator: bool,
    pub team_id: i32,
    pub first_position_preference: String,
    pub second_position_preference: String,
    pub subteam_index: i8,
    pub intra_subteam_position: i8,
    pub tft_npe_queue_bypass: bool,
    pub quickplay_player_state: String,
    pub strawberry_map_id: String,
    pub player_slots: Vec<LolLobbyQuickPlayPresetSlotDto>,
    pub ready: bool,
    pub show_ghosted_banner: bool,
    pub auto_fill_eligible: bool,
    pub auto_fill_protected_for_streaking: bool,
    pub auto_fill_protected_for_promos: bool,
    pub auto_fill_protected_for_soloing: bool,
    pub auto_fill_protected_for_remedy: bool,
    pub is_bot: bool,
    pub bot_id: String,
    pub bot_difficulty: LolLobbyLobbyBotDifficulty,
    pub bot_champion_id: i32,
    pub bot_position: String,
    pub bot_uuid: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyCustomGameSettingsDto {
    pub lobby_name: String,
    pub lobby_password: String,
    pub game_id: u64,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyReceivedInvitationDto {
    pub invitation_id: String,
    pub from_summoner_id: u64,
    pub state: LolLobbyLobbyInvitationState,
    pub timestamp: String,
    pub from_summoner_name: String,
    pub can_accept_invitation: bool,
    pub restrictions: Vec<LolLobbyEligibilityRestriction>,
    pub game_config: LolLobbyReceivedInvitationGameConfigDto,
    pub invitation_type: LolLobbyInvitationType,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyLobbyCustomChampSelectStartResponse {
    pub success: bool,
    pub failed_players: Vec<LolLobbyLobbyCustomFailedPlayer>,
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
pub enum LolLobbyQueueCustomGameSpectatorPolicy {
    #[default]
    AllAllowed,
    FriendsAllowed,
    LobbyAllowed,
    NotAllowed,
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
pub enum LolLobbyQueueAvailability {
    #[default]
    DoesntMeetRequirements,
    PlatformDisabled,
    Available,
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
pub enum LolLobbyLobbyPartyRewardType {
    #[default]
    None,
    Icon,
    Ip,
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
pub enum LolLobbyInvitationType {
    #[default]
    #[serde(rename = "party")]
    Party,
    #[serde(rename = "lobby")]
    Lobby,
    #[serde(rename = "invalid")]
    Invalid,
}

