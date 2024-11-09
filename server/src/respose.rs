use axum::Json;
use serde_json::json;

#[macro_export]
macro_rules! json_response {
    ($($json:tt)+) => {
        Json(json!({
            "data": json!($($json)+),
            "success": true
        }))
    };
}