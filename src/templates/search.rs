use models::Shortcut;
use serde::Serialize;
use tera_hot_reload::{TeraTemplate};

#[derive(TeraTemplate, Serialize)]
#[template(path = "index.html")]
struct SerachTemplate {
  shortcuts: Vec<Shortcut>;
  query: String;
  loc: String;
}