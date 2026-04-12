mod model;
mod parser;
mod rules;
mod utils;

use std::collections::HashMap;
use std::env;
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader};
use std::path::Path;

use model::{Alert, Severity};

use crate::parser::memory::parse_memory_log;
use crate::parser::syscall::parse_syscall_line;

use crate::rules::memory::apply_memory_rules;
use crate::rules::syscall::apply_syscall_rules;
use crate::rules::syscall::detect_high_frequency;
use crate::rules::syscall::detect_suspicious_sequence;

use utils::json::{write_combined_json_report, write_json_report};
use utils::markdown::{write_combined_markdown_report, write_markdown_report};
use utils::display::*;

pub struct SummaryStat {
    pub label: String,
    pub count: u32,
}

pub struct SummarySection {
    pub title: String,
    pub total: usize,
    pub stats: Vec<SummaryStat>,
}

pub fn severity_to_str(severity: &Severity) -> &'static str {
    match severity {
        Severity::Warning => "WARNING",
        Severity::Critical => "CRITICAL",
    }
}

fn build_syscall_summary_section(
    sorted_syscalls: &[(String, u32)],
    total_syscall_events: usize,
) -> SummarySection {
    SummarySection {
        title: "Parsed syscalls".to_string(),
        total: total_syscall_events,
        stats: sorted_syscalls
            .iter()
            .map(|(name, count)| SummaryStat {
                label: name.clone(),
                count: *count,
            })
            .collect(),
    }
}

fn memory_kind_label_from_message(message: &str) -> String {
    if message.starts_with("Invalid read detected") {
        "Invalid read".to_string()
    } else if message.starts_with("Invalid write detected") {
        "Invalid write".to_string()
    } else if message.starts_with("Invalid free detected") {
        "Invalid free".to_string()
    } else if message.starts_with("Use of uninitialised value detected") {
        "Uninitialised value".to_string()
    } else if message.starts_with("Use-after-free detected") {
        "Use-after-free".to_string()
    } else if message.starts_with("Heap overflow detected") {
        "Heap overflow".to_string()
    } else if message.starts_with("Stack corruption detected") {
        "Stack corruption".to_string()
    } else if message.starts_with("Segmentation fault detected") {
        "Segmentation fault".to_string()
    } else {
        "Other memory anomaly".to_string()
    }
}

fn build_memory_summary_section(alerts: &[Alert], total_memory_events: usize) -> SummarySection {
    let mut memory_counts: HashMap<String, u32> = HashMap::new();

    for alert in alerts {
        let kind_label = memory_kind_label_from_message(&alert.message);
        *memory_counts.entry(kind_label).or_insert(0) += 1;
    }

    let mut sorted_memory: Vec<(String, u32)> =
        memory_counts.iter().map(|(k, v)| (k.clone(), *v)).collect();

    sorted_memory.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));

    SummarySection {
        title: "Parsed memory".to_string(),
        total: total_memory_events,
        stats: sorted_memory
            .iter()
            .map(|(name, count)| SummaryStat {
                label: name.clone(),
                count: *count,
            })
            .collect(),
    }
}

fn run_syscall(input_path: &str, output_md_path: &str, output_json_path: &str) -> io::Result<()> {
    print_run_header("SYSCALL", input_path, None);

    let file = File::open(input_path)?;
    let reader = BufReader::new(file);

    let mut total_events = 0usize;
    let mut syscall_alerts: Vec<Alert> = Vec::new();
    let mut all_events = Vec::new();
    let mut syscall_counts: HashMap<String, u32> = HashMap::new();

    for line_result in reader.lines() {
        let line = line_result?;

        if let Some(event) = parse_syscall_line(&line) {
            total_events += 1;
            all_events.push(event.clone());
            *syscall_counts.entry(event.syscall.clone()).or_insert(0) += 1;

            let alerts = apply_syscall_rules(&event);
            for alert in alerts {
                syscall_alerts.push(alert);
            }
        }
    }

    let freq_alerts = detect_high_frequency(&all_events);
    for alert in freq_alerts {
        syscall_alerts.push(alert);
    }

    let seq_alerts = detect_suspicious_sequence(&all_events);
    for alert in seq_alerts {
        syscall_alerts.push(alert);
    }

    let mut sorted_syscalls: Vec<(String, u32)> =
        syscall_counts.iter().map(|(k, v)| (k.clone(), *v)).collect();

    sorted_syscalls.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));

     let summary_lines = vec![
        format!("Parsed syscall events : {}", total_events),
        format!("Total alerts          : {}", syscall_alerts.len()),
        format!("Syscall alerts        : {}", syscall_alerts.len()),
    ];

    let sections = vec![build_syscall_summary_section(&sorted_syscalls, total_events)];

    write_markdown_report(
        output_md_path,
        input_path,
        &summary_lines,
        &sections,
        "SYSCALL ALERTS",
        &syscall_alerts,
    )?;
    write_json_report(
        output_json_path,
        "syscall",
        input_path,
        &summary_lines,
        &sections,
        "syscall_alerts",
        &syscall_alerts,
    )?;

    print_summary(&summary_lines, &sections);
    print_alerts_section("SYSCALL ALERTS", &syscall_alerts);
    print_output_files(output_md_path, output_json_path);

    Ok(())
}

fn run_memory(input_path: &str, output_md_path: &str, output_json_path: &str) -> io::Result<()> {
    print_run_header("MEMORY", input_path, None);

    let content = fs::read_to_string(input_path)?;
    let events = parse_memory_log(&content);
    let memory_alerts = apply_memory_rules(&events);

    let summary_lines = vec![
        format!("Parsed memory events : {}", events.len()),
        format!("Total alerts         : {}", memory_alerts.len()),
        format!("Memory alerts        : {}", memory_alerts.len()),
    ];

    let sections = vec![build_memory_summary_section(&memory_alerts, events.len())];

    write_markdown_report(
        output_md_path,
        input_path,
        &summary_lines,
        &sections,
        "MEMORY ALERTS",
        &memory_alerts,
    )?;

   write_json_report(
        output_json_path,
        "memory",
        input_path,
        &summary_lines,
        &sections,
        "memory_alerts",
        &memory_alerts,
    )?;

    print_summary(&summary_lines, &sections);
    print_alerts_section("MEMORY ALERTS", &memory_alerts);
    print_output_files(output_md_path, output_json_path);

    Ok(())
}

fn run_combined(
    syscall_path: &str,
    memory_path: &str,
    output_md_path: &str,
    output_json_path: &str,
) -> io::Result<()> {
    print_run_header("COMBINED", syscall_path, Some(memory_path));

    let file = File::open(syscall_path)?;
    let reader = BufReader::new(file);

    let mut syscall_alerts: Vec<Alert> = Vec::new();
    let memory_alerts: Vec<Alert>;
    let mut all_events = Vec::new();
    let mut syscall_counts: HashMap<String, u32> = HashMap::new();
    let mut total_events = 0usize;

    for line_result in reader.lines() {
        let line = line_result?;

        if let Some(event) = parse_syscall_line(&line) {
            total_events += 1;
            all_events.push(event.clone());
            *syscall_counts.entry(event.syscall.clone()).or_insert(0) += 1;

            let alerts = apply_syscall_rules(&event);
            for alert in alerts {
                syscall_alerts.push(alert);
            }
        }
    }

    let freq_alerts = detect_high_frequency(&all_events);
    for alert in freq_alerts {
        syscall_alerts.push(alert);
    }

    let seq_alerts = detect_suspicious_sequence(&all_events);
    for alert in seq_alerts {
        syscall_alerts.push(alert);
    }

    let content = fs::read_to_string(memory_path)?;
    let mem_events = parse_memory_log(&content);
    memory_alerts = apply_memory_rules(&mem_events);

    let mut all_alerts: Vec<Alert> = Vec::new();
    all_alerts.extend(syscall_alerts.clone());
    all_alerts.extend(memory_alerts.clone());

    let mut sorted_syscalls: Vec<(String, u32)> =
        syscall_counts.iter().map(|(k, v)| (k.clone(), *v)).collect();

    sorted_syscalls.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));

    let summary_lines = vec![
        format!("Syscall events        : {}", total_events),
        format!("Memory events         : {}", mem_events.len()),
        format!("Total alerts          : {}", all_alerts.len()),
        format!("Syscall alerts        : {}", syscall_alerts.len()),
        format!("Memory alerts         : {}", memory_alerts.len()),
    ];

    let sections = vec![
        build_syscall_summary_section(&sorted_syscalls, total_events),
        build_memory_summary_section(&memory_alerts, mem_events.len()),
    ];
    
    write_combined_markdown_report(
        output_md_path,
        syscall_path,
        memory_path,
        &summary_lines,
        &sections,
        &syscall_alerts,
        &memory_alerts,
    )?;

    write_combined_json_report(
        output_json_path,
        syscall_path,
        memory_path,
        &summary_lines,
        &sections,
        &syscall_alerts,
        &memory_alerts,
)?;

    print_summary(&summary_lines, &sections);
    print_alerts_section("SYSCALL ALERTS", &syscall_alerts);
    print_alerts_section("MEMORY ALERTS", &memory_alerts);
    print_output_files(output_md_path, output_json_path);

    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("Usage:");
        eprintln!("  cargo run -- --syscall-input <syscall_trace_file>");
        eprintln!("  cargo run -- --memory-input <memory_log_file>");
        eprintln!("  cargo run -- --combined <syscall_log> <memory_log>");
        std::process::exit(1);
    }

    let mode = &args[1];
    let input_path = &args[2];

    let base_dir = match mode.as_str() {
        "--syscall-input" => "syscall",
        "--memory-input" => "memory",
        "--combined" => "combined",
        _ => {
            eprintln!("Unknown option: {}", mode);
            eprintln!("Use --syscall-input, --memory-input or --combined");
            std::process::exit(1);
        }
    };

    let output_name = if mode == "--combined" {
        if args.len() < 4 {
            eprintln!("Usage: --combined <syscall_log> <memory_log>");
            std::process::exit(1);
        }

        let memory_path = &args[3];

        let syscall_name = Path::new(input_path)
            .file_stem()
            .unwrap()
            .to_string_lossy();

        let memory_name = Path::new(memory_path)
            .file_stem()
            .unwrap()
            .to_string_lossy();

        format!("{}__{}", syscall_name, memory_name)
    } else {
        Path::new(input_path)
            .file_stem()
            .unwrap()
            .to_string_lossy()
            .to_string()
    };

    let output_dir = format!("../experiments/output/{}/{}", base_dir, output_name);
    let output_md_path = format!("{}/{}.md", output_dir, output_name);
    let output_json_path = format!("{}/{}.json", output_dir, output_name);

    let result = if mode == "--syscall-input" {
        run_syscall(input_path, &output_md_path, &output_json_path)
    } else if mode == "--memory-input" {
        run_memory(input_path, &output_md_path, &output_json_path)
    } else if mode == "--combined" {
        let memory_path = &args[3];
        run_combined(input_path, memory_path, &output_md_path, &output_json_path)
    } else {
        eprintln!("Unknown option: {}", mode);
        eprintln!("Use --syscall-input, --memory-input or --combined");
        std::process::exit(1);
    };

    if let Err(err) = result {
        eprintln!("Error: {}", err);
        std::process::exit(1);
    }
}