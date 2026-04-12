# Feuille de route du projet

---

## Phase 1 — Analyse des syscalls

### Objectif
Comprendre et analyser le comportement système d’un programme.

### Étapes

- Étude de `strace`
- Analyse de programmes simples
- Implémentation du parser syscall

---

## Phase 2 — Moteur de détection syscall

### Objectif
Détecter des comportements suspects.

### Implémentations

- execve (shell)
- openat (fichiers sensibles)
- exécution depuis `/tmp`

---

## Phase 3 — Extension du parser

### Objectif
Améliorer la couverture des traces.

### Ajouts

- read
- write
- close

---

## Phase 4 — Analyse avancée

### Objectif
Détecter des comportements complexes.

### Implémentations

- fréquence anormale
- séquences suspectes (fenêtre)

---

## Phase 5 — Exploitation des résultats

### Objectif
Rendre les résultats exploitables.

### Implémentations

- affichage terminal structuré
- génération Markdown
- génération JSON
- organisation des outputs

---

## Phase 6 — Intégration mémoire

### Objectif
Étendre l’analyse à la mémoire.

### Implémentations

- parsing logs Valgrind
- classification des erreurs

### Détections

- invalid read/write
- heap overflow
- use-after-free
- invalid free
- uninitialised value
- stack overflow
- segmentation fault

---

## Phase 7 — Analyse combinée

### Objectif
Fusionner les analyses syscall et mémoire.

### Implémentations

- mode `--combined`
- fusion des alertes
- rapport unifié

---

## Phase 8 — Structuration finale

### Objectif
Rendre le projet propre et présentable.

### Implémentations

- séparation parser/rules
- structure des dossiers
- outputs organisés :

```text
syscall / memory / combined
Objectif final atteint

Construire un prototype capable de détecter :

anomalies système
anomalies mémoire
comportements suspects corrélés

dans un environnement structuré et reproductible.

Améliorations futures possibles
corrélation avancée mémoire + syscall
détection comportementale
analyse en temps réel
support de nouveaux syscalls
regroupement intelligent des erreurs

---

## Phase 9 — Amélioration des alertes

### Objectif
Rendre les alertes plus pertinentes et exploitables.

### Implémentations

- filtrage des appels provenant des bibliothèques système
- localisation des anomalies dans le code utilisateur
- amélioration des messages d’erreur

---

## Phase 10 — Fusion intelligente

### Objectif
Réduire le bruit et améliorer la lisibilité.

### Implémentations

- fusion des événements multiples liés à une même anomalie
- regroupement des signaux (stack smashing, SIGABRT, etc.)
- fusion des anomalies sur lignes adjacentes

---

## Phase 11 — Validation expérimentale

### Objectif
Tester le système sur des cas réalistes.

### Implémentations

- programmes vulnérables (buffer overflow)
- tests mémoire complets
- scénarios combinés (full_test)

---

## Objectif final atteint

Construire un prototype capable de détecter :

- anomalies système
- anomalies mémoire
- comportements suspects corrélés

avec :

- alertes compréhensibles
- réduction du bruit
- résultats exploitables

dans un environnement structuré et reproductible.
