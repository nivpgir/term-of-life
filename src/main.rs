extern crate termion;

use termion::event::*;
use termion::cursor::{self, DetectCursorPos};
use std::collections::HashMap;
use std::collections::HashSet;
// use termion;
use termion::input::{TermRead, MouseTerminal};
use termion::raw::IntoRawMode;
use std::io::{self, Write};



#[derive(PartialEq, Eq, Hash, Debug)]
pub struct Cell{
    location: (u16,u16),
}

pub struct Colony {
    grid: HashSet<Cell>
}

impl Cell{
    fn neighbours(&self) -> Vec<Cell>{
        let (x,y) = self.location;
        vec![
            (x-1,y-1), (x,y-1), (x+1,y-1),
            (x-1,y),            (x+1,y),
            (x-1,y+1), (x,y+1), (x+1,y+1),]
            .iter()
            .map(|t| Cell{ location: *t })
            .collect()
    }
}


impl Colony {
    fn new() -> Self{
        Self{
            grid: HashSet::new(),
        }
    }
    
    fn neighbour_count(self: &Self) -> HashMap<Cell, i32>{
        let mut ncnt = HashMap::new();
        for cell in self.grid.iter().flat_map(Cell::neighbours){
            *ncnt.entry(cell).or_insert(0) += 1;
        }
        ncnt
    }
    
    fn next_gen(self) -> Self{
        Self{
            grid: self.neighbour_count()
                .into_iter()
                .filter_map(|(cell, cnt)|
                            match (self.grid.contains(&cell), cnt){
                                (true, 2) |
                                (_, 3) => Some(cell),
                                _ => None
                            })
                .collect()
        }
    }
}

fn main() {
    let stdin = termion::async_stdin();
    let mut stdout = MouseTerminal::from(io::stdout().into_raw_mode().unwrap());
    writeln!(stdout,"{}", termion::clear::All).unwrap();

    let mut colony = Colony::new();
    let mut events = stdin.events();
    loop {
        let evt = events.by_ref().next();
        match evt {
            Some(Ok(Event::Key(Key::Char('q')))) => break,
            // this will be changed to any event, not just mouse events
            Some(Ok(evt)) => {
                colony = handle_input(evt, colony.grid);
                ();
            },
            _ => (),
        }
        for (x,y) in colony.grid.into_iter().map(|c|c.location).collect(){
            write!(stdout,
                   "{}{}",
                   cursor::Goto(*x,*y),
                   "x")
                .unwrap();
        }
        
        let (x,y) = termion::terminal_size().unwrap();
        write!(stdout,"{}",cursor::Goto(x,y)).unwrap();
        stdout.flush().unwrap();            
        std::thread::sleep(std::time::Duration::new(0,500));

    }
    
}

fn handle_input(e: Event, mut colony: HashSet<Cell>) -> HashSet<Cell>{
    match e {
        Event::Mouse(me) => {
            match me {
                MouseEvent::Press(_, a, b) |
                MouseEvent::Release(a, b) |
                MouseEvent::Hold(a, b) => {
                    colony.insert(Cell{location:(a,b)}); colony
                }
            }
        },
        _ => colony,
    }
}


fn draw_screen(cells: &HashMap<(&u16,&u16), Cell>) {
    let mut stdout = MouseTerminal::from(io::stdout().into_raw_mode().unwrap());
    for ((x,y), cell) in cells{
        let cell_str = match cell.state {
            State::Alive => "x",
            State::Dead => ""
        };
        write!(stdout,
               "{}{}",
               cursor::Goto(**x,**y),
               cell_str)
            .unwrap();
    }
    cursor::Goto(1,1);
    stdout.flush().unwrap();            
}