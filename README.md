# token_grab_rust 
Token grabber en Rust pour le partiel
Réalisé par VullWen et GoldFire94

**Fonctionnalités clés**  
- 🔍 Extraction automatisée de tokens Discord  
- 📤 Exfiltration sécurisée via webhook
- 📦 Collecte d'informations système détaillées
- 🌐 Interface de monitoring web  

**Données collectées**  
- 🔑 Tokens d'authentification  
- 🌐 IP publique + géolocalisation approximative  
- 🖥️ User agent détaillé + configuration système  
- ⏱️ Horodatage des activités  

**Compilation**  
Mode Release : 
```bash
cargo build --release
./target/release/token_grabber_rust
```
Mode Debug : 
```bash
cargo build --release
./target/release/token_grabber_rust -d
