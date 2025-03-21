use index::IndexTemplate;
use Shortcuts::TERA;
use axum::response::{Html, IntoResponse};

mod index;
mod search;

async fn get_index() -> impl IntoResponse {
  let context = IndexTemplate {};

  Html(context.render(TERA.read().unwrap().clone()))
}