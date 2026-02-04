use ratatui;
use std::io;

mod app;
mod playerctl;

use crate::app::App;

fn main() -> io::Result<()> {
    ratatui::run(|terminal| App::default().run(terminal))
}
