use termion::event::{Key, Event, MouseEvent};
use termion::input::{TermRead, MouseTerminal};
use termion::raw::IntoRawMode;
use std::io::{Write, stdout, stdin};

const WORDS: &str = include_str!("../data/words_alpha.txt");

fn main() {
    let stdin = stdin();
    let mut stdout = MouseTerminal::from(stdout().into_raw_mode().unwrap());

    let mut query = String::new();

    write!(stdout, "{}", termion::clear::All).unwrap();
    stdout.flush().unwrap();
    for c in stdin.events() {
        let evt = c.unwrap();
        match evt {
            Event::Mouse(_) | Event::Unsupported(_) => {}
            Event::Key(key) => {
                match key {
                    Key::Esc => break,
                    Key::Char(ch) => {
                        query.push(ch);
                    }
                    Key::Backspace => {
                        query.pop();
                    }
                    _ => {}
                };
                write!(stdout, "{}", termion::clear::All).unwrap();
                let mut i = 0;
                for line in WORDS.lines() {
                    if line.contains(&query) {
                        write!(stdout, "{}{}", termion::cursor::Goto(1, 2 + i), line).unwrap();
                        i += 1;
                        if i > 8 {
                            break;
                        }
                    }
                }
                write!(stdout, "{}{}", termion::cursor::Goto(1, 1), query).unwrap();
            }
        }
        stdout.flush().unwrap();
    }
}
