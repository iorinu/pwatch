use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Clear, Paragraph, Row, Table};
use ratatui::Frame;

use super::app::{App, AppMode};
use crate::i18n::tr;

pub fn draw(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(1),
            Constraint::Length(1),
        ])
        .split(f.area());

    draw_table(f, app, chunks[0]);
    draw_status_bar(f, app, chunks[1]);

    if let AppMode::Confirm { force } = &app.mode {
        draw_confirm_dialog(f, app, *force);
    }
}

fn draw_table(f: &mut Frame, app: &App, area: Rect) {
    let filtered = app.filtered_ports();

    let header = Row::new(vec!["PORT", "PROTO", "PID", "PROCESS", "COMMAND"])
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD));

    let rows: Vec<Row> = filtered
        .iter()
        .enumerate()
        .map(|(i, p)| {
            let style = if i == app.selected {
                Style::default().bg(Color::DarkGray).fg(Color::White)
            } else {
                Style::default()
            };
            Row::new(vec![
                p.port.to_string(),
                p.protocol.clone(),
                p.pid.to_string(),
                p.process_name.clone(),
                p.command.clone(),
            ])
            .style(style)
        })
        .collect();

    let title = if app.filter.is_empty() {
        " pwatch ".to_string()
    } else {
        format!(" pwatch [{}: {}] ", tr!("search", "検索"), app.filter)
    };

    let table = Table::new(
        rows,
        [
            Constraint::Length(8),
            Constraint::Length(6),
            Constraint::Length(8),
            Constraint::Length(20),
            Constraint::Fill(1),
        ],
    )
    .header(header)
    .block(Block::default().borders(Borders::ALL).title(title));

    f.render_widget(table, area);
}

fn draw_status_bar(f: &mut Frame, app: &App, area: Rect) {
    let mode_text = match &app.mode {
        AppMode::Normal => "NORMAL",
        AppMode::Search => "SEARCH",
        AppMode::Confirm { .. } => "CONFIRM",
    };

    let help = match &app.mode {
        AppMode::Normal => tr!(
            "q:quit j/k:move d:kill D:force-kill /:search r:refresh a:auto +/-:interval",
            "q:終了 j/k:移動 d:kill D:force-kill /:検索 r:更新 a:自動 +/-:間隔"
        ),
        AppMode::Search => tr!("Enter:confirm Esc:cancel", "Enter:確定 Esc:キャンセル"),
        AppMode::Confirm { .. } => tr!("y:yes n:cancel", "y:実行 n:キャンセル"),
    };

    // 自動更新インジケータ
    let auto_indicator = if app.auto_refresh {
        format!(" AUTO {}s ", app.refresh_interval.as_secs_f64())
    } else {
        String::new()
    };

    let msg = app.message.as_deref().unwrap_or("");

    let line = Line::from(vec![
        Span::styled(
            format!(" {} ", mode_text),
            Style::default().bg(Color::Blue).fg(Color::White).add_modifier(Modifier::BOLD),
        ),
        Span::styled(
            auto_indicator,
            Style::default().bg(Color::Green).fg(Color::Black).add_modifier(Modifier::BOLD),
        ),
        Span::raw(" "),
        Span::styled(help, Style::default().fg(Color::DarkGray)),
        Span::raw("  "),
        Span::styled(msg, Style::default().fg(Color::Yellow)),
    ]);

    f.render_widget(Paragraph::new(line), area);
}

fn draw_confirm_dialog(f: &mut Frame, app: &App, force: bool) {
    let area = f.area();
    let dialog_width = 50u16.min(area.width.saturating_sub(4));
    let dialog_height = 5u16;
    let x = (area.width.saturating_sub(dialog_width)) / 2;
    let y = (area.height.saturating_sub(dialog_height)) / 2;
    let dialog_area = Rect::new(x, y, dialog_width, dialog_height);

    f.render_widget(Clear, dialog_area);

    let sig = if force {
        tr!("SIGKILL (force)", "SIGKILL (強制)")
    } else {
        "SIGTERM"
    };
    let text = if let Some(info) = app.selected_port() {
        tr!(
            format!(
                "Send {} to PID {} ({})?\n\n  y: confirm  n: cancel",
                sig, info.pid, info.process_name
            ),
            format!(
                "{} を PID {} ({}) に送信しますか?\n\n  y: 実行  n: キャンセル",
                sig, info.pid, info.process_name
            )
        )
    } else {
        tr!("No port selected", "ポートが選択されていません").to_string()
    };

    let paragraph = Paragraph::new(text).block(
        Block::default()
            .borders(Borders::ALL)
            .title(tr!(" Confirm ", " 確認 "))
            .border_style(Style::default().fg(Color::Red)),
    );

    f.render_widget(paragraph, dialog_area);
}
