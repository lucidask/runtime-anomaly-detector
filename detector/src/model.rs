#[derive(Debug, Clone)]
pub struct SyscallEvent {
    pub pid: u32,
    pub timestamp: String,
    pub syscall: String,
    pub target: String,
    pub raw_line: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Severity {
    Warning,
    Critical,
}

#[derive(Debug, Clone)]
pub struct Alert {
    pub severity: Severity,
    pub message: String,
    pub pid: u32,
    pub raw_line: String,
    pub occurrences: u32,
    pub evidence: Vec<String>,
    pub kind: Option<MemoryErrorKind>,
    pub location: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MemoryErrorKind {
    InvalidRead,
    InvalidWrite,
    InvalidFree,
    UninitialisedValue,
    UseAfterFree,
    HeapOverflow,
    StackOverflow,
    SegmentationFault,
}

#[derive(Debug, Clone)]
pub struct MemoryEvent {
    pub kind: MemoryErrorKind,
    pub raw_line: String,
    pub location: Option<String>,
}