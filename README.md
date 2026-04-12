# 🚨 Runtime Anomaly Detector

![Rust](https://img.shields.io/badge/language-Rust-orange)
![Platform](https://img.shields.io/badge/platform-Linux-blue)
![Status](https://img.shields.io/badge/status-Prototype-yellow)
![License](https://img.shields.io/badge/license-MIT-green)
![Security](https://img.shields.io/badge/focus-Security-red)

---

## 📌 Overview

**Runtime Anomaly Detector** is a dynamic analysis tool designed to detect abnormal behavior during the execution of Linux programs.

It analyzes:

- **system calls** using `strace`
- **memory-related behavior** using `Valgrind`

The detector follows a **rule-based approach** built on deterministic rules and focuses on identifying suspicious runtime activity in vulnerable programs.

---

## 🎯 Project Goals

This prototype was developed to:

- study the limits of static analysis for software security
- analyze runtime behavior instead of source code only
- detect suspicious or abnormal actions during execution
- demonstrate anomaly detection on vulnerable programs
- provide understandable alerts and reports for further analysis

---

## 🧠 Detection Capabilities

### 🔹 Syscall Behavior

The tool currently detects:

- shell execution (`/bin/sh`, `/bin/bash`)
- sensitive file access (`/etc/passwd`, `/etc/shadow`)
- execution from `/tmp`
- high syscall frequency
- suspicious sequences (sensitive access followed by execution)

### 🔹 Memory Behavior

The tool currently detects:

- invalid read
- invalid write
- heap overflow
- use-after-free
- invalid free
- use of uninitialised value
- segmentation fault
- stack corruption / stack smashing
- stack protection failures such as `__stack_chk_fail` and related `SIGABRT`

---

## ✅ Current Improvements

Compared to the initial prototype, the detector now supports:

- **better source-code localisation** for memory anomalies  
  (prefers the real user source file instead of libc/internal frames)
- **more expressive memory alerts**
- **stack corruption detection** through:
  - `stack smashing detected`
  - `__stack_chk_fail`
  - related `SIGABRT`
- **alert fusion / deduplication**
  - multiple runtime signals for the same anomaly are merged into a single alert
- **structured alerts**
  - occurrences count
  - evidence list
  - internal anomaly kind and location fields

---

## 🏗️ Architecture

```text
Target Program
      ↓
Trace Collection (strace / valgrind)
      ↓
Rust Parser
      ↓
Rule Engine
      ↓
Alert Generation
      ↓
Reports (Terminal / Markdown / JSON)

⚙️ Requirements
Linux
Rust / Cargo
strace
valgrind
GCC or Clang for compiling vulnerable C programs
Optional 32-bit support

Some advanced exploitation-oriented test programs use 32-bit inline assembly.
For those programs, you may need:

sudo dpkg --add-architecture i386
sudo apt update
sudo apt install gcc-multilib libc6-dev-i386 libc6-dbg:i386

And compile with:

gcc -m32 -g -o program program.c

📁 Project Structure
runtime-anomaly-detector/
│
├── detector/
│   └── src/
│       ├── main.rs
│       ├── model.rs
│       ├── parser/
│       │   ├── syscall.rs
│       │   └── memory.rs
│       ├── rules/
│       │   ├── syscall.rs
│       │   └── memory.rs
│       └── utils/
│           ├── display.rs
│           ├── json.rs
│           └── markdown.rs
│
├── experiments/
│   ├── programs/
│   │   ├── memory/
│   │   └── syscall/
│   ├── traces/
│   │   ├── memory/
│   │   └── syscall/
│   └── output/
│
└── README.md
🛠️ Build

Move into the detector folder before using Cargo:

cd detector
cargo build

🚀 Usage
1. Generate traces

Always generate traces from the project root.

Compile the target program
gcc -g program.c -o program

If the program is 32-bit specific:

gcc -m32 -g program.c -o program
Syscall trace
strace -f -tt -s 200 -o experiments/traces/syscall/program.log ./program
or
strace -f -tt -o experiments/traces/syscall/program.log ./program
Memory trace
valgrind --leak-check=full ./program 2> experiments/traces/memory/program.log
2. Run the detector

Move into the detector folder:

cd detector
Syscall analysis
cargo run -- --syscall-input ../experiments/traces/syscall/program.log
Memory analysis
cargo run -- --memory-input ../experiments/traces/memory/program.log
Combined analysis
cargo run -- --combined ../experiments/traces/syscall/program.log ../experiments/traces/memory/program.log

🧪 Detection Rules
Syscall Rules
Rule	Description	Severity
R1	Shell execution	CRITICAL
R2	Access to /etc/passwd	WARNING
R3	Access to /etc/shadow	CRITICAL
R4	Execution from /tmp	WARNING
R5	High syscall frequency	WARNING
R6	Suspicious sequence	CRITICAL
Memory Rules
Rule	Description	Severity
M1	Invalid write	CRITICAL
M2	Invalid read	WARNING
M3	Heap overflow	CRITICAL
M4	Use-after-free	CRITICAL
M5	Invalid free	WARNING
M6	Use of uninitialised value	WARNING
M7	Stack corruption / stack smashing	CRITICAL
M8	Segmentation fault	CRITICAL

📤 Output
Terminal
alert list
summary
anomaly messages with probable cause
Markdown
detailed report
parsed event summary
grouped anomaly statistics
JSON
structured alerts
machine-readable output
integration-friendly format

🧾 Alert Format

The detector now tries to produce more useful alerts.

Example:

Stack corruption detected at t1_book_reader.c line: 47.
Likely cause: unsafe stack memory operation or buffer overflow.

For stack protection failures, alerts may mention:

stack smashing detection
__stack_chk_fail
SIGABRT
likely earlier unsafe stack write

🔁 Alert Fusion

When multiple runtime signals correspond to the same underlying anomaly, the detector merges them into a single alert.

Example:

stack smashing detected
SIGABRT
__stack_chk_fail

These can be grouped into one final anomaly such as:

Stack corruption

This prevents redundant alerts and improves report clarity.

🧪 Tested Scenarios
Memory

The detector has been tested on vulnerable C programs involving:

classic stack-based buffer overflow
unsafe strcpy
unsafe gets
incorrect strncpy size usage
stack smashing / canary failure
exploit-oriented 32-bit buffer overflow examples
Syscalls

The detector is designed to be tested on programs involving:

shell execution
sensitive file access
execution from temporary directories
suspicious execution sequences

⚠️ Limitations
limited syscall coverage
rule-based only (no ML)
offline trace analysis
quality of localisation depends on available debug information
Valgrind behavior may depend on the environment, especially for 32-bit binaries
🔮 Future Work

Possible future improvements:

broader syscall coverage
better correlation between memory and syscall events
smarter anomaly aggregation
richer sequence detection
real-time or near real-time monitoring
more robust support for complex binaries and stripped executables

🎓 Academic Context

Project developed for:

Sécurité Logicielle (GLO-4009 / GLO-7009)

The project focuses on runtime anomaly detection for vulnerable Linux programs using behavioral analysis and deterministic detection rules.

👨‍💻 Author

Andler Skenty Bertrand

📄 License

MIT License

⭐ Notes

Important reminders:

compile the target program before analysis
generate strace / valgrind logs from the project root
run cargo from the detector/ folder
keep relative paths consistent
for some advanced 32-bit test programs, install the required 32-bit debug/runtime packages for Valgrind
