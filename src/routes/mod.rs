use axum::{
  middleware::from_fn,
  routing::{post, get},
  Router,
};

use crate::{service::shortcut::ShortcutService};

use self::{
  middleware::get_htmx_header,
  search::{get_shortcuts},
};

pub mod middleware;
pub mod search;

pub fn create_routes() -> Router<ShortcutService> {
  Router::new()
      .route("/search", get(get_shortcuts))
      .layer(from_fn(get_htmx_header))
}