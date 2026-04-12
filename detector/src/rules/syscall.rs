use crate::model::{Alert, Severity, SyscallEvent};
use std::collections::HashMap;

const HIGH_FREQUENCY_THRESHOLD: usize = 50;
const SEQUENCE_WINDOW_SIZE: usize = 5;

pub fn apply_syscall_rules(event: &SyscallEvent) -> Vec<Alert> {
    let mut alerts = Vec::new();

    if is_shell_execution(event) {
        alerts.push(build_alert(
            Severity::Critical,
            format!("Shell execution detected: {}", event.target),
            event.pid,
            event.raw_line.clone(),
        ));
    }

    if is_tmp_execution(event) {
        alerts.push(build_alert(
            Severity::Warning,
            format!("Execution from temporary directory detected: {}", event.target),
            event.pid,
            event.raw_line.clone(),
        ));
    }

    if is_passwd_access(event) {
        alerts.push(build_alert(
            Severity::Warning,
            format!("Sensitive file access detected: {}", event.target),
            event.pid,
            event.raw_line.clone(),
        ));
    }

    if is_shadow_access(event) {
        alerts.push(build_alert(
            Severity::Critical,
            format!("Critical sensitive file access detected: {}", event.target),
            event.pid,
            event.raw_line.clone(),
        ));
    }

    alerts
}

pub fn detect_high_frequency(events: &[SyscallEvent]) -> Vec<Alert> {
    let mut alerts = Vec::new();
    let mut counter: HashMap<String, Vec<&SyscallEvent>> = HashMap::new();

    for event in events {
        let second = extract_second(&event.timestamp);
        counter.entry(second).or_default().push(event);
    }

    for (second, events_in_second) in counter {
        if events_in_second.len() > HIGH_FREQUENCY_THRESHOLD {
            alerts.push(build_alert(
                Severity::Warning,
                format!(
                    "High syscall frequency: {} syscalls in second {}",
                    events_in_second.len(),
                    second
                ),
                events_in_second[0].pid,
                format!("Example: {}", events_in_second[0].raw_line),
            ));
        }
    }

    alerts
}

pub fn detect_suspicious_sequence(events: &[SyscallEvent]) -> Vec<Alert> {
    let mut alerts = Vec::new();

    for (i, current) in events.iter().enumerate() {
        if is_sensitive_access(current) {
            for next in events.iter().skip(i + 1).take(SEQUENCE_WINDOW_SIZE) {
                if next.syscall == "execve" {
                    alerts.push(build_alert(
                        Severity::Critical,
                        format!(
                            "Suspicious sequence (window): access to {} followed by execution {}",
                            current.target, next.target
                        ),
                        next.pid,
                        format!(
                            "Sequence:\n{}\n...\n{}",
                            current.raw_line,
                            next.raw_line
                        ),
                    ));
                    break;
                }
            }
        }
    }

    alerts
}

fn build_alert(severity: Severity, message: String, pid: u32, raw_line: String) -> Alert {
    Alert {
        severity,
        message,
        pid,
        raw_line: raw_line.clone(),
        occurrences: 1,
        evidence: vec![raw_line],
        kind: None,
        location: None,
    }
}

fn extract_second(timestamp: &str) -> String {
    timestamp.split('.').next().unwrap_or(timestamp).to_string()
}

fn is_shell_execution(event: &SyscallEvent) -> bool {
    event.syscall == "execve" && (event.target == "/bin/sh" || event.target == "/bin/bash")
}

fn is_tmp_execution(event: &SyscallEvent) -> bool {
    event.syscall == "execve" && event.target.starts_with("/tmp/")
}

fn is_passwd_access(event: &SyscallEvent) -> bool {
    event.syscall == "openat" && event.target == "/etc/passwd"
}

fn is_shadow_access(event: &SyscallEvent) -> bool {
    event.syscall == "openat" && event.target == "/etc/shadow"
}

fn is_sensitive_access(event: &SyscallEvent) -> bool {
    is_passwd_access(event) || is_shadow_access(event)
}