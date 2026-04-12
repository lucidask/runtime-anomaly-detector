use crate::model::SyscallEvent;

pub fn parse_syscall_line(line: &str) -> Option<SyscallEvent> {
    if line.contains("execve(") {
        return parse_execve_line(line);
    }

    if line.contains("openat(") {
        return parse_openat_line(line);
    }

    if line.contains("read(") {
        return parse_generic_syscall(line, "read");
    }

    if line.contains("write(") {
        return parse_generic_syscall(line, "write");
    }

    if line.contains("close(") {
        return parse_generic_syscall(line, "close");
    }

    None
}

fn parse_execve_line(line: &str) -> Option<SyscallEvent> {
    let (pid, timestamp) = parse_pid_and_timestamp(line)?;
    let target = extract_first_quoted_arg_after(line, "execve(")?;

    Some(build_event(pid, timestamp, "execve", target, line))
}

fn parse_openat_line(line: &str) -> Option<SyscallEvent> {
    let (pid, timestamp) = parse_pid_and_timestamp(line)?;
    let target = extract_first_quoted_arg_after(line, "openat(")?;

    Some(build_event(pid, timestamp, "openat", target, line))
}

fn parse_generic_syscall(line: &str, name: &str) -> Option<SyscallEvent> {
    let (pid, timestamp) = parse_pid_and_timestamp(line)?;

    Some(build_event(pid, timestamp, name, String::new(), line))
}

fn parse_pid_and_timestamp(line: &str) -> Option<(u32, String)> {
    let mut parts = line.split_whitespace();

    let pid_str = parts.next()?;
    let timestamp = parts.next()?.to_string();
    let pid = pid_str.parse().ok()?;

    Some((pid, timestamp))
}

fn extract_first_quoted_arg_after(line: &str, marker: &str) -> Option<String> {
    let start = line.find(marker)?;
    let after_marker = &line[start + marker.len()..];

    let first_quote = after_marker.find('"')?;
    let after_first_quote = &after_marker[first_quote + 1..];
    let second_quote = after_first_quote.find('"')?;

    Some(after_first_quote[..second_quote].to_string())
}

fn build_event(
    pid: u32,
    timestamp: String,
    syscall: &str,
    target: String,
    raw_line: &str,
) -> SyscallEvent {
    SyscallEvent {
        pid,
        timestamp,
        syscall: syscall.to_string(),
        target,
        raw_line: raw_line.to_string(),
    }
}