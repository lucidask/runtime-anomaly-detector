use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;

use crate::model::Alert;
use crate::{severity_to_str, SummarySection};

pub fn write_markdown_summary(
    file: &mut File,
    lines: &[String],
    sections: &[SummarySection],
) -> io::Result<()> {
    writeln!(file, "## Summary")?;
    writeln!(file)?;

    for line in lines {
        writeln!(file, "- {}", line)?;
    }

    for section in sections {
        writeln!(file)?;
        writeln!(file, "### {}", section.title)?;
        writeln!(file)?;

        let total = if section.total == 0 {
            1.0
        } else {
            section.total as f64
        };

        if section.stats.is_empty() {
            writeln!(file, "_(none)_")?;
        } else {
            for stat in &section.stats {
                let percent = (stat.count as f64 / total) * 100.0;
                writeln!(file, "- {}: {} ({:.2}%)", stat.label, stat.count, percent)?;
            }
        }
    }

    Ok(())
}

pub fn write_markdown_alerts_section(
    file: &mut File,
    title: &str,
    alerts: &[Alert],
) -> io::Result<()> {
    writeln!(file)?;
    writeln!(file, "## {}", title)?;
    writeln!(file)?;

    if alerts.is_empty() {
        writeln!(file, "No alerts detected.")?;
        return Ok(());
    }

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

pub fn write_markdown_report(
    output_path: &str,
    input_path: &str,
    summary_lines: &[String],
    sections: &[SummarySection],
    alerts_title: &str,
    alerts: &[Alert],
) -> io::Result<()> {
    if let Some(parent) = Path::new(output_path).parent() {
        fs::create_dir_all(parent)?;
    }

    let mut file = File::create(output_path)?;

    writeln!(file, "# Runtime Anomaly Detector Report")?;
    writeln!(file)?;
    writeln!(file, "## Input trace")?;
    writeln!(file, "`{}`", input_path)?;
    writeln!(file)?;

    write_markdown_summary(&mut file, summary_lines, sections)?;
    write_markdown_alerts_section(&mut file, alerts_title, alerts)?;

    Ok(())
}

pub fn write_combined_markdown_report(
    output_path: &str,
    syscall_path: &str,
    memory_path: &str,
    summary_lines: &[String],
    sections: &[SummarySection],
    syscall_alerts: &[Alert],
    memory_alerts: &[Alert],
) -> io::Result<()> {
    if let Some(parent) = Path::new(output_path).parent() {
        fs::create_dir_all(parent)?;
    }

    let mut file = File::create(output_path)?;

    writeln!(file, "# Runtime Anomaly Detector Report (Combined)")?;
    writeln!(file)?;
    writeln!(file, "## Input traces")?;
    writeln!(file, "- Syscall trace: `{}`", syscall_path)?;
    writeln!(file, "- Memory log: `{}`", memory_path)?;
    writeln!(file)?;

    write_markdown_summary(&mut file, summary_lines, sections)?;
    write_markdown_alerts_section(&mut file, "SYSCALL ALERTS", syscall_alerts)?;
    write_markdown_alerts_section(&mut file, "MEMORY ALERTS", memory_alerts)?;

    Ok(())
}