
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use serde_json::{json, Value, to_value};
use reqwest::Method;
use common::IsApiRequest;

mod additional;

// ENDPOINTS

pub struct GetLolNpeTutorialPathV1RewardsChamp {

}

impl IsApiRequest for GetLolNpeTutorialPathV1RewardsChamp {
    const METHOD: Method = Method::GET;
    type ReturnType = LolNpeTutorialPathCollectionsChampion;

    fn get_url(&self) -> String {
        "/lol-npe-tutorial-path/v1/rewards/champ".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_npe_tutorial_path_v_1_rewards_champ() -> GetLolNpeTutorialPathV1RewardsChamp {
    GetLolNpeTutorialPathV1RewardsChamp {
        
    }
}


pub struct GetLolNpeTutorialPathV1Settings {

}

impl IsApiRequest for GetLolNpeTutorialPathV1Settings {
    const METHOD: Method = Method::GET;
    type ReturnType = LolNpeTutorialPathAccountSettingsTutorial;

    fn get_url(&self) -> String {
        "/lol-npe-tutorial-path/v1/settings".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_npe_tutorial_path_v_1_settings() -> GetLolNpeTutorialPathV1Settings {
    GetLolNpeTutorialPathV1Settings {
        
    }
}


pub struct PutLolNpeTutorialPathV1Settings {

    pub body: LolNpeTutorialPathAccountSettingsTutorial,
}

impl IsApiRequest for PutLolNpeTutorialPathV1Settings {
    const METHOD: Method = Method::PUT;
    type ReturnType = Value;

    fn get_url(&self) -> String {
        "/lol-npe-tutorial-path/v1/settings".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn put_lol_npe_tutorial_path_v_1_settings(body: LolNpeTutorialPathAccountSettingsTutorial) -> PutLolNpeTutorialPathV1Settings {
    PutLolNpeTutorialPathV1Settings {
        body
    }
}


pub struct GetLolNpeTutorialPathV1Tutorials {

}

impl IsApiRequest for GetLolNpeTutorialPathV1Tutorials {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<LolNpeTutorialPathTutorial>;

    fn get_url(&self) -> String {
        "/lol-npe-tutorial-path/v1/tutorials".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_npe_tutorial_path_v_1_tutorials() -> GetLolNpeTutorialPathV1Tutorials {
    GetLolNpeTutorialPathV1Tutorials {
        
    }
}


pub struct PatchLolNpeTutorialPathV1TutorialsInit {

}

impl IsApiRequest for PatchLolNpeTutorialPathV1TutorialsInit {
    const METHOD: Method = Method::PATCH;
    type ReturnType = Value;

    fn get_url(&self) -> String {
        "/lol-npe-tutorial-path/v1/tutorials/init".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn patch_lol_npe_tutorial_path_v_1_tutorials_init() -> PatchLolNpeTutorialPathV1TutorialsInit {
    PatchLolNpeTutorialPathV1TutorialsInit {
        
    }
}


pub struct PutLolNpeTutorialPathV1TutorialsByTutorialIdView {

    pub tutorial_id: String,
}

impl IsApiRequest for PutLolNpeTutorialPathV1TutorialsByTutorialIdView {
    const METHOD: Method = Method::PUT;
    type ReturnType = Value;

    fn get_url(&self) -> String {
        format!("/lol-npe-tutorial-path/v1/tutorials/{}/view", self.tutorial_id)
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn put_lol_npe_tutorial_path_v_1_tutorials_by_tutorial_id_view(tutorial_id: String) -> PutLolNpeTutorialPathV1TutorialsByTutorialIdView {
    PutLolNpeTutorialPathV1TutorialsByTutorialIdView {
        tutorial_id
    }
}


// OBJECTS

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolNpeTutorialPathCollectionsChampion {
    pub alias: String,
    pub ban_vo_path: String,
    pub choose_vo_path: String,
    pub id: i32,
    pub name: String,
    pub roles: Vec<String>,
    pub square_portrait_path: String,
    pub stinger_sfx_path: String,
    pub passive: LolNpeTutorialPathCollectionsChampionSpell,
    pub spells: Vec<LolNpeTutorialPathCollectionsChampionSpell>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolNpeTutorialPathTutorial {
    pub id: String,
    pub step_number: i32,
    pub title: String,
    pub description: String,
    pub background_url: String,
    pub queue_id: String,
    pub use_quick_search_matchmaking: bool,
    pub use_chosen_champion: bool,
    pub status: LolNpeTutorialPathTutorialStatus,
    pub is_viewed: bool,
    pub type_: LolNpeTutorialPathTutorialType,
    pub rewards: Vec<LolNpeTutorialPathTutorialReward>,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolNpeTutorialPathCollectionsChampionSpell {
    pub name: String,
    pub description: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolNpeTutorialPathTutorialReward {
    pub reward_type: String,
    pub description: String,
    pub reward_fulfilled: bool,
    pub icon_url: String,
    pub item_id: String,
    pub sequence: i32,
    pub unique_name: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolNpeTutorialPathAccountSettingsTutorial {
    pub has_seen_tutorial_path: bool,
    pub has_skipped_tutorial_path: bool,
    pub should_see_new_player_experience: bool,
}


// ENUMS

#[derive(Serialize, Deserialize, Clone, PartialEq, Default, Debug)]
pub enum LolNpeTutorialPathTutorialStatus {
    #[default]
    #[serde(rename = "COMPLETED")]
    Completed,
    #[serde(rename = "UNLOCKED")]
    Unlocked,
    #[serde(rename = "LOCKED")]
    Locked,
}


#[derive(Serialize, Deserialize, Clone, PartialEq, Default, Debug)]
pub enum LolNpeTutorialPathTutorialType {
    #[default]
    #[serde(rename = "REWARD")]
    Reward,
    #[serde(rename = "CARD")]
    Card,
}

