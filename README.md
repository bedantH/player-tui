# player-tui

A vibrant and lightweight Terminal User Interface (TUI) for controlling media players on Linux via `playerctl`. Built with Rust and `ratatui`, it provides a sleek way to visualize and control your currently playing audio.

![App Screenshot](https://raw.githubusercontent.com/ratatui/ratatui/main/assets/logo.png) <!-- Placeholder for actual screenshot if available -->

## Features

- **Album Art Visualization**: High-quality album art rendering directly in your terminal (supports various protocols via `ratatui-image`).
- **Dynamic Metadata Display**: Real-time updates for Track Title, Artist, and Album.
- **Playback Controls**: Easily Play/Pause, Skip to Next, or go back to the Previous track.
- **Status Indicators**: Visual cues for "Playing", "Paused", and "Stopped" states.
- **Modern UI**: Rounded borders and a curated color palette for a premium look.

## Prerequisites

- **Linux OS** (Requires `playerctl` for media control).
- **Playerctl**: Ensure `playerctl` is installed and a compatible media player (e.g., Spotify, VLC, MPD) is running.
- **Rust**: [Rustup](https://rustup.rs/) (to build from source).

## Installation

1. **Clone the repository**:
   ```bash
   git clone https://github.com/bedanth/player-tui.git
   cd player-tui
   ```

2. **Build and Install**:
   ```bash
   cargo build --release
   ```
   The binary will be located at `./target/release/player-tui`.

## Usage

Run the application:
```bash
./target/release/player-tui
```

### Keybindings

| Key | Action |
|-----|--------|
| `Space` | Toggle Play/Pause |
| `n` | Next Track |
| `p` | Previous Track |
| `q` | Quit |

## License

Copyright (c) bedantH <bedanthota@gmail.com>

This project is licensed under the MIT license ([LICENSE] or <http://opensource.org/licenses/MIT>)

[LICENSE]: ./LICENSE
