use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use std::time::Duration;

use super::app::{App, AppMode};

pub fn handle_events(app: &mut App) -> std::io::Result<()> {
    // auto_refresh が ON のとき、次の更新タイミングまで待つ。
    // それ以外は 100ms ごとにポーリングする (キー入力レスポンス確保)
    let timeout = app
        .time_until_next_refresh()
        .map(|d| d.min(Duration::from_millis(100)))
        .unwrap_or_else(|| Duration::from_millis(100));

    if event::poll(timeout)?
        && let Event::Key(key) = event::read()?
    {
        if key.kind != KeyEventKind::Press {
            return Ok(());
        }

        match &app.mode {
            AppMode::Normal => handle_normal(app, key.code),
            AppMode::Search => handle_search(app, key.code),
            AppMode::Confirm { force } => {
                let force = *force;
                handle_confirm(app, key.code, force);
            }
        }
    }

    // タイマー満了による自動更新
    if let Some(remaining) = app.time_until_next_refresh()
        && remaining.is_zero()
    {
        app.auto_refresh_tick();
    }
    Ok(())
}

fn handle_normal(app: &mut App, code: KeyCode) {
    match code {
        KeyCode::Char('q') | KeyCode::Esc => app.should_quit = true,
        KeyCode::Up | KeyCode::Char('k') => app.move_up(),
        KeyCode::Down | KeyCode::Char('j') => app.move_down(),
        KeyCode::Char('d') => {
            if app.selected_port().is_some() {
                app.mode = AppMode::Confirm { force: false };
            }
        }
        KeyCode::Char('D') => {
            if app.selected_port().is_some() {
                app.mode = AppMode::Confirm { force: true };
            }
        }
        KeyCode::Char('/') => {
            app.filter.clear();
            app.selected = 0;
            app.mode = AppMode::Search;
        }
        KeyCode::Char('r') => app.refresh(),
        // 自動更新のトグルと間隔調整
        KeyCode::Char('a') => app.toggle_auto_refresh(),
        KeyCode::Char('+') | KeyCode::Char('=') => app.change_interval(0.5),
        KeyCode::Char('-') | KeyCode::Char('_') => app.change_interval(-0.5),
        _ => {}
    }
}

fn handle_search(app: &mut App, code: KeyCode) {
    match code {
        KeyCode::Enter => {
            app.selected = 0;
            app.mode = AppMode::Normal;
        }
        KeyCode::Esc => {
            app.filter.clear();
            app.selected = 0;
            app.mode = AppMode::Normal;
        }
        KeyCode::Backspace => {
            app.filter.pop();
            app.selected = 0;
        }
        KeyCode::Char(c) => {
            app.filter.push(c);
            app.selected = 0;
        }
        _ => {}
    }
}

fn handle_confirm(app: &mut App, code: KeyCode, force: bool) {
    match code {
        KeyCode::Char('y') => app.kill_selected(force),
        KeyCode::Char('n') | KeyCode::Esc => app.mode = AppMode::Normal,
        _ => {}
    }
}
