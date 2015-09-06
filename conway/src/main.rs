extern crate rustty;
use rustty::{Terminal, Cell, Style, Color, Event};
use std::thread::sleep_ms;

fn main() {
    let mut term = Terminal::new().unwrap();
    let white = Style::with_color(Color::White);
    let black = Style::with_color(Color::Black);

    loop {
        term.clear_with_styles(white, white).unwrap();
        let evt = term.get_event(0).unwrap();
        if evt.is_some() {
            match evt.unwrap() {
                Event::Key('s') => break, //break
                Event::Key(_)   => {},    //do nothing
            }
        }
        //main code
        


        sleep_ms(10u32);
        term.swap_buffers().unwrap();
    }
}
