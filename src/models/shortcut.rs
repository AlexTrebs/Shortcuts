use crate::utils::system_util::get_timestamp;
use axum::http::StatusCode;
use serde::{Deserialize, Serialize};
use url::Url;
use uuid::Uuid;

#[derive(Clone, Serialize, Deserialize)]
pub struct Shortcut {
  pub id: Uuid,
  pub created: i64,
  pub updated: i64,
  pub keyword: String,
  pub url: String,
}

impl Shortcut { 
  pub fn new(keyword: String, url: String) -> Self {
    let timestamp = get_timestamp();
    Self {
        id: Uuid::new_v4(),
        created: timestamp,
        updated: timestamp,
        keyword,
        url: url,
    }
}
}

#[derive(Deserialize)]
pub struct SearchRequest {
    pub keyword: String,
}

#[derive(Deserialize)]
pub struct PostRequest {
    pub keyword: String,
    pub url: String,
}

#[derive(Serialize)]
pub struct ListShortcutsResponse {
    pub shortcuts: Vec<Shortcut>,
    pub success: bool,
    pub error: String,
    pub status_code: u16,
}

impl ListShortcutsResponse {
  pub fn success(shortcuts: Vec<Shortcut>) -> Self {
      Self {
          status_code: StatusCode::OK.as_u16(),
          success: true,
          error: "".to_string(),
          shortcuts,
      }
  }

  pub fn error(status_code: u16, error: String) -> Self {
      Self {
          status_code,
          success: false,
          error,
          shortcuts: vec![],
      }
  }
}