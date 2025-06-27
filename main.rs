use crossterm::event::{self, Event};
use ratatui::{Frame, text::Text};
use std::{thread, time::Duration};

fn main() {
    let mut terminal = ratatui::init();
    loop {
        terminal.draw(draw).expect("failed to draw frame");
        if matches!(event::read().expect("failed to read"), Event::Key(_)) {
            break;
        }
        thread::sleep(Duration::from_secs(3));
        terminal.draw(draw2).expect("failed to draw frame");
        if matches!(event::read().expect("failed to read"), Event::Key(_)) {
            break;
        }
        thread::sleep(Duration::from_secs(3));
    }
    ratatui::restore();
}

fn draw(frame: &mut Frame) {
    let text = Text::raw("learning ratatui.... seems interesting so far!!");
    frame.render_widget(text, frame.area());
}

fn draw2(frame: &mut Frame) {
    let text = Text::raw("heres another thing im drawing");
    frame.render_widget(text, frame.area());
}
