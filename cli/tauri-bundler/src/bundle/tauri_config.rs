use super::category::AppCategory;
use anyhow::anyhow;
use serde::Deserialize;
use std::path::PathBuf;

use std::fs;

#[derive(PartialEq, Deserialize, Clone, Debug, Default)]
#[serde(tag = "deb", rename_all = "camelCase")]
pub struct DebConfig {
  pub depends: Option<Vec<String>>,
  #[serde(default)]
  pub use_bootstrapper: bool,
}

#[derive(PartialEq, Deserialize, Clone, Debug, Default)]
#[serde(tag = "osx", rename_all = "camelCase")]
pub struct OsxConfig {
  pub frameworks: Option<Vec<String>>,
  pub minimum_system_version: Option<String>,
  pub exception_domain: Option<String>,
  pub license: Option<String>,
  #[serde(default)]
  pub use_bootstrapper: bool,
}

#[derive(PartialEq, Deserialize, Clone, Debug, Default)]
#[serde(tag = "bundle", rename_all = "camelCase")]
pub struct BundleConfig {
  pub name: Option<String>,
  pub identifier: Option<String>,
  pub icon: Option<Vec<String>>,
  pub version: Option<String>,
  pub resources: Option<Vec<String>>,
  pub copyright: Option<String>,
  pub category: Option<AppCategory>,
  pub short_description: Option<String>,
  pub long_description: Option<String>,
  pub script: Option<PathBuf>,
  #[serde(default)]
  pub deb: DebConfig,
  #[serde(default)]
  pub osx: OsxConfig,
  pub external_bin: Option<Vec<String>>,
}

#[derive(PartialEq, Deserialize, Clone, Debug, Default)]
#[serde(tag = "tauri", rename_all = "camelCase")]
pub struct TauriConfig {
  #[serde(default)]
  pub bundle: BundleConfig,
}

#[derive(PartialEq, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Config {
  #[serde(default)]
  pub tauri: TauriConfig,
}

pub fn get() -> crate::Result<Config> {
  match std::env::var_os("TAURI_CONFIG") {
    Some(config) => {
      let json = &config.into_string().expect("failed to read TAURI_CONFIG");
      Ok(serde_json::from_str(json)?)
    },
    None => match std::env::var_os("TAURI_DIR") {
      Some(tauri_dir) => {
        let tauri_dir_str = tauri_dir.into_string().expect("failed to read TAURI_DIR");
        let json = &fs::read_to_string(format!("{}{}", tauri_dir_str, "/tauri.conf.json"))?;
        Ok(serde_json::from_str(json)?)
      },
      None => Err(anyhow!("Couldn't get tauri config; please specify the TAURI_CONFIG or TAURI_DIR environment variables"))
    }
  }
}
