use std::collections::HashMap;

use lazy_static::lazy_static;
use serde::Deserialize;
use serde_json::from_str;

const DEFAULT_VERSIONS: &str = include_str!("../../versions.json");

#[derive(Deserialize)]
pub struct VersionConfig {
    pub asset_bundle_url: String,
    pub ex_resource_url: String,
    pub lua_url: String,
    pub lua_version: String,
}

lazy_static! {
    pub static ref INSTANCE: HashMap<String, VersionConfig> = {
        let local_config = std::path::Path::new("versions.json");
        let data = if local_config.exists() {
            std::fs::read_to_string("versions.json").unwrap()
        } else {
            std::fs::write("versions.json", DEFAULT_VERSIONS).unwrap();
            DEFAULT_VERSIONS.to_string()
        };

        from_str(&data).unwrap()
    };
}
