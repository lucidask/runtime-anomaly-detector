use crate::model::{Alert, MemoryErrorKind, MemoryEvent, Severity};

pub fn apply_memory_rules(events: &[MemoryEvent]) -> Vec<Alert> {
    let mut alerts = Vec::new();

    for event in events {
        let severity = match event.kind {
            MemoryErrorKind::InvalidRead => Severity::Warning,
            MemoryErrorKind::InvalidWrite => Severity::Critical,
            MemoryErrorKind::InvalidFree => Severity::Warning,
            MemoryErrorKind::UninitialisedValue => Severity::Warning,
            MemoryErrorKind::UseAfterFree => Severity::Critical,
            MemoryErrorKind::HeapOverflow => Severity::Critical,
            MemoryErrorKind::StackOverflow => Severity::Critical,
            MemoryErrorKind::SegmentationFault => Severity::Critical,
        };

        let message = build_memory_message(event);

      alerts.push(Alert {
        severity,
        message,
        pid: event.pid,
        raw_line: event.raw_line.clone(),
        occurrences: 1,
        evidence: vec![event.raw_line.clone()],

        kind: Some(event.kind.clone()),
        location: event.location.clone(),
    });
    }

    merge_duplicate_memory_alerts(alerts)
}

fn build_memory_message(event: &MemoryEvent) -> String {
    let loc = event.location.as_deref().unwrap_or("unknown location");

    match event.kind {
        MemoryErrorKind::InvalidRead => format!(
            "Invalid read detected at {}. The program accessed memory outside a valid region. Likely cause: out-of-bounds access or invalid pointer.",
            loc
        ),
        MemoryErrorKind::InvalidWrite => format!(
            "Invalid write detected at {}. The program wrote outside the bounds of a valid memory region. Likely cause: buffer overflow or incorrect size/index handling.",
            loc
        ),
        MemoryErrorKind::InvalidFree => format!(
            "Invalid free detected at {}. The program attempted to free invalid or already released memory. Likely cause: double free or freeing non-heap memory.",
            loc
        ),
        MemoryErrorKind::UninitialisedValue => format!(
            "Use of uninitialised value detected at {}. The program used data before initialisation. Likely cause: missing assignment before use.",
            loc
        ),
        MemoryErrorKind::UseAfterFree => format!(
            "Use-after-free detected at {}. The program accessed memory after it had already been freed. Likely cause: dangling pointer reuse.",
            loc
        ),
        MemoryErrorKind::HeapOverflow => format!(
            "Heap overflow detected at {}. The program accessed memory beyond the limits of a heap-allocated block. Likely cause: buffer overflow on dynamically allocated memory.",
            loc
        ),
        MemoryErrorKind::StackOverflow => {
            if event.raw_line.contains("__stack_chk_fail") || event.raw_line.contains("SIGABRT") {
                format!(
                    "Stack corruption detected at {} (triggered by stack protection / __stack_chk_fail). Likely cause: an earlier unsafe stack write, possibly a buffer overflow.",
                    loc
                )
            } else {
                format!(
                    "Stack corruption detected at {}. Likely cause: unsafe stack memory operation or buffer overflow.",
                    loc
                )
            }
        }
        MemoryErrorKind::SegmentationFault => format!(
            "Segmentation fault detected at {}. The program accessed a memory region that is not mapped or not allowed. Likely cause: null pointer dereference or invalid pointer access.",
            loc
        ),
    }
}

fn merge_duplicate_memory_alerts(alerts: Vec<Alert>) -> Vec<Alert> {
    let mut merged: Vec<Alert> = Vec::new();

    for alert in alerts {
        if let Some(existing) = merged.iter_mut().find(|a| {
            a.severity == alert.severity
                && a.pid == alert.pid
                && a.kind == alert.kind
                && same_or_near_location(a.location.as_deref(), alert.location.as_deref())
            }) {
            existing.occurrences += alert.occurrences;

            for ev in alert.evidence {
                if !existing.evidence.contains(&ev) {
                    existing.evidence.push(ev);
                }
            }
        } else {
            merged.push(alert);
        }
    }

    merged
}

fn same_or_near_location(a: Option<&str>, b: Option<&str>) -> bool {
    match (parse_location(a), parse_location(b)) {
        (Some((file_a, line_a)), Some((file_b, line_b))) => {
            file_a == file_b && line_a.abs_diff(line_b) <= 1
        }
        (None, None) => true,
        _ => false,
    }
}

fn parse_location(location: Option<&str>) -> Option<(String, u32)> {
    let loc = location?;

    let marker = " ligne: ";
    let pos = loc.rfind(marker)?;

    let file = loc[..pos].trim().to_string();
    let line_str = loc[pos + marker.len()..].trim();

    let line = line_str.parse::<u32>().ok()?;
    Some((file, line))
}