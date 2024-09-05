use ratatui::{
    layout::{Constraint, Layout, Rect},
    prelude::Direction,
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};

use crate::app::{App, CurrentScreen, CurrentlyAdding};

pub fn ui(frame: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(1),
            Constraint::Length(3),
        ])
        .split(frame.area());

    let title_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

    let title = Paragraph::new(Text::styled(
        "SMAPI Instance Manager",
        Style::default().fg(Color::Green),
    ))
    .block(title_block);
    frame.render_widget(title, chunks[0]);

    let mut list_instances = Vec::<ListItem>::new();
    let active_style = Style::default()
        .fg(Color::Green)
        .add_modifier(Modifier::BOLD);

    for key in app.manager.instances.keys().cloned() {
        if key == app.current_instance {
            list_instances.push(ListItem::new(Line::from(Span::styled(
                format!("{: <25}", key),
                active_style,
            ))));
        } else {
            list_instances.push(ListItem::new(Line::from(Span::styled(
                format!("{: <25}", key),
                Style::default().fg(Color::Yellow),
            ))));
        }
    }

    let list = List::new(list_instances);
    frame.render_widget(list, chunks[1]);

    let current_navigation_text = vec![
        match app.screen {
            CurrentScreen::Main => Span::styled("Main Menu", Style::default().fg(Color::Green)),
            CurrentScreen::Add => Span::styled("Adding Menu", Style::default().fg(Color::Yellow)),
            CurrentScreen::Remove => Span::styled("Removing Menu", Style::default().fg(Color::Red)),
            CurrentScreen::Exit => {
                Span::styled("Exiting Menu", Style::default().fg(Color::LightRed))
            }
        }
        .to_owned(),
        Span::styled(" | ", Style::default().fg(Color::White)),
        {
            if let Some(adding) = &app.adding {
                match adding {
                    CurrentlyAdding::Name => {
                        Span::styled("Editing Name", Style::default().fg(Color::Green))
                    }
                    CurrentlyAdding::FolderName => {
                        Span::styled("Editing Folder Name", Style::default().fg(Color::Green))
                    }
                    CurrentlyAdding::SmapiPath => {
                        Span::styled("Editing Smapi Path", Style::default().fg(Color::Green))
                    }
                }
            } else {
                Span::styled("Not Editing Anything", Style::default().fg(Color::DarkGray))
            }
        },
    ];

    let mode_footer = Paragraph::new(Line::from(current_navigation_text))
        .block(Block::default().borders(Borders::ALL));
    let current_key_hints = {
        match app.screen {
            CurrentScreen::Add => Span::styled(
                "(Esc) to cancle/(Tab) to switch boxes/(Enter) to complete", 
                Style::default().fg(Color::Red)
            ),
            CurrentScreen::Main => Span::styled(
                "(a) to add new instance/(q) to quit/(r) to remove selected instance/(Enter) to start selected instance/ (up) and (down) to selecte instance", 
                Style::default().fg(Color::Red)
            ),
            CurrentScreen::Exit => Span::styled(
                "(q) or (y) to quit/(n) to go back to main menu", 
                Style::default().fg(Color::Red)
            ),
            CurrentScreen::Remove => Span::styled(
                "(n) to cancle/(y) to remove instance/(a) to delete with folder", 
                Style::default().fg(Color::Red)
            ),
        }
    };

    let key_notest_footer =
        Paragraph::new(Line::from(current_key_hints)).block(Block::default().borders(Borders::ALL));

    let footer_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(chunks[2]);
    frame.render_widget(mode_footer, footer_chunks[0]);
    frame.render_widget(key_notest_footer, footer_chunks[1]);
}

fn centered_rect(percentage_x: u16, percentage_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percentage_y) / 2),
            Constraint::Percentage(percentage_y),
            Constraint::Percentage((100 - percentage_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percentage_x) / 2),
            Constraint::Percentage(percentage_x),
            Constraint::Percentage((100 - percentage_x) / 2),
        ])
        .split(popup_layout[1])[1]
}
