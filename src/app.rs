use crossterm::{
    event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
};
use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph, Widget, Wrap},
};
use ratatui_image::{StatefulImage, picker::Picker, protocol::StatefulProtocol};
use urlencoding::decode;
use std::io;
use std::time::{Duration, Instant};

use crate::playerctl::{PlayerCtl, PlayerStatus, TrackMetadata};

pub struct App {
    pub metadata: TrackMetadata,
    pub status: PlayerStatus,
    pub exit: bool,
    pub album_art: Option<StatefulProtocol>,
    pub picker: Picker,
}

impl Default for App {
    fn default() -> Self {
        Self {
            metadata: TrackMetadata::default(),
            status: PlayerStatus::default(),
            exit: false,
            album_art: None,
            picker: Picker::from_query_stdio().unwrap(),
        }
    }
}

impl App {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        self.update_metadata().ok();

        let tick_rate = Duration::from_millis(250);
        let mut last_tick = Instant::now();
        let mut last_metadata_update = Instant::now();
        let metadata_update_interval = Duration::from_secs(1);

        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;

            let timeout = tick_rate
                .checked_sub(last_tick.elapsed())
                .unwrap_or_else(|| Duration::from_secs(0));

            if event::poll(timeout)? {
                if let Event::Key(key) = event::read()? {
                    if key.kind == KeyEventKind::Press {
                        self.handle_key_event(key);
                    }
                }
            }

            if last_tick.elapsed() >= tick_rate {
                last_tick = Instant::now();
            }

            if last_metadata_update.elapsed() >= metadata_update_interval {
                self.update_metadata().ok();
                last_metadata_update = Instant::now();
            }
        }

        Ok(())
    }
    
    pub fn load_album_art(&mut self, image_path: &str) -> Result<(), Box<dyn std::error::Error>> {
        if image_path.is_empty() {
            self.clear_album_art();
            return Ok(());
        }
        let decoded_path = decode(image_path)?;
        let path = if decoded_path.starts_with("file://") {
            decoded_path.trim_start_matches("file://")
        } else {
            &decoded_path
        };
        if path.is_empty() || !std::path::Path::new(path).exists() {
            return Ok(());
        }
        let img = image::ImageReader::open(path)?
            .with_guessed_format()?
            .decode()?;
        
        // Resize to a much larger resolution for better quality
        // Increase to 1200x1200 or even higher depending on your terminal capabilities
        let resized = img.resize(1200, 1200, image::imageops::FilterType::Lanczos3);
        
        self.album_art = Some(self.picker.new_resize_protocol(resized));
        Ok(())
    }
    
    pub fn clear_album_art(&mut self) {
        self.album_art = None;
    }

    pub fn refresh_album_art(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if !self.metadata.art_uri.is_empty() {
            let uri = self.metadata.art_uri.clone();
            self.load_album_art(&uri)?;
        } else {
            self.clear_album_art();
        }
        Ok(())
    }

    pub fn update_metadata(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let metadata = PlayerCtl::get_metadata()?;
        let status = PlayerCtl::get_player_status()?;

        self.metadata = metadata;
        self.status = status;
        self.refresh_album_art()?;
        Ok(())
    }

    fn draw(&mut self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Char('n') => self.play_next(),
            KeyCode::Char('p') => self.play_previous(),
            KeyCode::Char(' ') => self.toggle_play_pause(),
            _ => {}
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }

    fn play_next(&mut self) {
        PlayerCtl::next().expect("Failed to execute 'play_next' from player-tui");
        self.update_metadata().ok();
    }

    fn play_previous(&mut self) {
        PlayerCtl::previous().expect("Failed to execute 'play_previous' from player-tui");
        self.update_metadata().ok();
    }

    fn toggle_play_pause(&mut self) {
        PlayerCtl::play_pause().expect("Failed to execute 'toggle_play_pause' from player-tui");
        self.update_metadata().ok();
    }

    fn render_album_art(&mut self, area: Rect, buf: &mut Buffer) {
        let album_block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(Color::Magenta))
            .title(" Album Art ".bold());
        let inner = album_block.inner(area);
        album_block.render(area, buf);
        
        if let Some(protocol) = &mut self.album_art {
            let image_widget = StatefulImage::default();
            ratatui::widgets::StatefulWidget::render(image_widget, inner, buf, protocol);
        } else {
            // Enhanced ASCII art fallback
            let art_lines = vec![
                "                                ",
                "        ╔══════════════╗        ",
                "        ║  ▄▄▄▄▄▄▄▄▄▄  ║        ",
                "        ║ ▐░░░░░░░░░░▌ ║        ",
                "        ║ ▐░░░▄▄▄░░░▌ ║        ",
                "        ║ ▐░░▐███▌░░▌ ║        ",
                "        ║ ▐░░░▀▀▀░░░░▌ ║        ",
                "        ║ ▐░░░░♪░░░░▌ ║        ",
                "        ║ ▐░░♫░░░♪░░▌ ║        ",
                "        ║ ▐░░░░♫░░░░▌ ║        ",
                "        ║ ▐░░░░░░░░░░▌ ║        ",
                "        ║  ▀▀▀▀▀▀▀▀▀▀  ║        ",
                "        ╚══════════════╝        ",
                "                                ",
                "        ♪  No Album Art  ♫      ",
                "                                ",
            ];
            
            let art_text: Vec<Line> = art_lines
                .iter()
                .enumerate()
                .map(|(i, line)| {
                    let color = if i < 13 {
                        Color::Cyan
                    } else {
                        Color::DarkGray
                    };
                    Line::from(Span::styled(*line, Style::default().fg(color)))
                })
                .collect();
            
            let art_paragraph = Paragraph::new(art_text)
                .alignment(Alignment::Center);
            art_paragraph.render(inner, buf);
        }
    }
    
    fn render_metadata(&self, area: Rect, buf: &mut Buffer) {
        let metadata_block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(Color::Green))
            .title(" Track Info ".bold());
        let inner = metadata_block.inner(area);
        metadata_block.render(area, buf);
        
        let title = if !self.metadata.title.is_empty() {
            &self.metadata.title
        } else {
            "Unknown Title"
        };
        let artist = if !self.metadata.artist.is_empty() {
            &self.metadata.artist
        } else {
            "Unknown Artist"
        };
        let album = if !self.metadata.album.is_empty() {
            &self.metadata.album
        } else {
            "Unknown Album"
        };
        
        let available_width = inner.width.saturating_sub(12) as usize;
        let mut metadata_lines = vec![Line::from("")];

        for (i, chunk) in title.chars().collect::<Vec<_>>().chunks(available_width).enumerate() {
            let text: String = chunk.iter().collect();
            if i == 0 {
                metadata_lines.push(Line::from(vec![
                    Span::styled("  Title:  ", Style::default().fg(Color::Gray).add_modifier(Modifier::DIM)),
                    Span::styled(text, Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
                ]));
            } else {
                metadata_lines.push(Line::from(Span::styled(format!("          {}", text), Style::default().fg(Color::White).add_modifier(Modifier::BOLD))));
            }
        }
        metadata_lines.push(Line::from(""));

        for (i, chunk) in artist.chars().collect::<Vec<_>>().chunks(available_width).enumerate() {
            let text: String = chunk.iter().collect();
            if i == 0 {
                metadata_lines.push(Line::from(vec![
                    Span::styled("  Artist: ", Style::default().fg(Color::Gray).add_modifier(Modifier::DIM)),
                    Span::styled(text, Style::default().fg(Color::Cyan)),
                ]));
            } else {
                metadata_lines.push(Line::from(Span::styled(format!("          {}", text), Style::default().fg(Color::Cyan))));
            }
        }
        metadata_lines.push(Line::from(""));

        for (i, chunk) in album.chars().collect::<Vec<_>>().chunks(available_width).enumerate() {
            let text: String = chunk.iter().collect();
            if i == 0 {
                metadata_lines.push(Line::from(vec![
                    Span::styled("  Album:  ", Style::default().fg(Color::Gray).add_modifier(Modifier::DIM)),
                    Span::styled(text, Style::default().fg(Color::Magenta)),
                ]));
            } else {
                metadata_lines.push(Line::from(Span::styled(format!("          {}", text), Style::default().fg(Color::Magenta))));
            }
        }
        metadata_lines.push(Line::from(""));

        metadata_lines.push(Line::from("  ─────────────────────────────"));
        metadata_lines.push(Line::from(""));
        metadata_lines.push(Line::from(vec![
            Span::styled("  Status: ", Style::default().fg(Color::Gray).add_modifier(Modifier::DIM)),
            Span::styled(
                format!("{}  {}", self.status.icon(), self.status.text()),
                Style::default().fg(self.status.color()).add_modifier(Modifier::BOLD),
            ),
        ]));
        
        let metadata_paragraph = Paragraph::new(metadata_lines)
            .wrap(Wrap { trim: false });
        metadata_paragraph.render(inner, buf);
    }

    fn render_footer(&self, area: Rect, buf: &mut Buffer) {
        let footer_block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(Color::Yellow));

        let inner = footer_block.inner(area);
        footer_block.render(area, buf);

        let controls = Line::from(vec![
            Span::styled(" [Space] ", Style::default().fg(Color::Black).bg(Color::Green)),
            Span::raw(" Play/Pause  "),
            Span::styled(" [N] ", Style::default().fg(Color::Black).bg(Color::Cyan)),
            Span::raw(" Next  "),
            Span::styled(" [P] ", Style::default().fg(Color::Black).bg(Color::Cyan)),
            Span::raw(" Previous  "),
            Span::styled(" [Q] ", Style::default().fg(Color::Black).bg(Color::Red)),
            Span::raw(" Quit "),
        ])
        .centered();

        Paragraph::new(controls).render(inner, buf);
    }
}

impl Widget for &mut App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let vertical_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),  // Header
                Constraint::Min(10),    // Content
                Constraint::Length(3),  // Footer
            ])
            .split(area);

        let header = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(Color::Cyan))
            .title(
                Line::from(vec![
                    Span::raw("♫ "),
                    Span::styled("Music Player TUI", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
                    Span::raw(" ♫"),
                ])
                .centered()
            );
        header.render(vertical_chunks[0], buf);

        let main_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(40), 
                Constraint::Percentage(60),
            ])
            .margin(1)
            .split(vertical_chunks[1]);

        self.render_album_art(main_chunks[0], buf);

        self.render_metadata(main_chunks[1], buf);

        self.render_footer(vertical_chunks[2], buf);
    }
}