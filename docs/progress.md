# Progression du projet

---

## Étape 1 — Mise en place de l’environnement

Initialisation du projet en Rust avec `cargo`.

Structure initiale :

- parser syscall
- moteur de règles
- affichage terminal

Outils utilisés :

- Linux
- Rust
- cargo
- strace

---

## Étape 2 — Analyse des syscalls

### Tests réalisés

- programme simple (`ls`)
- exécution de `/bin/sh`
- accès à `/etc/passwd`
- accès à `/etc/shadow`
- exécution depuis `/tmp`

### Résultats

- détection correcte des comportements suspects
- pas de faux positifs sur un programme normal

---

## Étape 3 — Amélioration du parser syscall

Ajout du support :

- `read`
- `write`
- `close`

### Résultat

- meilleure couverture des traces réelles
- analyse plus complète

---

## Étape 4 — Détection avancée (syscall)

Implémentation de :

- fréquence anormale (par seconde)
- séquences suspectes (fenêtre d’événements)

### Résultat

- détection de comportements corrélés
- analyse globale du programme

---

## Étape 5 — Génération de rapports

Ajout de :

- rapports Markdown
- rapports JSON
- organisation automatique des outputs

### Résultat

- analyses reproductibles
- sorties exploitables

---

## Étape 6 — Refactorisation du projet

Nouvelle structure :

- séparation `parser/` (syscall + memory)
- séparation `rules/` (syscall + memory)
- séparation `utils/` (display + markdown + json)

### Résultat

- architecture modulaire
- extensibilité facilitée

---

## Étape 7 — Intégration de l’analyse mémoire

Ajout de l’analyse via **Valgrind**.

Détection de :

- invalid read / write
- heap overflow
- use-after-free
- invalid free
- uninitialised value
- stack overflow
- segmentation fault

### Résultat

- détection d’erreurs mémoire critiques
- classification Warning / Critical

---

## Étape 8 — Tests mémoire

Tests réalisés :

- invalid write
- invalid read
- use-after-free
- heap overflow (cas complexes)
- segmentation fault
- stack overflow

### Résultat

- parsing correct des logs Valgrind
- gestion de plusieurs erreurs dans un même programme

---

## Étape 9 — Amélioration des alertes mémoire

Améliorations implémentées :

- filtrage des appels internes aux bibliothèques système (libc)
- priorisation des lignes issues du code utilisateur
- amélioration des messages d’erreur

### Résultat

- alertes plus pertinentes
- réduction du bruit lié aux bibliothèques système
- meilleure compréhension des anomalies

---

## Étape 10 — Fusion intelligente des alertes

Implémentation de :

- fusion des événements liés à une même anomalie
- regroupement des signaux multiples (stack smashing, SIGABRT, etc.)
- fusion des anomalies sur lignes adjacentes

### Résultat

- réduction du nombre d’alertes
- meilleure lisibilité des résultats
- représentation plus fidèle des anomalies réelles

---

## Étape 11 — Tests avancés

Tests réalisés :

- programmes vulnérables avec buffer overflow
- programmes 32 bits (ROP / exploitation)
- tests complets avec `full_test`

### Résultat

- validation du système sur des scénarios réalistes
- robustesse du détecteur confirmée

---

## Étape 12 — Finalisation du projet

Le prototype permet désormais :

- analyse des syscalls
- analyse mémoire (Valgrind)
- analyse combinée
- détection de comportements suspects
- détection d’anomalies mémoire critiques
- fusion intelligente des alertes
- génération de rapports (terminal / Markdown / JSON)

---

## Conclusion

Le projet est devenu un analyseur runtime complet, combinant :

- comportement système
- comportement mémoire

avec une logique de détection cohérente et des résultats exploitables.
