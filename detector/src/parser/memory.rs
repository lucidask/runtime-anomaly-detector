use crate::model::{MemoryErrorKind, MemoryEvent};

pub fn parse_memory_log(content: &str) -> Vec<MemoryEvent> {
    let mut events = Vec::new();
    let lines: Vec<&str> = content.lines().collect();

    for i in 0..lines.len() {
        let line = lines[i];

        if line.contains("Invalid read") {
            let kind = if line_after_contains(&lines, i, "free'd") {
                MemoryErrorKind::UseAfterFree
            } else if line_after_contains(&lines, i, "block of size")
                && line_after_contains(&lines, i, "alloc'd")
            {
                MemoryErrorKind::HeapOverflow
            } else {
                MemoryErrorKind::InvalidRead
            };

            let location = extract_location(&lines, i);
            events.push(MemoryEvent {
                pid: extract_pid(line).unwrap_or(0),
                kind,
                raw_line: line.to_string(),
                location,
            });
        } else if line.contains("Invalid write") {
            let kind = if line_after_contains(&lines, i, "Address 0x0")
                || line_after_contains(&lines, i, "SIGSEGV")
                || line_after_contains(&lines, i, "Access not within mapped region")
            {
                MemoryErrorKind::SegmentationFault
            } else if line_after_contains(&lines, i, "block of size")
                && line_after_contains(&lines, i, "alloc'd")
            {
                MemoryErrorKind::HeapOverflow
            } else {
                MemoryErrorKind::InvalidWrite
            };

            let location = extract_location(&lines, i);
            events.push(MemoryEvent {
                pid: extract_pid(line).unwrap_or(0),
                kind,
                raw_line: line.to_string(),
                location,
            });
        } else if line.contains("Invalid free") {

            let location = extract_location(&lines, i);
            events.push(MemoryEvent {
                pid: extract_pid(line).unwrap_or(0),
                kind: MemoryErrorKind::InvalidFree,
                raw_line: line.to_string(),
                location,
            });
        } else if line.contains("Conditional jump or move depends on uninitialised value")
            || line.contains("Use of uninitialised value")
        {
            let location = extract_location(&lines, i);
            events.push(MemoryEvent {
                pid: extract_pid(line).unwrap_or(0),
                kind: MemoryErrorKind::UninitialisedValue,
                raw_line: line.to_string(),
                location,
            });
        } else if line.contains("stack smashing detected")
            || line.contains("__stack_chk_fail")
            || (line.contains("SIGABRT") && line_after_contains(&lines, i, "__stack_chk_fail"))
        {
            let location = extract_location(&lines, i);
            events.push(MemoryEvent {
                pid: extract_pid(line).unwrap_or(0),
                kind: MemoryErrorKind::StackOverflow,
                raw_line: line.to_string(),
                location,
            });
        }
    }

    events
}

fn line_after_contains(lines: &[&str], index: usize, needle: &str) -> bool {
    let end = usize::min(index + 12, lines.len());
    lines[index..end].iter().any(|l| l.contains(needle))
}

fn extract_location(lines: &[&str], index: usize) -> Option<String> {
    let end = usize::min(index + 12, lines.len());
    let mut fallback: Option<String> = None;

    for line in &lines[index..end] {
        if let Some(start) = line.find('(') {
            if let Some(end_paren) = line[start + 1..].find(')') {
                let location = &line[start + 1..start + 1 + end_paren];

                if let Some(colon_pos) = location.rfind(':') {
                    let file = &location[..colon_pos];
                    let line_number = &location[colon_pos + 1..];

                    if line_number.chars().all(|c| c.is_ascii_digit()) {
                        let formatted = format!("{} ligne: {}", file, line_number);

                        if is_user_code_file(file) {
                            return Some(formatted);
                        }

                        if fallback.is_none() {
                            fallback = Some(formatted);
                        }

                        continue;
                    }
                }

                if let Some(path) = location.strip_prefix("in ") {
                    let formatted = path.trim().to_string();

                    if is_user_code_file(&formatted) {
                        return Some(formatted);
                    }

                    if fallback.is_none() {
                        fallback = Some(formatted);
                    }
                }
            }
        }
    }

    fallback
}

fn is_user_code_file(file: &str) -> bool {
    let lower = file.to_ascii_lowercase();

    let system_markers = [
        "/usr/",
        "/lib/",
        "/lib64/",
        "/build/",
        "glibc",
        "pthread_kill.c",
        "pthread_kill_internal",
        "stack_chk_fail.c",
        "fortify_fail.c",
        "abort.c",
        "raise.c",
        "libc_fatal.c",
        "vgpreload",
        "valgrind",
        "ioputs.c",
        "genops.c",
        "fileops.c",
        "strlen",
        "malloc.c",
    ];

    !system_markers.iter().any(|marker| lower.contains(marker))
}

fn extract_pid(line: &str) -> Option<u32> {
    if let Some(rest) = line.strip_prefix("==") {
        if let Some(end) = rest.find("==") {
            return rest[..end].trim().parse::<u32>().ok();
        }
    }
    None
}