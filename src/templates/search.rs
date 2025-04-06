use crate::TERA;

use axum::response::{Html, IntoResponse};
use serde::Serialize;
use tera_hot_reload::TeraTemplate;

#[derive(TeraTemplate, Serialize)]
#[template(path = "searchPage.html")]
pub struct SearchTemplate {
  current_page: String,
}

pub async fn get_search() -> impl IntoResponse {
  let context = SearchTemplate { current_page: "String".to_string() };

  Html(context.render(TERA.read().unwrap().clone()))
}