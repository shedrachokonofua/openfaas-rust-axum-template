use axum::response::Json;
use serde_json::{json, Value};

pub async fn handle() -> Json<Value> {
  Json(json!({ "ok": true }))
}
