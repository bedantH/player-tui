use std::io;
use std::process::Command;
use ratatui::style::Color;
pub struct PlayerCtl;

#[derive(Debug, Default)]
pub struct TrackMetadata {
    pub artist: String,
    pub title: String,
    pub album: String,
    pub track_id: String,
    pub art_uri: String,
}

#[derive(Debug, Default)]
pub enum PlayerStatus {
    #[default]
    Playing,
    Paused,
    Stopped,
}

impl PlayerStatus {
    pub fn toggle_status(self) -> PlayerStatus {
        match self {
            PlayerStatus::Playing => PlayerStatus::Paused,
            PlayerStatus::Paused => PlayerStatus::Playing,
            PlayerStatus::Stopped => PlayerStatus::Playing,
        }
    }

    pub fn icon(&self) -> &str {
        match self {
            PlayerStatus::Playing => "▶",
            PlayerStatus::Paused => "⏸",
            PlayerStatus::Stopped => "⏹",
        }
    }

    pub fn color(&self) -> Color {
        match self {
            PlayerStatus::Playing => Color::Green,
            PlayerStatus::Paused => Color::Yellow,
            PlayerStatus::Stopped => Color::Red,
        }
    }

    pub fn text(&self) -> &str {
        match self {
            PlayerStatus::Playing => "Playing",
            PlayerStatus::Paused => "Paused",
            PlayerStatus::Stopped => "Stopped",
        }
    }
}

impl PlayerCtl {
    pub fn get_player_status() -> io::Result<PlayerStatus> {
        let output = Command::new("playerctl").arg("status").output()?;

        let status = String::from_utf8_lossy(&output.stdout).trim().to_string();

        match status.as_str() {
            "Playing" => Ok(PlayerStatus::Playing),
            "Paused" => Ok(PlayerStatus::Paused),
            _ => Ok(PlayerStatus::Stopped),
        }
    }

    pub fn play_pause() -> io::Result<()> {
        let output = Command::new("playerctl").arg("play-pause").output()?;

        let output_text = String::from_utf8_lossy(&output.stdout).trim().to_string();

        if output_text == "" {
            Ok(())
        } else {
            Err(io::Error::new(
                io::ErrorKind::Other,
                String::from_utf8_lossy(&output.stderr).to_string(),
            ))
        }
    }

    pub fn next() -> io::Result<()> {
        let output = Command::new("playerctl").arg("next").output()?;

        let output_text = String::from_utf8_lossy(&output.stdout).trim().to_string();

        if output_text == "" {
            Ok(())
        } else {
            Err(io::Error::new(
                io::ErrorKind::Other,
                String::from_utf8_lossy(&output.stderr).to_string(),
            ))
        }
    }

    pub fn previous() -> io::Result<()> {
        let output = Command::new("playerctl").arg("previous").output()?;

        let output_text = String::from_utf8_lossy(&output.stdout).trim().to_string();

        if output_text == "" {
            Ok(())
        } else {
            Err(io::Error::new(
                io::ErrorKind::Other,
                String::from_utf8_lossy(&output.stderr).to_string(),
            ))
        }
    }

    pub fn get_metadata() -> io::Result<TrackMetadata> {
        let output = Command::new("playerctl")
            .args(vec!["metadata", "--format", "{{xesam:artist}}\t{{xesam:title}}\t{{xesam:album}}\t{{mpris:trackid}}\t{{mpris:artUrl}}"])
            .output()?;

        let output_text = String::from_utf8_lossy(&output.stdout);
        let trimmed = output_text.trim();

        let outputs: Vec<&str> = trimmed.split('\t').collect();

        let metadata = TrackMetadata {
            artist: outputs.get(0).unwrap_or(&"").to_string(),
            title: outputs.get(1).unwrap_or(&"").to_string(),
            album: outputs.get(2).unwrap_or(&"").to_string(),
            track_id: outputs.get(3).unwrap_or(&"").to_string(),
            art_uri: outputs.get(4).unwrap_or(&"").to_string(),
        };

        return Ok(metadata);
    }
}
