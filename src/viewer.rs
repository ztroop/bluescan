use crossterm::event::{self, Event, KeyCode};
use ratatui::backend::Backend;
use ratatui::layout::Alignment;
use ratatui::text::Span;
use ratatui::widgets::{Block, Borders, Clear, Paragraph};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    Terminal,
};
use std::error::Error;
use std::sync::atomic::Ordering;
use std::time::Duration;

use crate::app::{App, DeviceData};
use crate::structs::DeviceInfo;
use crate::utils::centered_rect;
use crate::widgets::detail_table::detail_table;
use crate::widgets::device_table::device_table;
use crate::widgets::info_table::info_table;
use crate::widgets::inspect_overlay::inspect_overlay;

/// Displays the detected Bluetooth devices in a table and handles the user input.
/// The user can navigate the table, pause the scanning, and quit the application.
/// The detected devices are received through the provided `mpsc::Receiver`.
pub async fn viewer<B: Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
) -> Result<(), Box<dyn Error>> {
    app.table_state.select(Some(0));

    loop {
        // Draw UI
        terminal.draw(|f| {
            app.frame_count = f.count();
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints(
                    [
                        Constraint::Percentage(70),
                        Constraint::Percentage(20),
                        Constraint::Percentage(10),
                    ]
                    .as_ref(),
                )
                .split(f.size());

            let device_binding = &DeviceInfo::default();
            let selected_device = app
                .devices
                .get(app.table_state.selected().unwrap_or(0))
                .unwrap_or(device_binding);

            // Draw the device table
            let device_table = device_table(app.table_state.selected(), &app.devices);
            f.render_stateful_widget(device_table, chunks[0], &mut app.table_state);

            // Draw the detail table
            let detail_table = detail_table(selected_device);
            f.render_widget(detail_table, chunks[1]);

            // Draw the info table
            app.frame_count += 1;
            let info_table: ratatui::widgets::Table<'_> = info_table(
                app.pause_status.load(Ordering::SeqCst),
                &app.is_loading,
                &app.frame_count,
            );
            f.render_widget(info_table, chunks[2]);

            // Draw the inspect overlay
            if app.inspect_view {
                let area = centered_rect(60, 60, f.size());
                let inspect_overlay = inspect_overlay(
                    &app.selected_characteristics,
                    app.inspect_overlay_scroll,
                    area.height,
                );
                f.render_widget(Clear, area);
                f.render_widget(inspect_overlay, area);
            }

            // Draw the error overlay
            if app.error_view {
                let error_message_clone = app.error_message.clone();
                let area = centered_rect(60, 10, f.size());
                let error_block = Paragraph::new(Span::from(error_message_clone))
                    .alignment(Alignment::Center)
                    .block(Block::default().borders(Borders::ALL).title("Notification"));
                f.render_widget(Clear, area);
                f.render_widget(error_block, area);
            }
        })?;

        // Event handling
        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => {
                        break;
                    }
                    KeyCode::Char('s') => {
                        let current_state = app.pause_status.load(Ordering::SeqCst);
                        app.pause_status.store(!current_state, Ordering::SeqCst);
                    }
                    KeyCode::Char('e') => {
                        app.error_message = match app.get_devices_csv() {
                            Ok(success_message) => success_message,
                            Err(e) => e.to_string(),
                        };
                        app.error_view = true;
                    }
                    KeyCode::Enter => {
                        if app.error_view {
                            app.error_view = false;
                        } else if app.inspect_view {
                            app.inspect_view = false;
                        } else {
                            app.is_loading = true;
                            app.connect().await;
                        }
                    }
                    KeyCode::Down | KeyCode::Char('j') => {
                        if app.inspect_view {
                            app.inspect_overlay_scroll += 1;
                        } else if !app.devices.is_empty() {
                            let next = match app.table_state.selected() {
                                Some(selected) => {
                                    if selected >= app.devices.len() - 1 {
                                        0
                                    } else {
                                        selected + 1
                                    }
                                }
                                None => 0,
                            };
                            app.table_state.select(Some(next));
                        }
                    }
                    KeyCode::Up | KeyCode::Char('k') => {
                        if app.inspect_view {
                            app.inspect_overlay_scroll =
                                app.inspect_overlay_scroll.saturating_sub(1);
                        } else {
                            let previous = match app.table_state.selected() {
                                Some(selected) => {
                                    if selected == 0 {
                                        app.devices.len() - 1
                                    } else {
                                        selected - 1
                                    }
                                }
                                None => 0,
                            };
                            app.table_state.select(Some(previous));
                        }
                    }
                    _ => {}
                }
            }
        }

        // Check for updates
        if let Ok(new_device) = app.rx.try_recv() {
            match new_device {
                DeviceData::DeviceInfo(device) => app.devices.push(device),
                DeviceData::Characteristics(characteristics) => {
                    app.selected_characteristics = characteristics;
                    app.inspect_view = true;
                    app.is_loading = false;
                }
                DeviceData::Error(error) => {
                    app.error_message = error;
                    app.error_view = true;
                    app.is_loading = false;
                }
            }

            if app.table_state.selected().is_none() {
                app.table_state.select(Some(0));
            }
        }
    }
    Ok(())
}
