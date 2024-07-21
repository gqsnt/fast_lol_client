use std::fmt;
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
use serde::de::Visitor;


#[derive( Debug, Clone, Default, PartialEq, Eq, Hash)]
pub enum LolGameMode{
    #[default]
    CLASSIC,
    ARAM,
    TFT,
    OTHER(String),
}


impl Serialize for LolGameMode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            LolGameMode::CLASSIC => serializer.serialize_str("CLASSIC"),
            LolGameMode::ARAM => serializer.serialize_str("ARAM"),
            LolGameMode::TFT => serializer.serialize_str("TFT"),
            LolGameMode::OTHER(s) => serializer.serialize_str(s),
        }
    }
}

impl<'de> Deserialize<'de> for LolGameMode {
    fn deserialize<D>(deserializer: D) -> Result<LolGameMode, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct LolGameModeVisitor;

        impl<'de> Visitor<'de> for LolGameModeVisitor {
            type Value = LolGameMode;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a string representing a game mode")
            }

            fn visit_str<E>(self, value: &str) -> Result<LolGameMode, E>
            where
                E: de::Error,
            {
                match value {
                    "CLASSIC" => Ok(LolGameMode::CLASSIC),
                    "ARAM" => Ok(LolGameMode::ARAM),
                    "TFT" => Ok(LolGameMode::TFT),
                    other => Ok(LolGameMode::OTHER(other.to_string())),
                }
            }
        }

        deserializer.deserialize_str(LolGameModeVisitor)
    }
}

impl std::fmt::Display for LolGameMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LolGameMode::ARAM => write!(f, "ARAM"),
            LolGameMode::CLASSIC => write!(f, "CLASSIC"),
            LolGameMode::TFT => write!(f, "TFT"),
            LolGameMode::OTHER(s) => write!(f, "{}", s),
        }
    }
}





