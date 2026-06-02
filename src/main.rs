mod config;
mod display;
mod i18n;
mod platform;
mod port;
mod tui;
use crate::i18n::{Lang, tr};
use clap::{CommandFactory, Parser, Subcommand};
use clap_complete::{Shell, generate};
use colored::Colorize;
use figlet_rs::FIGfont;

#[derive(Parser)]
#[command(
    name = "pwatch",
    about = "A fast, friendly port viewer and process killer"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
    /// Output as JSON
    #[arg(long, global = true)]
    pub json: bool,
}

#[derive(Subcommand)]
pub enum Command {
    /// List all listening ports in a table
    List {
        /// Continuously refresh the listing
        #[arg(short = 'w', long)]
        watch: bool,
        /// Refresh interval in seconds (used with --watch)
        #[arg(long, default_value_t = 2.0, value_name = "SECONDS")]
        interval: f64,
    },
    /// Check usage of a specific port
    Check { port: u16 },
    /// Kill the process(es) bound to one or more ports
    Kill {
        /// One or more ports (e.g. `pwatch kill 8080 3000 5173`)
        #[arg(required = true, num_args = 1..)]
        ports: Vec<u16>,
        /// Force kill with SIGKILL
        #[arg(long)]
        force: bool,
    },
    /// Launch the TUI mode
    Ui,
    /// Update configuration
    Config {
        /// Config key (banner, lang)
        key: String,
        /// Value (banner: on/off, lang: en/ja)
        value: String,
    },
    /// Generate shell completion script (bash, zsh, fish, powershell, elvish)
    Completion {
        /// Target shell
        shell: Shell,
    },
}

fn main() {
    let args = Cli::parse();
    let cfg = config::load();
    i18n::init(cfg.language);

    // watch / completion はバナー抑制 (前者は画面クリア、後者はスクリプト純粋出力のため)
    let is_watch = matches!(&args.command, Command::List { watch: true, .. });
    let is_completion = matches!(&args.command, Command::Completion { .. });
    if cfg.show_banner && !args.json && !is_watch && !is_completion {
        let standard_font = FIGfont::standard().unwrap();
        let figure = standard_font.convert("pwatch").unwrap();
        let colors = ["red", "yellow", "green", "cyan", "blue", "magenta"];
        for line in figure.to_string().lines() {
            for (i, ch) in line.chars().enumerate() {
                print!("{}", ch.to_string().color(colors[i % colors.len()]));
            }
            println!();
        }
    }

    match args.command {
        Command::List { watch, interval } => {
            if watch {
                run_watch(args.json, interval);
            } else {
                let ports = port::scan();
                if args.json {
                    display::print_json(&ports);
                } else {
                    display::print_port_list(&ports);
                }
            }
        }
        Command::Check { port: p } => {
            let info = port::check(p);
            if args.json {
                display::print_json(&info.into_iter().collect::<Vec<_>>());
            } else {
                display::print_check_result(p, info.as_ref());
            }
        }
        Command::Kill { ports, force } => {
            // 指定されたポートを順に処理。各ポートで kill 結果をその場で出力する
            for p in ports {
                let info = match port::check(p) {
                    Some(info) => info,
                    None => {
                        println!("{}", tr!(
                            format!("Port {} is not in use", p),
                            format!("ポート {} は未使用です", p)
                        ));
                        continue;
                    }
                };
                let result = port::kill_process(info.pid, force);
                display::print_kill_result(p, &info, result, force);
            }
        }
        Command::Ui => {
            let mut terminal = ratatui::init();
            let mut app = tui::app::App::new();
            loop {
                terminal
                    .draw(|f| tui::ui::draw(f, &app))
                    .expect("failed to render frame");
                tui::handler::handle_events(&mut app).expect("failed to handle events");
                if app.should_quit {
                    break;
                }
            }
            ratatui::restore();
        }
        Command::Config { key, value } => {
            let mut cfg = cfg;
            match key.as_str() {
                "banner" => match value.as_str() {
                    "on" => {
                        cfg.show_banner = true;
                        config::save(&cfg).expect("failed to save config");
                        println!("{}", tr!("Banner enabled", "バナー表示を有効にしました"));
                    }
                    "off" => {
                        cfg.show_banner = false;
                        config::save(&cfg).expect("failed to save config");
                        println!("{}", tr!("Banner disabled", "バナー表示を無効にしました"));
                    }
                    _ => eprintln!("{}", tr!(
                        "Value must be on or off",
                        "値は on または off を指定してください"
                    )),
                },
                "lang" => match Lang::parse(&value) {
                    Some(lang) => {
                        cfg.language = lang;
                        config::save(&cfg).expect("failed to save config");
                        // 保存後の言語で表示するため再初期化したいが OnceLock は再設定不可
                        // → 直接 match して表示
                        let msg = match lang {
                            Lang::En => "Language set to English",
                            Lang::Ja => "言語を日本語に設定しました",
                        };
                        println!("{}", msg);
                    }
                    None => eprintln!("{}", tr!(
                        "Value must be en or ja",
                        "値は en または ja を指定してください"
                    )),
                },
                _ => eprintln!("{}", tr!(
                    format!("Unknown config key: {} (available: banner, lang)", key),
                    format!("不明な設定項目: {} (使用可能: banner, lang)", key)
                )),
            }
        }
        Command::Completion { shell } => {
            // 補完スクリプトを stdout に出力。バナーは事前に抑制済み
            let mut cmd = Cli::command();
            let bin_name = cmd.get_name().to_string();
            generate(shell, &mut cmd, bin_name, &mut std::io::stdout());
        }
    }
}

// watch モード本体: 画面をクリアしながら一定間隔で再描画する
fn run_watch(json: bool, interval: f64) {
    // あまりに短い間隔は CPU を食い潰すのでガード
    let interval = interval.max(0.1);
    let duration = std::time::Duration::from_secs_f64(interval);
    loop {
        // ANSI: 画面消去 (ESC[2J) + カーソル左上へ (ESC[H)
        print!("\x1B[2J\x1B[H");
        let ports = port::scan();
        if json {
            display::print_json(&ports);
        } else {
            display::print_port_list(&ports);
        }
        let footer = tr!(
            format!("Refreshing every {}s — Ctrl+C to exit", interval),
            format!("更新間隔 {}秒 — Ctrl+C で終了", interval)
        );
        println!("\n{}", footer.dimmed());
        std::thread::sleep(duration);
    }
}
