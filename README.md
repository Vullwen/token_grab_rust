# token_grab_rust 
Token grabber en Rust pour le partiel de cybersécurité

**Fonctionnalités clés**  
- 🎯 Injection de code dans les processus cibles (Discord/Apps web)  
- 🔍 Extraction automatisée de tokens (Discord, sessions navigateurs)  
- 📤 Exfiltration sécurisée via webhook Discord chiffré  
- 🌐 Interface de monitoring web  

**Données collectées**  
- 🔑 Tokens d'authentification  
- 🌐 IP publique + géolocalisation approximative  
- 🖥️ User agent détaillé + configuration système  
- ⏱️ Horodatage des activités  

**Techniques d'obfuscation**  
- Patternes regex
- Appels réseau
- Appels système
- Chiffrement des strings
- Anti débug
- Anti VM/sandbox

**Compilation**  
- 🦀 Rust 1.67+ requis  
- 📦 Crates essentielles : `winapi`, `reqwest`, `sysinfo`  
- 🔧 Flags recommandés : `--release -C opt-level=z -C panic=abort`  


