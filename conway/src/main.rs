#![feature(clone_from_slice)]
#![feature(convert)] extern crate rustty;
use rustty::{Terminal, Cell, Style, Color, Event};
use std::thread::sleep_ms;

//need to implement random terminal

fn main() {
    let mut term = Terminal::new().unwrap();
    let dead = Style::with_color(Color::White);
    let alive = Style::with_color(Color::Black);
    loop {
        term.clear_with_styles(alive, alive).unwrap();
        let evt = term.get_event(0).unwrap();
        if evt.is_some() {
            match evt.unwrap() {
                Event::Key('q') => break, //break
                Event::Key(_)   => {},    //do nothing
            }
        }
        //main code
        
        let mut bufferterm: Vec<bool> = term.to_vec()
            .iter()
            .map(|cell| &cell.bg() == &alive)
            .collect();
        let termcopy: Vec<bool> = bufferterm.clone();
        let termsize: (usize, usize) = term.size();
        
        //bufferterm[(term.size().0 * 1) - 1] = false;
        for y in 0..termsize.1 {
            for x in 0..termsize.0 {
                bufferterm[(y * termsize.0) + x] = is_alive(&(x, y), &termcopy, &termsize);
            }
        }

        let newterm: Vec<Cell> = bufferterm.iter()
            .map(|value| if *value { Cell::with_styles(alive, alive)}
                else { Cell::with_styles(dead, dead)})
            .collect();
        term.clone_from_slice(&newterm.as_slice());
        sleep_ms(10u32);
        term.swap_buffers().unwrap();
    }
}

fn get_neighbors(idx: &(usize, usize), size: &(usize, usize)) -> Vec<(usize, usize)> {
    let mut collected = vec![((idx.0 + 1), idx.1), ((idx.0 - 1), idx.1),
                             (idx.0, (idx.1 + 1)), (idx.0, (idx.1 - 1)),
                             ((idx.0 + 1), (idx.1 + 1)), ((idx.0 - 1), (idx.1 + 1)),
                             ((idx.0 + 1), (idx.1 - 1)), ((idx.0 - 1), (idx.1 - 1))];
    if idx.0 == 0 || idx.0 >= size.0 - 1 {
        collected = collected.iter().map(|x| ((x.0 % size.0), x.1)).collect();
    }
    if idx.1 == 0 || idx.1 >= size.1 - 1 {
        collected = collected.iter().map(|x| (x.0, (x.1 % size.1))).collect();
    }
    collected
}

//needs total rewrite
fn is_alive(idx: &(usize, usize), term: &Vec<bool>, size: &(usize, usize)) -> bool {
    let neighbors = get_neighbors(idx, size);
    let statuses: Vec<bool> = neighbors.iter()
        .map(|i| term[(i.1 * size.0) + i.0])
        .collect();
    let live: usize = statuses.iter().fold(0usize, |acc, &item| if item { acc + 1 } else {acc});
    //println!("{}", live);
    //for n in neighbors {
        //println!("{}, {}", n.0, n.1);
    //}
    if (term[(idx.1 * size.0) + idx.0]) {
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

    #[test]
    fn test_life() {
        assert!(!super::is_alive(&(2,2),
        &vec![false, true, false,
        false, false, true,
        false, false, false],
        &(3,3)))
    }
}
