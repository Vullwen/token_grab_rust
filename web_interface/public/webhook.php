<?php
// Récupération du corps brut de la requête
$raw = file_get_contents('php://input');

// Si le contenu est envoyé en application/x-www-form-urlencoded
if (empty($raw) && isset($_POST['content'])) {
    $raw = $_POST['content'];
}

$data = json_decode($raw, true);
if (json_last_error() !== JSON_ERROR_NONE) {
    http_response_code(400);
    exit('Erreur : JSON invalide ou déchiffrement échoué');
}

// Validation et nettoyage des champs
$entry = [
    'ip' => isset($data['ip']) && filter_var($data['ip'], FILTER_VALIDATE_IP) ? $data['ip'] : 'Inconnue',
    'location' => isset($data['location']) ? substr($data['location'], 0, 50) : 'Inconnue',
    'tokens' => isset($data['tokens']) && is_array($data['tokens']) ? array_slice($data['tokens'], 0, 20) : [],
    'system_info' => [
        'os'  => isset($data['system_info']['os'])  ? substr($data['system_info']['os'], 0, 30)  : 'Inconnu',
        'cpu' => isset($data['system_info']['cpu']) ? substr($data['system_info']['cpu'], 0, 30) : 'Inconnu',
        'ram' => isset($data['system_info']['ram']) ? (int)$data['system_info']['ram']         : 0
    ],
    'timestamp' => isset($data['timestamp']) ? (int)$data['timestamp'] : time()
];

// Lecture et mise à jour de data.json
$filename = __DIR__ . '/../data/data.json';
$current_data = file_exists($filename)
    ? json_decode(file_get_contents($filename), true)
    : [];

if (!is_array($current_data)) {
    $current_data = [];
}

// On ajoute la nouvelle entrée en tête (max 100 entrées)
array_unshift($current_data, $entry);
file_put_contents($filename, json_encode(array_slice($current_data, 0, 100), JSON_UNESCAPED_UNICODE|JSON_PRETTY_PRINT));

// Journalisation pour débogage
$logfile = __DIR__ . '/../data/debug.log';
file_put_contents($logfile, date('Y-m-d H:i:s') . " — Données enregistrées avec succès\n", FILE_APPEND);

// Réponse au client
echo "Succès";
