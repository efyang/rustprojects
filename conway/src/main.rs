extern crate rustty;
use rustty::{Terminal, Cell, Style, Color, Event};
use std::thread::sleep_ms;

fn main() {
    let mut term = Terminal::new().unwrap();
    let dead = Style::with_color(Color::White);
    let alive = Style::with_color(Color::Black);
    loop {
        term.clear_with_styles(alive, alive).unwrap();
        let evt = term.get_event(0).unwrap();
        if evt.is_some() {
            match evt.unwrap() {
                Event::Key('s') => break, //break
                Event::Key(_)   => {},    //do nothing
            }
        }
        //main code
        
        let bufferterm = term.to_vec();
        //******would probably be more efficient to convert to bool vector before getting neighbors
        //and checking status

        //term.move_from(bufferterm, 0, bufferterm.len());

        sleep_ms(10u32);
        term.swap_buffers().unwrap();
    }
}

fn get_neighbors(idx: &(usize, usize), size: &(usize, usize)) -> Vec<(usize, usize)> {
    let mut collected = vec![((idx.0 + 1), idx.1), ((idx.0 - 1), idx.1),
                             (idx.0, (idx.1 + 1)), (idx.0, (idx.1 - 1)),
                             ((idx.0 + 1), (idx.1 + 1)), ((idx.0 - 1), (idx.1 + 1)),
                             ((idx.0 + 1), (idx.1 - 1)), ((idx.0 - 1), (idx.1 - 1))];
    if idx.0 == 0 || idx.0 == size.0 {
        collected = collected.iter().map(|x| ((x.0 % size.0), x.1)).collect();
    }
    if idx.1 == 0 || idx.1 == size.1 {
        collected = collected.iter().map(|x| (x.0, (x.1 % size.1))).collect();
    }
    collected
}

fn is_alive(idx: &(usize, usize), term: &Terminal, livestyle: &Style) -> bool {
    let neighbors = get_neighbors(idx, &term.size());
    let statuses: Vec<bool> = neighbors.iter()
        .map(|i| livestyle == &(term[*i].bg()))
        .collect();
    let live: usize = statuses.iter().fold(0usize, |acc, &item| if item { acc + 1 } else {acc});
    if &(term[*idx]).bg() == livestyle {
        //if cell is already alive 
        if live < 2 || live > 3 {
            return false;
        }
        else {
            return true; 
        }
    }
    else {
        //if cell is dead
        if live == 3 {
            return true;
        }
        else {
            return false;
        }
    }
}

#[cfg(test)] 
mod tests {
    
    #[test]
    fn test_neighbors() {
        assert_eq!(vec![(2,1), (0,1), (1,2), (1,0), (2,2), (0,2), (2,0), (0,0)], super::get_neighbors(&(1,1), &(20,20)));
    }
}
