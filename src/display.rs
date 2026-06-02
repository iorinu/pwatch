use crate::i18n::tr;
use crate::port::PortInfo;
use colored::Colorize;
use comfy_table::{ContentArrangement, Table, presets::UTF8_FULL};

pub fn print_port_list(ports: &[PortInfo]) {
    if ports.is_empty() {
        println!(
            "{}",
            tr!(
                "No listening ports found",
                "リスニング中のポートが見つかりません"
            )
            .yellow()
        );
        return;
    }

    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_header(vec![
            format!("{}", "PORT".cyan().bold()),
            format!("{}", "PROTO".green()),
            format!("{}", "PID".yellow()),
            format!("{}", "PROCESS".magenta()),
            format!("{}", "COMMAND".white().bold()),
        ]);

    for p in ports {
        table.add_row(vec![
            format!("{}", p.port.to_string().cyan().bold()),
            format!("{}", p.protocol.green()),
            format!("{}", p.pid.to_string().yellow()),
            format!("{}", p.process_name.magenta()),
            p.command.clone(),
        ]);
    }

    println!("{table}");
}

pub fn print_check_result(port: u16, info: Option<&PortInfo>) {
    match info {
        Some(p) => {
            println!(
                "{}",
                tr!(
                    format!(
                        "Port {} is used by {} (PID: {})",
                        port.to_string().red().bold(),
                        p.process_name.cyan(),
                        p.pid.to_string().yellow()
                    ),
                    format!(
                        "ポート {} は {} (PID: {}) が使用中",
                        port.to_string().red().bold(),
                        p.process_name.cyan(),
                        p.pid.to_string().yellow()
                    )
                )
            );
            println!("  {} {}", tr!("Command:", "コマンド:"), p.command);
        }
        None => {
            println!(
                "{}",
                tr!(
                    format!("Port {} is {}", port.to_string().bold(), "free".green()),
                    format!("ポート {} は{}", port.to_string().bold(), "未使用".green())
                )
            );
        }
    }
}

pub fn print_kill_result(port: u16, info: &PortInfo, result: Result<(), String>, force: bool) {
    let sig = if force { "SIGKILL" } else { "SIGTERM" };
    match result {
        Ok(()) => println!(
            "{} {}",
            "✓".green(),
            tr!(
                format!(
                    "Sent {} to process {} (PID: {}) on port {}",
                    sig,
                    info.process_name.cyan(),
                    info.pid,
                    port,
                ),
                format!(
                    "ポート {} のプロセス {} (PID: {}) に {} を送信しました",
                    port,
                    info.process_name.cyan(),
                    info.pid,
                    sig,
                )
            )
        ),
        Err(e) => eprintln!("{} {}", "✗".red(), e),
    }
}

pub fn print_json(ports: &[PortInfo]) {
    let json = serde_json::to_string_pretty(ports).unwrap();
    println!("{}", json);
}
