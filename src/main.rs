use std::{
    io::{stdin, stdout, Write},
    time::Instant,
};
use termion::event::{Event, Key};
use termion::input::{MouseTerminal, TermRead};
use termion::raw::IntoRawMode;

const WORDS: &str = include_str!("../data/words_alpha.txt");
const MAX_MATCHES: u16 = 8;

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
                let start = Instant::now();
                let mut i = 0;
                for line in WORDS.lines() {
                    if line.contains(&query) {
                        write!(stdout, "{}{}", termion::cursor::Goto(1, 2 + i), line).unwrap();
                        i += 1;
                        if i > MAX_MATCHES {
                            break;
                        }
                    }
                }
                write!(
                    stdout,
                    "{}{}us",
                    termion::cursor::Goto(1, 3 + i),
                    start.elapsed().as_micros()
                )
                .unwrap();
                write!(stdout, "{}{}", termion::cursor::Goto(1, 1), query).unwrap();
            }
        }
        stdout.flush().unwrap();
    }
}
