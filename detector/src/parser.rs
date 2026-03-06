use crate::model::SyscallEvent;

pub fn parse_line(line: &str) -> Option<SyscallEvent> {
    if line.contains("execve(") {
        return parse_execve_line(line);
    }

    if line.contains("openat(") {
        return parse_openat_line(line);
    }

    None
}

fn parse_execve_line(line: &str) -> Option<SyscallEvent> {
    let mut parts = line.split_whitespace();

    let pid_str = parts.next()?;
    let pid: u32 = pid_str.parse().ok()?;

    let execve_pos = line.find("execve(")?;
    let after_execve = &line[execve_pos + "execve(".len()..];

    let first_quote = after_execve.find('"')?;
    let after_first_quote = &after_execve[first_quote + 1..];
    let second_quote = after_first_quote.find('"')?;
    let target = &after_first_quote[..second_quote];

    Some(SyscallEvent {
        pid,
        syscall: "execve".to_string(),
        target: target.to_string(),
        raw_line: line.to_string(),
    })
}

fn parse_openat_line(line: &str) -> Option<SyscallEvent> {
    let mut parts = line.split_whitespace();

    let pid_str = parts.next()?;
    let pid: u32 = pid_str.parse().ok()?;

    let openat_pos = line.find("openat(")?;
    let after_openat = &line[openat_pos + "openat(".len()..];

    let first_quote = after_openat.find('"')?;
    let after_first_quote = &after_openat[first_quote + 1..];
    let second_quote = after_first_quote.find('"')?;
    let target = &after_first_quote[..second_quote];

    Some(SyscallEvent {
        pid,
        syscall: "openat".to_string(),
        target: target.to_string(),
        raw_line: line.to_string(),
    })
}