use reqwest::Method;
use serde_json::Value;
use crate::impl_api_plugin;

impl_api_plugin!(
    "/lol-end-of-game",
    V1{
      PostDismissStats{
          post_dismiss_stats,Method::POST,"/state/dismiss-stats"
      } => Value
    }
);