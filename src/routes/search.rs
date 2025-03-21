use crate::{
  models::{
    response::{BaseResponse, Json},
    shortcut::{ListShortcutsResponse, SearchRequest, Shortcut, PostRequest},
  },
  service::shortcut::{ShortcutService, ShortcutServiceTrait},
  utils::system_util::get_timestamp,
};

use axum::{
  extract::{Path, State},
  http::StatusCode,
  Extension,
  response::{Html, IntoResponse},
};

use serde::Serialize;

use tera::Tera;

pub async fn get_shortcuts(
  Extension(htmx): Extension<bool>,
  State(mut shortcut_service): State<ShortcutService>,
  State(templates): State<Tera>,
  text: String,
) -> Result<impl IntoResponse, ()> {
  let result = shortcut_service.find_similar(text).await;

  return match result {
    Ok(shortcuts) => {
      let rendered = templates.render("search/search_results.html", SearchTemplate {shortcuts, query: text, nav: "index".to_string()});

      Ok(Html(rendered));
    },
    Err(err) => {
      let rendered = templates.render("components/error.html", ErrorTemplate {error: err.to_string(), query: text, nav: "index".to_string()});

      Ok(Html(rendered));
    }
  };
}

#[derive(Serialize)]
struct SearchTemplate {
  nav: String,
  query: String,
  shortcuts: Vec<Shortcut>,
}

#[derive(Serialize)]
struct ErrorTemplate {
  nav: String,
  query: String,
  error: String,
}

