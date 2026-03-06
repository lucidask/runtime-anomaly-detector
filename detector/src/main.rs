mod model;
mod parser;
mod rules;

use std::env;
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, Write};

use model::{Alert, Severity};

fn severity_to_str(severity: &Severity) -> &'static str {
    match severity {
        Severity::Info => "INFO",
        Severity::Warning => "WARNING",
        Severity::Critical => "CRITICAL",
    }
}

fn print_alert(alert: &Alert) {
    let severity = severity_to_str(&alert.severity);
    println!("[{}] PID {} - {}", severity, alert.pid, alert.message);
    println!("  -> {}", alert.raw_line);
}

fn write_markdown_report(
    output_path: &str,
    input_path: &str,
    parsed_events: usize,
    alerts: &[Alert],
) -> io::Result<()> {
    if let Some(parent) = std::path::Path::new(output_path).parent() {
        fs::create_dir_all(parent)?;
    }

    let mut file = File::create(output_path)?;

    writeln!(file, "# Runtime Anomaly Detector Report")?;
    writeln!(file)?;
    writeln!(file, "## Input trace")?;
    writeln!(file, "`{}`", input_path)?;
    writeln!(file)?;
    writeln!(file, "## Summary")?;
    writeln!(file, "- Parsed events: {}", parsed_events)?;
    writeln!(file, "- Alerts: {}", alerts.len())?;
    writeln!(file)?;

    if alerts.is_empty() {
        writeln!(file, "## Alerts")?;
        writeln!(file, "No alerts detected.")?;
        return Ok(());
    }

    writeln!(file, "## Alerts")?;
    writeln!(file)?;

    for (index, alert) in alerts.iter().enumerate() {
        let severity = severity_to_str(&alert.severity);

        writeln!(file, "### Alert {}", index + 1)?;
        writeln!(file, "- Severity: {}", severity)?;
        writeln!(file, "- PID: {}", alert.pid)?;
        writeln!(file, "- Message: {}", alert.message)?;
        writeln!(file, "- Raw line:")?;
        writeln!(file)?;
        writeln!(file, "```text")?;
        writeln!(file, "{}", alert.raw_line)?;
        writeln!(file, "```")?;
        writeln!(file)?;
    }

    Ok(())
}

fn run(input_path: &str, output_path: &str) -> io::Result<()> {
    let file = File::open(input_path)?;
    let reader = BufReader::new(file);

    let mut total_events = 0usize;
    let mut all_alerts: Vec<Alert> = Vec::new();

    for line_result in reader.lines() {
        let line = line_result?;

        if let Some(event) = parser::parse_line(&line) {
            total_events += 1;

            let alerts = rules::apply_rules(&event);

            for alert in alerts {
                print_alert(&alert);
                all_alerts.push(alert);
            }
        }
    }

    println!();
    println!("Summary:");
    println!("  Parsed events: {}", total_events);
    println!("  Alerts: {}", all_alerts.len());

    write_markdown_report(output_path, input_path, total_events, &all_alerts)?;

    println!();
    println!("Report written to: {}", output_path);

    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 || args[1] != "--input" {
        eprintln!("Usage: cargo run -- --input <trace_file>");
        std::process::exit(1);
    }

    let input_path = &args[2];
    use std::path::Path;

    let input_file = Path::new(input_path)
    .file_stem()
    .unwrap()
    .to_string_lossy();

    let output_path = format!("../experiments/output/report_{}.md", input_file);

    if let Err(err) = run(input_path, output_path) {
        eprintln!("Error: {}", err);
        std::process::exit(1);
    }
}