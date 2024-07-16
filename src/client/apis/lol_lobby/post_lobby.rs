use std::fmt;
use serde::{Deserialize, Serialize};
use crate::client::apis::lol_game_queues::get_queues::LolGameMode;

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyPostLobbyBody {
    pub queue_id: i32,
    pub is_custom: bool,
    pub custom_game_lobby: Option<LolLobbyLobbyCustomGameLobby>,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyLobbyCustomGameLobby {
    pub lobby_name: String,
    pub lobby_password: Option<String>,
    pub configuration: LolLobbyLobbyCustomGameConfiguration,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyLobbyCustomGameConfiguration {
    pub map_id: i32,
    pub game_mode: LolGameMode,
    pub mutators: LolLobbyQueueGameTypeConfig,
    pub spectator_policy: LolLobbyQueueCustomGameSpectatorPolicy,
    pub team_size: i32,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct LolLobbyQueueGameTypeConfig {
    pub id: i64,
}


#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq)]
pub enum LolLobbyQueueCustomGameSpectatorPolicy {
    AllAllowed,
    FriendsAllowed,
    LobbyAllowed,
    #[default]
    NotAllowed,
}

impl LolLobbyQueueCustomGameSpectatorPolicy{
    pub fn cases() -> Vec<LolLobbyQueueCustomGameSpectatorPolicy>{
        vec![
            LolLobbyQueueCustomGameSpectatorPolicy::NotAllowed,
            LolLobbyQueueCustomGameSpectatorPolicy::AllAllowed,
            LolLobbyQueueCustomGameSpectatorPolicy::FriendsAllowed,
            LolLobbyQueueCustomGameSpectatorPolicy::LobbyAllowed
        ]
    }
}

impl fmt::Display for LolLobbyQueueCustomGameSpectatorPolicy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LolLobbyQueueCustomGameSpectatorPolicy::AllAllowed => write!(f, "AllAllowed"),
            LolLobbyQueueCustomGameSpectatorPolicy::FriendsAllowed => write!(f, "FriendsAllowed"),
            LolLobbyQueueCustomGameSpectatorPolicy::LobbyAllowed => write!(f, "LobbyAllowed"),
            LolLobbyQueueCustomGameSpectatorPolicy::NotAllowed => write!(f, "NotAllowed"),
        }
    }
}




#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LolLobbySession {
    #[serde(rename = "partyId")]
    pub party_id: String,
    #[serde(rename = "partyType")]
    pub party_type: String,
    pub members: Vec<Member>,
    #[serde(rename = "localMember")]
    pub local_member: Member,
    pub invitations: Vec<Invitation>,
    #[serde(rename = "canStartActivity")]
    pub can_start_activity: bool,
    pub restrictions: Option<Vec<Restriction>>,
    pub warnings: Option<Vec<Restriction>>,
    #[serde(rename = "gameConfig")]
    pub game_config: GameConfig,
    #[serde(rename = "multiUserChatId")]
    pub multi_user_chat_id: String,
    #[serde(rename = "multiUserChatPassword")]
    pub multi_user_chat_password: String,
    #[serde(rename = "mucJwtDto")]
    pub muc_jwt_dto: MucJwtDto,
    #[serde(rename = "scarcePositions")]
    pub scarce_positions: Vec<String>,
}


#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlayerSlot {
    #[serde(rename = "championId")]
    pub champion_id: i64,
    #[serde(rename = "skinId")]
    pub skin_id: i64,
    #[serde(rename = "positionPreference")]
    pub position_preference: String,
    pub perks: String,
    pub spell1: i64,
    pub spell2: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Member {
    #[serde(rename = "summonerId")]
    pub summoner_id: i64,
    #[serde(rename = "summonerIconId")]
    pub summoner_icon_id: i64,
    #[serde(rename = "summonerName")]
    pub summoner_name: String,
    #[serde(rename = "summonerInternalName")]
    pub summoner_internal_name: String,
    pub puuid: String,
    #[serde(rename = "summonerLevel")]
    pub summoner_level: i64,
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
    pub team_id: i64,
    #[serde(rename = "firstPositionPreference")]
    pub first_position_preference: String,
    #[serde(rename = "secondPositionPreference")]
    pub second_position_preference: String,
    #[serde(rename = "subteamIndex")]
    pub subteam_index: Option<i64>,
    #[serde(rename = "intraSubteamPosition")]
    pub intra_subteam_position: Option<i64>,
    #[serde(rename = "tftNPEQueueBypass")]
    pub tft_npe_queue_bypass: Option<bool>,
    #[serde(rename = "quickplayPlayerState")]
    pub quickplay_player_state: Option<String>,
    #[serde(rename = "strawberryMapId")]
    pub strawberry_map_id: Option<String>,
    #[serde(rename = "playerSlots")]
    pub player_slots: Vec<PlayerSlot>,
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
    pub bot_difficulty: String,
    #[serde(rename = "botChampionId")]
    pub bot_champion_id: i64,
    #[serde(rename = "botPosition")]
    pub bot_position: String,
    #[serde(rename = "botUuid")]
    pub bot_uuid: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Invitation {
    #[serde(rename = "invitationId")]
    pub invitation_id: String,
    #[serde(rename = "toSummonerId")]
    pub to_summoner_id: i64,
    pub state: String,
    pub timestamp: String,
    #[serde(rename = "toSummonerName")]
    pub to_summoner_name: String,
    #[serde(rename = "invitationType")]
    pub invitation_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RestrictionArgs {
    #[serde(rename = "additionalProp1")]
    pub additional_prop1: String,
    #[serde(rename = "additionalProp2")]
    pub additional_prop2: String,
    #[serde(rename = "additionalProp3")]
    pub additional_prop3: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Restriction {
    #[serde(rename = "restrictionCode")]
    pub restriction_code: String,
    #[serde(rename = "restrictionArgs")]
    pub restriction_args: RestrictionArgs,
    #[serde(rename = "expiredTimestamp")]
    pub expired_timestamp: i64,
    #[serde(rename = "summonerIds")]
    pub summoner_ids: Vec<i64>,
    #[serde(rename = "summonerIdsString")]
    pub summoner_ids_string: String,
}


#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GameConfig {
    #[serde(rename = "gameMode")]
    pub game_mode: String,
    #[serde(rename = "mapId")]
    pub map_id: i64,
    #[serde(rename = "queueId")]
    pub queue_id: i64,
    #[serde(rename = "pickType")]
    pub pick_type: String,
    #[serde(rename = "maxTeamSize")]
    pub max_team_size: i64,
    #[serde(rename = "maxLobbySize")]
    pub max_lobby_size: i64,
    #[serde(rename = "maxHumanPlayers")]
    pub max_human_players: i64,
    #[serde(rename = "allowablePremadeSizes")]
    pub allowable_premade_sizes: Vec<i64>,
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
    pub custom_team100: Vec<Member>,
    #[serde(rename = "customTeam200")]
    pub custom_team200: Vec<Member>,
    #[serde(rename = "customSpectators")]
    pub custom_spectators: Vec<Member>,
    #[serde(rename = "customSpectatorPolicy")]
    pub custom_spectator_policy: String,
    #[serde(rename = "customRewardsDisabledReasons")]
    pub custom_rewards_disabled_reasons: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MucJwtDto {
    pub jwt: String,
    #[serde(rename = "channelClaim")]
    pub channel_claim: String,
    pub domain: String,
    #[serde(rename = "targetRegion")]
    pub target_region: String,
}

