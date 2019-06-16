extern crate termion;

use std::sync::mpsc;
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
    
    fn next_gen(&self) -> Self{
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
    let stdin = std::io::stdin();
    let mut stdout = MouseTerminal::from(io::stdout().into_raw_mode().unwrap());
    writeln!(stdout,"{}", termion::clear::All).unwrap();

    // setup a thread to handle events
    let (tx, rx) = mpsc::channel();
    std::thread::spawn(move ||{
        for evt in stdin.events(){
            match evt {
                Ok(Event::Key(Key::Char('q'))) => break,
                // any other event:
                Ok(evt) => {
                    handle_input(evt, &tx);
                },
                _ => (),
            }
        }
    });
    // setup initial colony
    let glider: Vec<(u16,u16)> = vec![
        (1,0),
        (2,1),
        (0,2),  (1,2),  (2,2)]
        .into_iter()
        .map(|(x,y)| (x+15,y+20)).collect();
    let colony = Colony{
        grid: glider.into_iter().map(|c|Cell{location: c}).collect()
    };
    loop{
        let mut colony = colony.next_gen();
        let evts = rx.try_recv();
        match evts {
            Err(mpsc::TryRecvError::Empty) => (),
            Err(mpsc::TryRecvError::Disconnected) => break,
            Ok(cell) => {colony.grid.insert(cell);},
            
        }
        redraw_screen(&colony);
        let (x,y) = termion::terminal_size().unwrap();
        write!(stdout,"{}",cursor::Goto(x,y)).unwrap();
        stdout.flush().unwrap();            
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
    
}

fn handle_input(e: Event, tx: &std::sync::mpsc::Sender<Cell>){
    // Colony{
    // grid:
    match e {
        Event::Key(Key::Char('q')) => drop(tx),
        Event::Mouse(me) => {
            match me {
                MouseEvent::Press(_, a, b) |
                MouseEvent::Release(a, b) |
                MouseEvent::Hold(a, b) => {
                    tx.send(Cell{location: (a,b)});
                    // colony.grid.insert(Cell{location:(a,b)}); colony.grid
                }
            }
        },
        _ => (),
        // }
        // }
    }
}

fn redraw_screen(colony: &Colony) {
    // let mut stdout = MouseTerminal::from(io::stdout().into_raw_mode().unwrap());
    let mut stdout = io::stdout().into_raw_mode().unwrap();
    write!(stdout,"{}", termion::clear::All).unwrap();
    for (x,y) in colony.grid.iter().map(|c|c.location){
        write!(stdout,
               "{}{}",
               cursor::Goto(x,y),
               "x")
            .unwrap();
    }
    cursor::Goto(1,1);
    stdout.flush().unwrap();            
}
