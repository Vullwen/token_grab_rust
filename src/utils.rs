use reqwest::Client;
use serde::Deserialize;
use std::error::Error;
use std::io::Read;
use std::net::TcpStream;
use std::time::{SystemTime, UNIX_EPOCH, Duration};
use std::thread;
use rand::Rng;
use sysinfo::{CpuExt, System, SystemExt};


#[derive(Deserialize)]
struct IpApiResponse {
    status: String,
    country: Option<String>,
    city: Option<String>,
}

pub async fn get_public_ip() -> Result<Option<String>, Box<dyn std::error::Error>> {
    let ip = Client::new()
        .get("https://api.ipify.org")
        .send()
        .await?
        .text()
        .await?
        .trim()
        .to_string();
    Ok(if ip.is_empty() { None } else { Some(ip) })
}

pub async fn get_geolocation(ip: &str) -> Option<(String, String)> {
    let url = format!("http://ip-api.com/json/{}?fields=status,country,city", ip);
    let resp = reqwest::Client::new()
        .get(&url)
        .send()
        .await
        .ok()?
        .json::<IpApiResponse>()
        .await
        .ok()?;
    if resp.status != "success" {
        return None;
    }
    Some((
        resp.city.unwrap_or_default(),
        resp.country.unwrap_or_default(),
    ))
}

pub fn get_system_info() -> Vec<String> {
    let mut sys = System::new_all();
    sys.refresh_all();

    let os_name = sys.name().unwrap_or_else(|| "Inconnu".to_string());
    let os_version = sys.os_version().unwrap_or_else(|| "Inconnu".to_string());

    let cpu_brand = sys
        .cpus()
        .first()
        .map(|c| c.brand().to_string())
        .unwrap_or_else(|| "Inconnu".to_string());
    let cpu_freq = sys
        .cpus()
        .first()
        .map(|c| c.frequency().to_string())
        .unwrap_or_else(|| "0".to_string());

    let total_ram_mb = sys.total_memory() / 1024 / 1024 / 1024; 

    let info_list = vec![
        format!("{}{}", os_name, os_version),
        format!("{}{} MHz", cpu_brand, cpu_freq),
        format!("{} GB", total_ram_mb),
    ];
    info_list
}

pub fn get_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

pub fn random_delay(min_ms: u64, max_ms: u64) {
    if min_ms > max_ms {
        return;
    }
    
    let mut rng = rand::thread_rng();
    let delay_ms = rng.gen_range(min_ms..=max_ms);
    
    thread::sleep(Duration::from_millis(delay_ms));
}