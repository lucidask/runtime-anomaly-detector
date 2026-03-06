#[derive(Debug, Clone)]
pub struct SyscallEvent {
    pub pid: u32,
    pub syscall: String,
    pub target: String,
    pub raw_line: String,
}

#[derive(Debug, Clone)]
pub enum Severity {
    Info,
    Warning,
    Critical,
}

#[derive(Debug, Clone)]
pub struct Alert {
    pub severity: Severity,
    pub message: String,
    pub pid: u32,
    pub raw_line: String,
}