<?php
$dataDir  = __DIR__ . '/../data';
$dataFile = $dataDir . '/data.json';
$logFile  = $dataDir . '/debug.log';

$rawBody = file_get_contents('php://input');
$timestamp = date('Y-m-d H:i:s');
file_put_contents($logFile, "[$timestamp] REQUETE BRUTE: " . $rawBody . "\n", FILE_APPEND);

$data = json_decode($rawBody, true);
if (json_last_error() !== JSON_ERROR_NONE) {
    $errMsg = json_last_error_msg();
    file_put_contents($logFile, "[$timestamp] ERREUR JSON: $errMsg\n", FILE_APPEND);
    http_response_code(400);
    echo "Erreur JSON invalide: $errMsg";
    exit;
}
print("JSON valide, traitement en cours...\n");
file_put_contents($logFile, "[$timestamp] DONNÉES DÉCODEES: " . print_r($data, true) . "\n", FILE_APPEND);

$entry = [
    'ip'         => (isset($data['ip']) && filter_var($data['ip'], FILTER_VALIDATE_IP)) ? $data['ip'] : 'Inconnue',
    'location'   => isset($data['location']) ? substr($data['location'], 0, 50) : 'Inconnue',
    'tokens'     => (isset($data['tokens']) && is_array($data['tokens'])) ? array_slice($data['tokens'], 0, 20) : [],
    'system_info'=> [
        'os'  => isset($data['system_info']['os']) ? substr($data['system_info']['os'], 0, 30) : 'Inconnu',
        'cpu' => isset($data['system_info']['cpu']) ? substr($data['system_info']['cpu'], 0, 30) : 'Inconnu',
        'ram' => isset($data['system_info']['ram']) ? (int)$data['system_info']['ram'] : 0
    ],
    'timestamp'  => isset($data['timestamp']) ? (int)$data['timestamp'] : time()
];

file_put_contents($logFile, "[$timestamp] ENTRY FORMATÉE: " . print_r($entry, true) . "\n", FILE_APPEND);

if (!is_dir($dataDir)) {
    mkdir($dataDir, 0755, true);
}
$current = file_exists($dataFile)
    ? json_decode(file_get_contents($dataFile), true)
    : [];
if (!is_array($current)) {
    $current = [];
    file_put_contents($logFile, "[$timestamp] WARNING: data.json corrompu, réinitialisation.\n", FILE_APPEND);
}

array_unshift($current, $entry);
$current = array_slice($current, 0, 100);
$result = file_put_contents($dataFile, json_encode($current, JSON_UNESCAPED_UNICODE | JSON_PRETTY_PRINT));
if ($result === false) {
    file_put_contents($logFile, "[$timestamp] ERREUR: impossibilité d'écrire dans data.json\n", FILE_APPEND);
    http_response_code(500);
    echo "Erreur interne: impossible d’enregistrer";
    exit;
}

file_put_contents($logFile, "[$timestamp] SUCCÈS: entrée enregistrée. Total=" . count($current) . "\n", FILE_APPEND);
http_response_code(200);
echo "Succès: entrée ajoutée (Total entries=" . count($current) . ")";
