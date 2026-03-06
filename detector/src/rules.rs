use crate::model::{Alert, Severity, SyscallEvent};

pub fn apply_rules(event: &SyscallEvent) -> Vec<Alert> {
    let mut alerts = Vec::new();

    if event.syscall == "execve" && (event.target == "/bin/sh" || event.target == "/bin/bash") {
        alerts.push(Alert {
            severity: Severity::Critical,
            message: format!("Shell execution detected: {}", event.target),
            pid: event.pid,
            raw_line: event.raw_line.clone(),
        });
    }

        if event.syscall == "execve" && event.target.starts_with("/tmp/") {
        alerts.push(Alert {
            severity: Severity::Warning,
            message: format!("Execution from temporary directory detected: {}", event.target),
            pid: event.pid,
            raw_line: event.raw_line.clone(),
        });
    }

    if event.syscall == "openat" && event.target == "/etc/passwd" {
        alerts.push(Alert {
            severity: Severity::Warning,
            message: format!("Sensitive file access detected: {}", event.target),
            pid: event.pid,
            raw_line: event.raw_line.clone(),
        });
    }

    if event.syscall == "openat" && event.target == "/etc/shadow" {
        alerts.push(Alert {
            severity: Severity::Critical,
            message: format!("Critical sensitive file access detected: {}", event.target),
            pid: event.pid,
            raw_line: event.raw_line.clone(),
        });
    }

    alerts
}