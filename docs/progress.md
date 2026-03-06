Progression du projet
Étape 1 — Mise en place de l’environnement

Le projet a été initialisé en Rust avec cargo.

Structure principale du projet :

runtime-anomaly-detector
│
├── detector
│   ├── src
│   │   ├── main.rs
│   │   ├── parser.rs
│   │   ├── rules.rs
│   │   └── model.rs
│
├── experiments
│   ├── traces
│   └── output
│
└── docs

Les outils utilisés sont :

Linux

Rust

cargo

strace

Étape 2 — Observation du comportement normal

Commande exécutée :

strace -f -tt -s 200 -o experiments/traces/ls.log ls

Extrait de trace :

execve("/usr/bin/ls", ["ls"], ...) = 0
openat(AT_FDCWD, "/etc/ld.so.cache", O_RDONLY|O_CLOEXEC) = 3

Analyse avec le détecteur :

cargo run -- --input ../experiments/traces/ls.log

Résultat :

Summary:
Parsed events: 8
Alerts: 0

Conclusion :

Le système ne génère pas d’alerte pour un comportement normal.

Étape 3 — Détection d’un shell

Un programme de test a été créé afin d’exécuter /bin/sh.

Trace obtenue :

execve("./launch_shell", ["./launch_shell"], ...) = 0
execve("/bin/sh", ["/bin/sh"], NULL) = 0

Analyse :

cargo run -- --input ../experiments/traces/suspicious_shell.log

Résultat :

[CRITICAL] PID 6123 - Shell execution detected: /bin/sh

Conclusion :

Le moteur de règles détecte correctement l’exécution d’un shell.

Étape 4 — Accès à un fichier sensible

Commande exécutée :

strace -f -tt -s 200 -o passwd.log cat /etc/passwd

Extrait de trace :

openat(AT_FDCWD, "/etc/passwd", O_RDONLY) = 3

Analyse :

cargo run -- --input ../experiments/traces/passwd.log

Résultat :

[WARNING] Sensitive file access detected: /etc/passwd

Conclusion :

Le système identifie l’accès à un fichier sensible.

Étape 5 — Accès à un fichier critique

Commande exécutée :

strace -f -tt -s 200 -o shadow.log cat /etc/shadow

Résultat :

[CRITICAL] PID 8469 - Critical sensitive file access detected: /etc/shadow

Conclusion :

L’accès au fichier /etc/shadow est détecté comme une activité critique.

Étape 6 — Exécution depuis /tmp

Un programme de test a été compilé dans /tmp.

Trace obtenue :

execve("/tmp/tmp_exec", ["/tmp/tmp_exec"], ...) = 0

Analyse :

cargo run -- --input ../experiments/traces/tmp_exec.log

Résultat :

[WARNING] Execution from temporary directory detected: /tmp/tmp_exec

Conclusion :

Le moteur de règles détecte l’exécution d’un programme depuis un répertoire temporaire.

Étape 7 — Génération automatique de rapports

Le détecteur génère maintenant un rapport Markdown pour chaque analyse.

Exemples de fichiers générés :

experiments/output/report_ls.md
experiments/output/report_passwd.md
experiments/output/report_shadow.md
experiments/output/report_tmp_exec.md
experiments/output/report_suspicious_shell.md

Ces rapports contiennent :

le fichier de trace analysé

le nombre d’événements parsés

les alertes détectées

les lignes de trace correspondantes

Cela permet de conserver une trace claire des analyses réalisées.

État actuel du projet

Le prototype est capable de :

analyser des traces strace

parser certains appels système (execve, openat)

appliquer un moteur de règles simple

détecter plusieurs comportements suspects

générer des rapports d’analyse lisibles

Prochaines étapes

Les prochaines améliorations prévues sont :

ajouter de nouvelles règles de détection

analyser davantage d’appels système

améliorer le parseur

tester le système sur des programmes plus complexes

explorer une analyse en temps réel

💡 Très important :
