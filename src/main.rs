mod config;
mod display;
mod i18n;
mod platform;
mod port;
mod tui;
use crate::i18n::{Lang, tr};
use clap::{Parser, Subcommand};
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
    List,
    /// Check usage of a specific port
    Check { port: u16 },
    /// Kill the process bound to a port
    Kill {
        port: u16,
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
}

fn main() {
    let args = Cli::parse();
    let cfg = config::load();
    i18n::init(cfg.language);

    if cfg.show_banner && !args.json {
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
        Command::List => {
            let ports = port::scan();
            if args.json {
                display::print_json(&ports);
            } else {
                display::print_port_list(&ports);
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
        Command::Kill { port: p, force } => {
            let info = match port::check(p) {
                Some(info) => info,
                None => {
                    println!("{}", tr!(
                        format!("Port {} is not in use", p),
                        format!("ポート {} は未使用です", p)
                    ));
                    return;
                }
            };
            let result = port::kill_process(info.pid, force);
            display::print_kill_result(p, &info, result, force);
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
    }
}
