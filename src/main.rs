extern crate termion;

use termion::event::*;
use termion::cursor::{self, DetectCursorPos};
use std::collections::HashMap;
// use termion;
use termion::input::{TermRead, MouseTerminal};
use termion::raw::IntoRawMode;
use std::io::{self, Write};

#[derive(PartialEq, Eq, Hash, Debug)]
pub enum State {
    Dead,
    Alive,
}


#[derive(PartialEq, Eq, Hash, Debug)]
pub struct Cell{
    state: State,
}

fn main() {
    let stdin = termion::async_stdin();
    let mut stdout = MouseTerminal::from(io::stdout().into_raw_mode().unwrap());
    writeln!(stdout,"{}", termion::clear::All).unwrap();

    let mut cells: HashMap<(u16,u16), Cell> = HashMap::new();
    let mut events = stdin.events();
    loop {
        let evt = events.by_ref().next();
        match evt {
            Some(Ok(Event::Key(Key::Char('q')))) => break,
            // this will be changed to any event, not just mouse events
            Some(Ok(evt)) => {
                cells = handle_input(evt, cells);
                ();
            },
            _ => (),
        }
        for ((x,y), cell) in &cells{
            let cell_str = match cell.state {
                State::Alive => "x",
                State::Dead => ""
            };
            write!(stdout,
                   "{}{}",
                   cursor::Goto(*x,*y),
                   cell_str)
                .unwrap();
        }
        let (x,y) = termion::terminal_size().unwrap();
        write!(stdout,"{}",cursor::Goto(x,y)).unwrap();
        stdout.flush().unwrap();            
        std::thread::sleep(std::time::Duration::new(0,500));

    }
    
}

fn handle_input(e: Event, mut cells: HashMap<(u16,u16), Cell>) -> HashMap<(u16,u16), Cell>{
    match e {
        Event::Mouse(me) => {
            match me {
                MouseEvent::Press(_, a, b) |
                MouseEvent::Release(a, b) |
                MouseEvent::Hold(a, b) => {
                    cells.insert((a, b), Cell{state: State::Alive}); cells
                }
            }
        },
        _ => cells,
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
