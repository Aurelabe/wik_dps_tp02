# Serveur HTTP en Rust

Ce projet implémente un serveur HTTP simple en Rust. Le serveur écoute les requêtes TCP et répond de manière spécifique en fonction de la route demandée.

## Fonctionnalités

- Le serveur écoute sur un port configurable via la variable d'environnement `PING_LISTEN_PORT`. Par défaut, il écoute sur le port `8001`.
- Pour une requête `GET /ping`, le serveur retourne un code de statut `200 OK` et les en-têtes de la requête dans un format JSON.
- Pour toute autre requête, le serveur répond avec un code de statut `404 NOT FOUND`.

## Lancer le serveur

### Prérequis

- Rust installé sur votre machine. Si ce n'est pas déjà fait, suivez la [documentation officielle de Rust](https://www.rust-lang.org/learn/get-started).

### Démarrer le serveur

1. Clonez ce dépôt :

   ```bash
   git clone https://github.com/votre-utilisateur/votre-repository.git
   cd votre-repository
   ```

2. Compilez et exécutez le serveur :

   ```bash
   cargo run
   ```

   Par défaut, le serveur écoutera sur le port `8001`. Vous pouvez changer ce port en définissant la variable d'environnement `PING_LISTEN_PORT` :

   ```bash
   PING_LISTEN_PORT=9000 cargo run
   ```

## Tester avec `curl`

Vous pouvez tester le serveur en envoyant des requêtes `GET` :

1. Tester la route `/ping` :

   ```bash
   curl -i localhost:8001/ping
   ```

   Exemple de réponse pour `/ping` :

   ```json
   HTTP/1.1 200 OK
   Content-Type: application/json
   Content-Length: 84

   {"headers": "Host: localhost:8001\r\nUser-Agent: curl/8.8.0\r\nAccept: */*"}
   ```

2. Tester une route non définie (par exemple `/pinggg`) qui renverra une erreur `404` :

   ```bash
   curl -i localhost:8001/pinggg
   ```

   Exemple de réponse pour toute autre route :

   ```plaintext
   HTTP/1.1 404 NOT FOUND
   ```

## Dépendances

Ce projet n'utilise aucune bibliothèque externe, il se base uniquement sur les bibliothèques standards de Rust.
