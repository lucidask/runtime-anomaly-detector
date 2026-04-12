use crate::{severity_to_str, SummarySection};
use crate::model::Alert;

pub fn print_separator() {
    println!("============================================================");
}

pub fn print_subseparator() {
    println!("------------------------------------------------------------");
}

pub fn print_alert(index: usize, alert: &Alert) {
    let severity = severity_to_str(&alert.severity);

    print_separator();
    println!("ALERT #{}", index);
    print_separator();
    println!("Severity : {}", severity);
    println!("PID      : {}", alert.pid);
    println!("Message  : {}", alert.message);
    print_subseparator();
    println!("Raw line:");
    println!("{}", alert.raw_line);
}

pub fn print_alerts_section(title: &str, alerts: &[Alert]) {
    println!();
    print_separator();
    println!("{}", title);
    print_separator();

    if alerts.is_empty() {
        println!("No alerts detected.");
        return;
    }

    for (index, alert) in alerts.iter().enumerate() {
        print_alert(index + 1, alert);
    }
}

pub fn print_summary(lines: &[String], sections: &[SummarySection]) {
    println!();
    print_separator();
    println!("SUMMARY");
    print_separator();

    for line in lines {
        println!("{}", line);
    }

    for section in sections {
        println!();
        println!("{}:", section.title);
        print_subseparator();

        let total = if section.total == 0 {
            1.0
        } else {
            section.total as f64
        };

        if section.stats.is_empty() {
            println!("(none)");
        } else {
            for stat in &section.stats {
                let percent = (stat.count as f64 / total) * 100.0;
                println!("- {}: {} ({:.2}%)", stat.label, stat.count, percent);
            }
        }
    }
}

pub fn print_output_files(md_path: &str, json_path: &str) {
    println!();
    print_separator();
    println!("OUTPUT FILES");
    print_separator();
    println!("Markdown : {}", md_path);
    println!("JSON     : {}", json_path);
}

pub fn print_section_title(title: &str) {
    println!();
    print_separator();
    println!("{}", title);
    print_separator();
}

pub fn print_run_header(mode_label: &str, primary_input: &str, secondary_input: Option<&str>) {
    print_section_title("RUNTIME ANOMALY DETECTOR");

    println!("Mode           : {}", mode_label);
    println!("Primary input  : {}", primary_input);

    if let Some(input) = secondary_input {
        println!("Secondary input: {}", input);
    }

    print_subseparator();
    println!("Detection output");
    print_subseparator();
}