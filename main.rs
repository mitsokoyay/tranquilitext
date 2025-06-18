use crossterm::event::{self, Event};
use ratatui::{text::Text, Frame};


fn main(){
    let mut terminal = ratatui::init();
    loop{
        terminal.draw(draw).expect("failed to draw frame");
        if matches!(event::read().expect("failed to read"), Event::Key(_)){
            break;
        }
    }
    ratatui::restore();
}


fn draw(frame: &mut Frame){
    let text = Text::raw("learning ratatui.... seems interesting so far!!");
    frame.render_widget(text, frame.area());
}
