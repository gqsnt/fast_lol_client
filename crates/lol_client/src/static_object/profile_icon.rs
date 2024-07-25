use std::collections::HashMap;
use std::fmt;
use std::fmt::Write;
use std::path::PathBuf;
use iced::widget::image;
use serde::{Deserialize, Deserializer, Serialize};
use serde::de::Error;
use plugin_lol_summoner::LolSummonerSummonerIcon;
use crate::static_object::Image;



#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileIcons{
    pub version: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub data: HashMap<i64, ProfileIcon>,
}


#[derive( Debug, Clone, Serialize, Deserialize)]
pub struct ProfileIcon {
    #[serde(deserialize_with = "deserialize_i64_from_any")]
    pub id: i64,
    pub image: Image,
}




fn deserialize_i64_from_any<'de, D>(deserializer: D) -> Result<i64, D::Error>
where
    D: Deserializer<'de>,
{
    struct StringOrI64;

    impl<'de> serde::de::Visitor<'de> for StringOrI64 {
        type Value = i64;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("an i64 or a string representing an i64")
        }

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            value.parse::<i64>().map_err(E::custom)
        }

        fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(value)
        }

        fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            if value <= i64::MAX as u64 {
                Ok(value as i64)
            } else {
                Err(E::custom("u64 value too large for i32"))
            }
        }
    }

    deserializer.deserialize_any(StringOrI64)
}