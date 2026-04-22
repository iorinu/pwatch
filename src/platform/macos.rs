use super::PortScanner;
use crate::port::PortInfo;
use std::collections::HashMap;
use std::process::Command;

pub struct MacosScanner;

impl PortScanner for MacosScanner {
    fn scan() -> Vec<PortInfo> {
        let output = match Command::new("lsof")
            .args(["-iTCP", "-sTCP:LISTEN", "-P", "-n", "-l"])
            .output()
        {
            Ok(o) => o,
            Err(_) => return Vec::new(),
        };

        let stdout = String::from_utf8_lossy(&output.stdout);
        let partial: Vec<(u32, String, u16)> = stdout
            .lines()
            .skip(1) // ヘッダー行をスキップ
            .filter_map(parse_lsof_line)
            .collect();

        // 全PIDのコマンドラインを1回のpsで取得
        let pids: Vec<u32> = partial.iter().map(|(pid, _, _)| *pid).collect();
        let cmd_map = batch_get_commands(&pids);

        let mut result: Vec<PortInfo> = partial
            .into_iter()
            .map(|(pid, process_name, port)| PortInfo {
                port,
                protocol: "tcp".to_string(),
                pid,
                process_name,
                command: cmd_map.get(&pid).cloned().unwrap_or_default(),
            })
            .collect();

        result.sort_by_key(|p| p.port);
        result
    }
}

/// lsof出力の1行をパース
/// 形式: COMMAND PID USER FD TYPE DEVICE SIZE/OFF NODE NAME
fn parse_lsof_line(line: &str) -> Option<(u32, String, u16)> {
    let fields: Vec<&str> = line.split_whitespace().collect();
    if fields.len() < 9 {
        return None;
    }

    let process_name = fields[0].to_string();
    let pid: u32 = fields[1].parse().ok()?;

    // NAME列は最終フィールドを使う（フィールド数が変動しても安全）
    let name = fields.last()?;
    let port_str = name.rsplit(':').next()?;
    let port: u16 = port_str.parse().ok()?;

    Some((pid, process_name, port))
}

/// 1回のps呼び出しで全PIDのコマンドラインをまとめて取得
fn batch_get_commands(pids: &[u32]) -> HashMap<u32, String> {
    let mut map = HashMap::new();
    if pids.is_empty() {
        return map;
    }

    let pid_args: Vec<String> = pids.iter().map(|p| p.to_string()).collect();
    let output = Command::new("ps")
        .args(["-o", "pid=,command=", "-p"])
        .arg(pid_args.join(","))
        .output();

    if let Ok(o) = output {
        let stdout = String::from_utf8_lossy(&o.stdout);
        for line in stdout.lines() {
            let trimmed = line.trim_start();
            if let Some(pos) = trimmed.find(' ') {
                if let Ok(pid) = trimmed[..pos].parse::<u32>() {
                    map.insert(pid, trimmed[pos..].trim_start().to_string());
                }
            }
        }
    }

    map
}
