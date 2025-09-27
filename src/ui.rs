use crate::app::{App, AppMode};
use ratatui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph},
    Frame,
};

// Catppuccin Mocha color palette
const MAUVE: Color = Color::Rgb(203, 166, 247);
const LAVENDER: Color = Color::Rgb(180, 190, 254);
const TEXT: Color = Color::Rgb(205, 214, 244);
const SUBTEXT1: Color = Color::Rgb(186, 194, 222);
const SURFACE2: Color = Color::Rgb(88, 91, 112);
const SURFACE1: Color = Color::Rgb(69, 71, 90);
const SURFACE0: Color = Color::Rgb(49, 50, 68);
const CRUST: Color = Color::Rgb(17, 17, 27);

pub fn ui<B: Backend>(f: &mut Frame, app: &mut App) {
    // Create a global background
    let background = Block::default().style(Style::default().bg(CRUST));
    f.render_widget(background, f.size());

    // Create a centered viewport based on the margin/zoom level
    let vertical_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(app.margin), // Top padding
            Constraint::Min(0),             // Content
            Constraint::Length(app.margin), // Bottom padding
        ].as_ref())
        .split(f.size());

    let horizontal_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(app.margin * 2), // Left padding (x2 for better aspect ratio)
            Constraint::Min(0),                  // Content
            Constraint::Length(app.margin * 2),  // Right padding
        ].as_ref())
        .split(vertical_chunks[1]);

    let viewport = horizontal_chunks[1];

    // Render the application within the calculated viewport
    let app_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(3)].as_ref())
        .split(viewport);

    render_tasks(f, app, app_chunks[0]);
    render_footer(f, app_chunks[1]);

    if let AppMode::Insert = app.mode {
        render_input_popup(f, app);
    }
}

fn render_tasks(f: &mut Frame, app: &mut App, area: Rect) {
    let items: Vec<ListItem> = app
        .tasks
        .iter()
        .map(|task| {
            let (style, symbol) = if task.completed {
                (
                    Style::default().fg(SURFACE2).add_modifier(Modifier::CROSSED_OUT),
                    " ✔ ",
                )
            } else {
                (Style::default().fg(TEXT), " ❯ ")
            };

            let content = Line::from(vec![
                Span::styled(symbol, Style::default().fg(MAUVE)),
                Span::raw(task.description.clone()),
            ]);

            ListItem::new(content).style(style)
        })
        .collect();

    let list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" To-Do ")
                .border_style(Style::default().fg(SURFACE1))
                .title_style(Style::default().fg(LAVENDER)),
        )
        .highlight_style(
            Style::default()
                .bg(SURFACE0)
                .fg(LAVENDER)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(" שׂ ");

    f.render_stateful_widget(list, area, &mut app.state);
}

fn render_footer(f: &mut Frame, area: Rect) {
    let key_style = Style::default().fg(MAUVE).add_modifier(Modifier::BOLD);
    let desc_style = Style::default().fg(SUBTEXT1);

    let help_spans = Line::from(vec![
        Span::styled("q", key_style),
        Span::styled(":quit ", desc_style),
        Span::styled("a", key_style),
        Span::styled(":add ", desc_style),
        Span::styled("d", key_style),
        Span::styled(":delete ", desc_style),
        Span::styled("+/-", key_style),
        Span::styled(":zoom", desc_style),
    ]);

    let help = Paragraph::new(help_spans)
        .block(Block::default().borders(Borders::ALL)
        .border_style(Style::default().fg(SURFACE1))
        .title(" Controls ")
        .title_style(Style::default().fg(LAVENDER)));

    f.render_widget(help, area);
}

fn render_input_popup(f: &mut Frame, app: &App) {
    let area = centered_rect(60, 20, f.size());
    let input_block = Paragraph::new(app.input.as_str())
        .block(
            Block::default()
                .title(" New Task ")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(MAUVE))
                .title_style(Style::default().fg(LAVENDER)),
        )
        .style(Style::default().fg(TEXT));

    f.render_widget(Clear, area); //this clears the background
    f.render_widget(input_block, area);
}

/// helper function to create a centered rect using up certain percentage of the available rect `r`
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_y) / 2),
            ]
            .as_ref(),
        )
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ]
            .as_ref(),
        )
        .split(popup_layout[1])[1]
}
