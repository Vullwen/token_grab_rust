use std::thread;
use std::time::Duration;
use base64::{engine::general_purpose, Engine as _};
use rand::Rng;

pub fn obfuscate_string(input: &str) -> String {
    general_purpose::STANDARD.encode(input.as_bytes())
}

pub fn deobfuscate_string(input: &str) -> String {
    match general_purpose::STANDARD.decode(input) {
        Ok(bytes) => String::from_utf8_lossy(&bytes).to_string(),
        Err(_) => String::new(),
    }
}

pub fn detect_debugging() -> bool {
    std::env::vars().any(|(k, _)| k.to_lowercase().contains("debug"))
}

pub fn detect_virtualization() -> bool {
    std::path::Path::new("/.dockerenv").exists() ||
    std::fs::read_to_string("/proc/cpuinfo").map(|s| s.to_lowercase().contains("hypervisor"))
    .unwrap_or(false)
}

pub fn encrypt_with_key(data: &[u8], key: &[u8]) -> Vec<u8> {
    data.iter()
        .enumerate()
        .map(|(i, &b)| b ^ key[i % key.len()])
        .collect()
}

pub fn decrypt_with_key(data: &[u8], key: &[u8]) -> Vec<u8> {
    encrypt_with_key(data, key)
}

pub fn apply_anti_analysis_delay() {
    let millis = rand::thread_rng().gen_range(500..=2000);
    thread::sleep(Duration::from_millis(millis));
}
