use crate::i18n::tr;
use crate::port::{self, PortInfo};
use std::time::{Duration, Instant};

#[derive(Debug, Clone, Default, PartialEq)]
pub enum AppMode {
    #[default]
    Normal,
    Search,
    Confirm {
        force: bool,
    },
}

#[derive(Debug, Clone)]
pub struct App {
    pub ports: Vec<PortInfo>,
    pub selected: usize,
    pub filter: String,
    pub mode: AppMode,
    pub should_quit: bool,
    pub message: Option<String>,
    // 自動更新の有効/無効と間隔。Instant は最後にスキャンした時刻
    pub auto_refresh: bool,
    pub refresh_interval: Duration,
    pub last_refresh: Instant,
}

impl App {
    pub fn new() -> Self {
        let ports = port::scan();
        Self {
            ports,
            selected: 0,
            filter: String::new(),
            mode: AppMode::Normal,
            should_quit: false,
            message: None,
            auto_refresh: false,
            refresh_interval: Duration::from_secs(2),
            last_refresh: Instant::now(),
        }
    }

    // auto_refresh が ON のとき、次の更新まで残り時間を返す
    pub fn time_until_next_refresh(&self) -> Option<Duration> {
        if !self.auto_refresh {
            return None;
        }
        let elapsed = self.last_refresh.elapsed();
        Some(self.refresh_interval.saturating_sub(elapsed))
    }

    pub fn toggle_auto_refresh(&mut self) {
        self.auto_refresh = !self.auto_refresh;
        self.last_refresh = Instant::now();
        self.message = Some(if self.auto_refresh {
            tr!(
                format!("Auto-refresh ON ({}s)", self.refresh_interval.as_secs_f64()),
                format!("自動更新 ON ({}秒)", self.refresh_interval.as_secs_f64())
            )
        } else {
            tr!("Auto-refresh OFF", "自動更新 OFF").to_string()
        });
    }

    pub fn change_interval(&mut self, delta_secs: f64) {
        let new_secs = (self.refresh_interval.as_secs_f64() + delta_secs).clamp(0.5, 60.0);
        self.refresh_interval = Duration::from_secs_f64(new_secs);
        self.message = Some(tr!(
            format!("Interval: {}s", new_secs),
            format!("間隔: {}秒", new_secs)
        ));
    }

    pub fn refresh(&mut self) {
        self.ports = port::scan();
        if self.selected >= self.ports.len() && !self.ports.is_empty() {
            self.selected = self.ports.len() - 1;
        }
        self.last_refresh = Instant::now();
        self.message = Some(tr!("Refreshed", "リフレッシュしました").to_string());
    }

    // 自動更新タイマーが満了したときの静かなリフレッシュ (メッセージは出さない)
    pub fn auto_refresh_tick(&mut self) {
        self.ports = port::scan();
        if self.selected >= self.ports.len() && !self.ports.is_empty() {
            self.selected = self.ports.len() - 1;
        }
        self.last_refresh = Instant::now();
    }

    pub fn filtered_ports(&self) -> Vec<&PortInfo> {
        if self.filter.is_empty() {
            self.ports.iter().collect()
        } else {
            self.ports
                .iter()
                .filter(|p| {
                    p.port.to_string().contains(&self.filter)
                        || p.process_name.contains(&self.filter)
                        || p.command.contains(&self.filter)
                })
                .collect()
        }
    }

    pub fn move_up(&mut self) {
        if self.selected > 0 {
            self.selected -= 1;
        }
    }

    pub fn move_down(&mut self) {
        let len = self.filtered_ports().len();
        if len > 0 && self.selected < len - 1 {
            self.selected += 1;
        }
    }

    pub fn selected_port(&self) -> Option<&PortInfo> {
        self.filtered_ports().get(self.selected).copied()
    }

    pub fn kill_selected(&mut self, force: bool) {
        if let Some(info) = self.selected_port().cloned() {
            match port::kill_process(info.pid, force) {
                Ok(()) => {
                    let sig = if force { "SIGKILL" } else { "SIGTERM" };
                    self.message = Some(tr!(
                        format!(
                            "Sent {} to PID {} ({})",
                            sig, info.pid, info.process_name
                        ),
                        format!(
                            "PID {} ({}) に {} を送信しました",
                            info.pid, info.process_name, sig
                        )
                    ));
                    self.refresh();
                }
                Err(e) => {
                    self.message = Some(tr!(
                        format!("Error: {}", e),
                        format!("エラー: {}", e)
                    ));
                }
            }
        }
        self.mode = AppMode::Normal;
    }
}
