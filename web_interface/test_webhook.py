import requests
import time


# URL du webhook
webhook_url = 'https://rust.vullwen.com/webhook.php'

# Données fictives pour le test
test_data = {
    'ip': '192.WEEWOOOWEEOOOWEEWOOOWEEWOOO.1.254',
    'location': 'Paris, France',
    'tokens': [
        'WEEWOOOWEEOOOWEEWOOOWEEWOOO',
        'WEEWOOOWEEOOOWEEWOOOWEEWOOO'
    ],
    'system_info': {
        'os': 'WEEWOOOWEEOOOWEEWOOOWEEWOOO 11 Home 64-bit',
        'cpu': 'Intel WEEWOOOWEEOOOWEEWOOOWEEWOOO i7-12700K @ 3.6GHz',
        'ram': 32
    },
    'timestamp': int(time.time())
}

# Envoi de la requête POST sans vérification SSL
print("Envoi des données au webhook...")
response = requests.post(webhook_url, json=test_data, verify=False)

# Affichage du résultat
if response.status_code == 200:
    print(f"✅ Succès ! Code de statut: {response.status_code}")
    print(f"Le webhook a bien reçu les données")
else:
    print(f"❌ Échec ! Code de statut: {response.status_code}")
    print(f"Réponse: {response.text}")
