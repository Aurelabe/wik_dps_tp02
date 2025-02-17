# WIK DPS TP02

## Description
Ce projet est un serveur HTTP simple en Rust, développé dans le cadre du TP02 du module WIK-DPS. Il utilise l'écosystème Rust pour construire et exécuter un serveur qui répond aux requêtes HTTP de base.

## Objectif du TP
- Créer une image Docker du serveur en utilisant deux approches :
  - **Single-stage build** : Construction et exécution dans une seule image.
  - **Multi-stage build** : Compilation dans une image temporaire puis utilisation d'une image finale allégée pour l'exécution.
- Scanner les images Docker pour identifier des vulnérabilités avec Trivy.

## Prérequis
- Rust (version 1.84.1 ou plus récente)
- Docker
- Trivy (pour l'analyse des vulnérabilités)

## Installation

### Compilation locale
```bash
cargo build
```

### Exécution
```bash
cargo run
```

## Construction des images Docker

### Single-stage build
```bash
sudo docker build -t wik_dps_tp02_single .
```

### Multi-stage build
```bash
sudo docker build -t wik_dps_tp02_multi -f Dockerfile.multi .
```

## Analyse de l'image avec Trivy

### Pour l'image single-stage
```bash
trivy image wik_dps_tp02_single
```

### Pour l'image multi-stage
```bash
trivy image wik_dps_tp02_multi
```

## Utilisation

### Lancer le conteneur Single-stage
```bash
sudo docker run -p 8080:8080 wik_dps_tp02_single
```

### Lancer le conteneur Multi-stage
```bash
sudo docker run -p 8080:8080 wik_dps_tp02_multi
```

Le serveur sera accessible sur `http://localhost:8080`.

## Auteur
- Aurelabe

## Liens utiles
- [Documentation Rust](https://doc.rust-lang.org/)
- [Docker](https://docs.docker.com/)
- [Trivy](https://aquasecurity.github.io/trivy/)

