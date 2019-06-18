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


fn main() {
    let stdin = termion::async_stdin();
    let mut stdout = MouseTerminal::from(io::stdout().into_raw_mode().unwrap());
    writeln!(stdout,"{}", termion::clear::All).unwrap();

    let glider: Vec<(u16,u16)> = vec![
        (1,0),
        (2,1),
        (0,2),  (1,2),  (2,2)]
        .into_iter()
        .map(|(x,y)| (x+15,y+20)).collect();

    let mut colony = Colony{
        grid: glider.into_iter().map(|c|Cell{location: c}).collect(),
        is_paused: true,
    };
    let mut events = stdin.events();
    let mut i = 0;
    loop {
        i += 1;
        while let Some(inp) = events.by_ref().next() {
            if let Ok(evt) = inp {
                if let Some(action) = handle_input(evt){
                    match do_action(action, colony){
                        Some(col) => {colony = col;},
                        None => return
                    }
                }
            } else {
                continue;
            };
        };
        redraw_screen(&mut stdout, &colony);
        // write!(stdout,"{}{}{}",cursor::Goto(1,1), i, colony.is_paused).unwrap();
        if ! colony.is_paused { i += 1; colony = colony.next_gen(); }
        let (x,y) = termion::terminal_size().unwrap();
        write!(stdout,"{}",cursor::Goto(x,y)).unwrap();
        // write!(stdout,"{}{}",cursor::Goto(1,2), colony.is_paused).unwrap();
        stdout.flush().unwrap();
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}

pub enum Action{
    Step,
    Pause,
    Continue,
    Quit,
    Restart,
    Put(u16,u16),
    Remove(u16,u16)
}

fn do_action(a: Action, mut col: Colony) -> Option<Colony> {
    match a {
        Action::Step =>Some(Colony{
            is_paused: true,
            grid: col.next_gen().grid
        }),
        Action::Pause => Some(Colony{
            is_paused: true,
            ..col
        }),
        Action::Continue => Some(Colony{
            is_paused: false,
            ..col
        }),
        Action::Quit => None,
        Action::Restart => Some(Colony{
            grid: HashSet::new(),
            ..col
        }),
        Action::Put(a,b) => {col.grid.insert(Cell{location: (a,b)}); Some(col)},
        Action::Remove(a,b) => {col.grid.remove(&Cell{location: (a,b)}); Some(col)},
    }
}


fn handle_input(e: Event) -> Option<Action>{
    match e {
        Event::Key(Key::Char('q')) => Some(Action::Quit),
        Event::Key(Key::Char('p')) => Some(Action::Pause),
        Event::Key(Key::Char('c')) => Some(Action::Continue),
        Event::Key(Key::Char('r')) => Some(Action::Restart),
        Event::Key(Key::Char('s')) => Some(Action::Step),
        Event::Mouse(me) => {
            match me {
                MouseEvent::Press(button, a, b) => {
                    if button == MouseButton::Right {
                        Some(Action::Remove(a,b))
                    } else {
                        Some(Action::Put(a,b))
                    }
                },
                // MouseEvent::Release(a, b) |
                MouseEvent::Hold(a, b) => Some(Action::Put(a,b)),
                _ => None,
            }
        }
        _ => None
    }
}
#[derive(PartialEq, Eq, Hash, Debug)]
pub struct Cell{
    location: (u16,u16),
}

pub struct Colony {
    grid: HashSet<Cell>,
    is_paused: bool,
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
            is_paused: true,
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
                .collect(),
            ..self
        }
    }
}






fn redraw_screen<W>(screen: &mut W, colony: &Colony) where W: Write{
    // let mut stdout = MouseTerminal::from(io::stdout().into_raw_mode().unwrap());
    let mut stdout = io::stdout().into_raw_mode().unwrap();
    write!(*screen,"{}", termion::clear::All).unwrap();
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
