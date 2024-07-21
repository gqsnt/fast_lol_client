
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use serde_json::{json, Value, to_value};
use reqwest::Method;
use common::IsApiRequest;

mod additional;

// ENDPOINTS

pub struct PutLolLoadoutsV4LoadoutsById {

    pub id: String,
    pub body: LolLoadoutsUpdateLoadoutDto,
}

impl IsApiRequest for PutLolLoadoutsV4LoadoutsById {
    const METHOD: Method = Method::PUT;
    type ReturnType = LolLoadoutsScopedLoadout;

    fn get_url(&self) -> String {
        format!("/lol-loadouts/v4/loadouts/{}", self.id)
    }

    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn put_lol_loadouts_v_4_loadouts_by_id(id: String, body: LolLoadoutsUpdateLoadoutDto) -> PutLolLoadoutsV4LoadoutsById {
    PutLolLoadoutsV4LoadoutsById {
        id, body
    }
}


pub struct PatchLolLoadoutsV4LoadoutsById {

    pub id: String,
    pub body: LolLoadoutsUpdateLoadoutDto,
}

impl IsApiRequest for PatchLolLoadoutsV4LoadoutsById {
    const METHOD: Method = Method::PATCH;
    type ReturnType = LolLoadoutsScopedLoadout;

    fn get_url(&self) -> String {
        format!("/lol-loadouts/v4/loadouts/{}", self.id)
    }

    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn patch_lol_loadouts_v_4_loadouts_by_id(id: String, body: LolLoadoutsUpdateLoadoutDto) -> PatchLolLoadoutsV4LoadoutsById {
    PatchLolLoadoutsV4LoadoutsById {
        id, body
    }
}


pub struct DeleteLolLoadoutsV4LoadoutsById {

    pub id: String,
}

impl IsApiRequest for DeleteLolLoadoutsV4LoadoutsById {
    const METHOD: Method = Method::DELETE;
    type ReturnType = Value;

    fn get_url(&self) -> String {
        format!("/lol-loadouts/v4/loadouts/{}", self.id)
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn delete_lol_loadouts_v_4_loadouts_by_id(id: String) -> DeleteLolLoadoutsV4LoadoutsById {
    DeleteLolLoadoutsV4LoadoutsById {
        id
    }
}


pub struct GetLolLoadoutsV1LoadoutsReady {

}

impl IsApiRequest for GetLolLoadoutsV1LoadoutsReady {
    const METHOD: Method = Method::GET;
    type ReturnType = bool;

    fn get_url(&self) -> String {
        "/lol-loadouts/v1/loadouts-ready".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_loadouts_v_1_loadouts_ready() -> GetLolLoadoutsV1LoadoutsReady {
    GetLolLoadoutsV1LoadoutsReady {
        
    }
}


pub struct GetLolLoadoutsV4LoadoutsByLoadoutId {

    pub loadout_id: String,
}

impl IsApiRequest for GetLolLoadoutsV4LoadoutsByLoadoutId {
    const METHOD: Method = Method::GET;
    type ReturnType = LolLoadoutsScopedLoadout;

    fn get_url(&self) -> String {
        format!("/lol-loadouts/v4/loadouts/{}", self.loadout_id)
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_loadouts_v_4_loadouts_by_loadout_id(loadout_id: String) -> GetLolLoadoutsV4LoadoutsByLoadoutId {
    GetLolLoadoutsV4LoadoutsByLoadoutId {
        loadout_id
    }
}


pub struct GetLolLoadoutsV4LoadoutsScopeAccount {

}

impl IsApiRequest for GetLolLoadoutsV4LoadoutsScopeAccount {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<LolLoadoutsScopedLoadout>;

    fn get_url(&self) -> String {
        "/lol-loadouts/v4/loadouts/scope/account".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_loadouts_v_4_loadouts_scope_account() -> GetLolLoadoutsV4LoadoutsScopeAccount {
    GetLolLoadoutsV4LoadoutsScopeAccount {
        
    }
}


pub struct GetLolLoadoutsV4LoadoutsScopeByScopeByScopeItemId {

    pub scope: String,
    pub scope_item_id: u32,
}

impl IsApiRequest for GetLolLoadoutsV4LoadoutsScopeByScopeByScopeItemId {
    const METHOD: Method = Method::GET;
    type ReturnType = Vec<LolLoadoutsScopedLoadout>;

    fn get_url(&self) -> String {
        format!("/lol-loadouts/v4/loadouts/scope/{}/{}", self.scope, self.scope_item_id)
    }

    fn get_body(&self) -> Option<Value> {
        None
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn get_lol_loadouts_v_4_loadouts_scope_by_scope_by_scope_item_id(scope: String, scope_item_id: u32) -> GetLolLoadoutsV4LoadoutsScopeByScopeByScopeItemId {
    GetLolLoadoutsV4LoadoutsScopeByScopeByScopeItemId {
        scope, scope_item_id
    }
}


pub struct PostLolLoadoutsV4Loadouts {

    pub body: LolLoadoutsCreateLoadoutDto,
}

impl IsApiRequest for PostLolLoadoutsV4Loadouts {
    const METHOD: Method = Method::POST;
    type ReturnType = LolLoadoutsScopedLoadout;

    fn get_url(&self) -> String {
        "/lol-loadouts/v4/loadouts".to_string()
    }

    fn get_body(&self) -> Option<Value> {
        Some(to_value(&self.body).unwrap())
    }

    fn get_query_params(&self) -> Option<Value> {
        None
    }
}

pub fn post_lol_loadouts_v_4_loadouts(body: LolLoadoutsCreateLoadoutDto) -> PostLolLoadoutsV4Loadouts {
    PostLolLoadoutsV4Loadouts {
        body
    }
}


// OBJECTS

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolLoadoutsUpdateLoadoutDto {
    pub id: String,
    pub name: String,
    pub loadout: LolLoadoutsItemKey,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolLoadoutsItemKey {
    pub inventory_type: String,
    pub content_id: String,
    pub item_id: i32,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolLoadoutsScopedLoadout {
    pub scope: String,
    pub item_id: Option<u32>,
    pub name: String,
    pub loadout: LolLoadoutsItemKey,
    pub refresh_time: String,
    pub id: String,
}


#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolLoadoutsCreateLoadoutDto {
    pub scope: String,
    pub item_id: Option<u32>,
    pub name: String,
    pub loadout: LolLoadoutsItemKey,
    pub refresh_time: String,
}


// ENUMS

