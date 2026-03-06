# Runtime Anomaly Detector

Prototype développé dans le cadre du cours de Sécurité Logicielle.

## Description
Ce projet vise à développer un prototype capable de détecter des activités anormales à l’exécution d’un programme Linux à partir de l’analyse de ses appels système.

Le prototype repose sur :
- la collecte des traces avec `strace`
- un parseur en Rust
- un moteur de règles de détection
- l’émission d’alertes lisibles
- la génération automatique de rapports d’analyse

---

# Objectif

Surveiller le comportement d’un programme et détecter certains comportements suspects, par exemple :

- l’exécution d’un shell (`/bin/sh`, `/bin/bash`)
- l’accès à des fichiers sensibles (`/etc/passwd`, `/etc/shadow`)
- l’exécution d’un binaire situé dans `/tmp`

---

# Architecture

Programme cible  
→ `strace`  
→ fichier de trace  
→ parseur Rust  
→ moteur de règles  
→ alertes + rapport Markdown

---

# État actuel

Le prototype initial est fonctionnel et permet déjà :

- de parser les appels système `execve`
- de parser les appels système `openat`
- de détecter l’exécution de `/bin/sh`
- de détecter l’accès à `/etc/passwd`
- de détecter l’accès à `/etc/shadow`
- de détecter l’exécution depuis `/tmp`
- de générer un rapport Markdown pour chaque analyse

---

# Règles implémentées

- **R1** : `execve("/bin/sh")` ou `execve("/bin/bash")` → `CRITICAL`
- **R2** : `openat(..., "/etc/passwd", ...)` → `WARNING`
- **R3** : `openat(..., "/etc/shadow", ...)` → `CRITICAL`
- **R4** : `execve("/tmp/...")` → `WARNING`

---

# Structure du projet


runtime-anomaly-detector
│
├── detector
│ └── src
│ ├── main.rs
│ ├── parser.rs
│ ├── rules.rs
│ └── model.rs
│
├── experiments
│ ├── traces
│ └── output
│
└── docs
├── progress.md
└── roadmap.md


---
# Prérequis

Pour utiliser ce prototype, les outils suivants doivent être installés :

- **Linux** (le projet a été testé sous Ubuntu)
- **Rust** (installé via `rustup`)
- **cargo** (gestionnaire de compilation Rust)
- **strace** (outil de capture des appels système)
- **gcc** (pour compiler les programmes de test)

---

# Installation

Cloner le dépôt :

```bash
git clone https://github.com/lucidask/runtime-anomaly-detector.git
cd runtime-anomaly-detector

Compiler le détecteur :

cd detector
cargo build

# Générer une trace

Exemple :

```bash
strace -f -tt -s 200 -o experiments/traces/ls.log ls
Lancer le détecteur

Depuis le dossier detector :

cargo run -- --input ../experiments/traces/ls.log
Rapports générés

Chaque analyse produit automatiquement un rapport Markdown dans :

experiments/output/

Exemples :

report_ls.md
report_passwd.md
report_shadow.md
report_tmp_exec.md
report_suspicious_shell.md

Ces rapports contiennent :

le fichier de trace analysé

le nombre d’événements parsés

les alertes détectées

les lignes de trace correspondantes
