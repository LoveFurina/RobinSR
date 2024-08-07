use axum::Json;
use serde_json::json;

pub const LOGIN_WITH_PASSWORD_ENDPOINT: &str = "/:product_name/mdk/shield/api/login";
pub const LOGIN_WITH_SESSION_TOKEN_ENDPOINT: &str = "/:product_name/mdk/shield/api/verify";
pub const GRANTER_LOGIN_VERIFICATION_ENDPOINT: &str = "/:product_name/combo/granter/login/v2/login";
pub const RISKY_API_CHECK_ENDPOINT: &str = "/account/risky/api/check";

#[tracing::instrument]
pub async fn login_with_password() -> Json<serde_json::Value> {
    Json(json!({
    "data": {
        "account": {
            "area_code": "**",
            "email": "LingshaSR",
            "country": "RU",
            "is_email_verify": "1",
            "token": "mostsecuretokenever",
            "uid": "1337"
        },
        "device_grant_required": false,
        "reactivate_required": false,
        "realperson_required": false,
        "safe_mobile_required": false
    },
    "message": "OK",
    "retcode": 0
    }))
}

#[tracing::instrument]
pub async fn login_with_session_token() -> Json<serde_json::Value> {
    Json(json!({
    "data": {
        "account": {
            "area_code": "**",
            "email": "LingshaSR",
            "country": "RU",
            "is_email_verify": "1",
            "token": "mostsecuretokenever",
            "uid": "1337"
        },
        "device_grant_required": false,
        "reactivate_required": false,
        "realperson_required": false,
        "safe_mobile_required": false
    },
    "message": "OK",
    "retcode": 0
    }))
}

#[tracing::instrument]
pub async fn granter_login_verification() -> Json<serde_json::Value> {
    Json(json!({
        "data": {
            "account_type": 1,
            "combo_id": "1337",
            "combo_token": "mostsecuretokenever",
            "data": "{\"guest\":false}",
            "heartbeat": false,
            "open_id": "1337"
        },
        "message": "OK",
        "retcode": 0
    }))
}

#[tracing::instrument]
pub async fn risky_api_check() -> Json<serde_json::Value> {
    Json(json!({
        "data": {
            "id": "mostsecuretokenever",
            "action": "ACTION_NONE",
            "geetest": null
        },
        "message": "OK",
        "retcode": 0
    }))
}
