use std::fmt::Write as FmtWrite;
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;

use crate::model::Alert;
use crate::{severity_to_str, SummarySection};

fn json_escape(value: &str) -> String {
    let mut escaped = String::new();

    for ch in value.chars() {
        match ch {
            '\\' => escaped.push_str("\\\\"),
            '"' => escaped.push_str("\\\""),
            '\n' => escaped.push_str("\\n"),
            '\r' => escaped.push_str("\\r"),
            '\t' => escaped.push_str("\\t"),
            c if c.is_control() => {
                write!(&mut escaped, "\\u{:04x}", c as u32).unwrap();
            }
            c => escaped.push(c),
        }
    }

    escaped
}

fn split_summary_line(line: &str) -> (String, String) {
    let mut parts = line.splitn(2, ':');

    let label = parts.next().unwrap_or("").trim().to_string();
    let value = parts.next().unwrap_or("").trim().to_string();

    (label, value)
}

fn write_json_summary(json: &mut String, lines: &[String]) {
    writeln!(json, "  \"summary\": [").unwrap();

    for (index, line) in lines.iter().enumerate() {
        let (label, value) = split_summary_line(line);

        if index + 1 == lines.len() {
            writeln!(
                json,
                "    {{ \"label\": \"{}\", \"value\": \"{}\" }}",
                json_escape(&label),
                json_escape(&value)
            )
            .unwrap();
        } else {
            writeln!(
                json,
                "    {{ \"label\": \"{}\", \"value\": \"{}\" }},",
                json_escape(&label),
                json_escape(&value)
            )
            .unwrap();
        }
    }

    writeln!(json, "  ],").unwrap();
}

fn write_json_sections(json: &mut String, sections: &[SummarySection]) {
    writeln!(json, "  \"sections\": [").unwrap();

    for (section_index, section) in sections.iter().enumerate() {
        writeln!(json, "    {{").unwrap();
        writeln!(json, "      \"title\": \"{}\",", json_escape(&section.title)).unwrap();
        writeln!(json, "      \"total\": {},", section.total).unwrap();
        writeln!(json, "      \"stats\": [").unwrap();

        for (stat_index, stat) in section.stats.iter().enumerate() {
            let total = if section.total == 0 {
                1.0
            } else {
                section.total as f64
            };

            let percent = (stat.count as f64 / total) * 100.0;

            if stat_index + 1 == section.stats.len() {
                writeln!(
                    json,
                    "        {{ \"label\": \"{}\", \"count\": {}, \"percent\": {:.2} }}",
                    json_escape(&stat.label),
                    stat.count,
                    percent
                )
                .unwrap();
            } else {
                writeln!(
                    json,
                    "        {{ \"label\": \"{}\", \"count\": {}, \"percent\": {:.2} }},",
                    json_escape(&stat.label),
                    stat.count,
                    percent
                )
                .unwrap();
            }
        }

        writeln!(json, "      ]").unwrap();

        if section_index + 1 == sections.len() {
            writeln!(json, "    }}").unwrap();
        } else {
            writeln!(json, "    }},").unwrap();
        }
    }

    writeln!(json, "  ],").unwrap();
}

fn write_json_alerts_array(
    json: &mut String,
    key: &str,
    alerts: &[Alert],
    trailing_comma: bool,
) {
    writeln!(json, "  \"{}\": [", json_escape(key)).unwrap();

    for (index, alert) in alerts.iter().enumerate() {
        let severity = severity_to_str(&alert.severity);

        writeln!(json, "    {{").unwrap();
        writeln!(json, "      \"severity\": \"{}\",", json_escape(severity)).unwrap();
        writeln!(json, "      \"pid\": {},", alert.pid).unwrap();
        writeln!(json, "      \"message\": \"{}\",", json_escape(&alert.message)).unwrap();
        writeln!(json, "      \"raw_line\": \"{}\"", json_escape(&alert.raw_line)).unwrap();

        if index + 1 == alerts.len() {
            writeln!(json, "    }}").unwrap();
        } else {
            writeln!(json, "    }},").unwrap();
        }
    }

    if trailing_comma {
        writeln!(json, "  ],").unwrap();
    } else {
        writeln!(json, "  ]").unwrap();
    }
}

pub fn write_json_report(
    output_path: &str,
    report_type: &str,
    input_path: &str,
    summary_lines: &[String],
    sections: &[SummarySection],
    alerts_key: &str,
    alerts: &[Alert],
) -> io::Result<()> {
    if let Some(parent) = Path::new(output_path).parent() {
        fs::create_dir_all(parent)?;
    }

    let mut file = File::create(output_path)?;
    let mut json = String::new();

    writeln!(json, "{{").unwrap();
    writeln!(json, "  \"report_type\": \"{}\",", json_escape(report_type)).unwrap();
    writeln!(json, "  \"input_trace\": \"{}\",", json_escape(input_path)).unwrap();

    write_json_summary(&mut json, summary_lines);
    write_json_sections(&mut json, sections);
    write_json_alerts_array(&mut json, alerts_key, alerts, false);

    writeln!(json, "}}").unwrap();

    file.write_all(json.as_bytes())?;
    Ok(())
}

pub fn write_combined_json_report(
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
    let mut json = String::new();

    writeln!(json, "{{").unwrap();
    writeln!(json, "  \"report_type\": \"combined\",").unwrap();
    writeln!(json, "  \"inputs\": {{").unwrap();
    writeln!(
        json,
        "    \"syscall_trace\": \"{}\",",
        json_escape(syscall_path)
    )
    .unwrap();
    writeln!(json, "    \"memory_log\": \"{}\"", json_escape(memory_path)).unwrap();
    writeln!(json, "  }},").unwrap();

    write_json_summary(&mut json, summary_lines);
    write_json_sections(&mut json, sections);
    write_json_alerts_array(&mut json, "syscall_alerts", syscall_alerts, true);
    write_json_alerts_array(&mut json, "memory_alerts", memory_alerts, false);

    writeln!(json, "}}").unwrap();

    file.write_all(json.as_bytes())?;
    Ok(())
}