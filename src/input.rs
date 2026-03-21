use std::io;

use crossterm::event;
use crossterm::terminal;

pub fn enable_raw_mode() -> io::Result<()> {
    terminal::enable_raw_mode()
}

pub fn disable_raw_mode() -> io::Result<()> {
    terminal::disable_raw_mode()
}

pub fn read_event() -> io::Result<event::Event> {
    event::read()
}
