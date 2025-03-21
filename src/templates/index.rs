use serde::Serialize;
use tera_hot_reload::{TeraTemplate};

#[derive(TeraTemplate, Serialize)]
#[template(path = "index.html")]
struct IndexTemplate {}