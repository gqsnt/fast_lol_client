
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use serde_json::{json, Value, to_value};
use std::collections::hash_map::Values;
use reqwest::Method;
use common::IsApiRequest;

mod additional;

// ENDPOINTS

pub struct GetLolKrPlaytimeReminderV1HoursPlayed {}

impl IsApiRequest for GetLolKrPlaytimeReminderV1HoursPlayed {
    const METHOD: Method = Method::GET;
    type ReturnType = u32;
    fn get_url(&self) -> String {"/lol-kr-playtime-reminder/v1/hours-played".to_string()}
}

pub fn get_lol_kr_playtime_reminder_v1_hours_played() -> GetLolKrPlaytimeReminderV1HoursPlayed {
    GetLolKrPlaytimeReminderV1HoursPlayed{}
}


// OBJECTS


// ENUMS

