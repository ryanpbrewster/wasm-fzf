use fst::IntoStreamer;
use fst::{automaton::Subsequence, Streamer};
use std::{
    io::{stdin, stdout, Write},
    time::Instant,
};
use termion::event::{Event, Key};
use termion::input::{MouseTerminal, TermRead};
use termion::{cursor::Goto, raw::IntoRawMode};

const WORDS: &str = include_str!("../data/words_alpha.txt");
const MAX_MATCHES: u16 = 8;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let set = fst::Set::from_iter(WORDS.lines())?;

    let stdin = stdin();
    let mut stdout = MouseTerminal::from(stdout().into_raw_mode()?);

    let mut query = String::new();

    write!(stdout, "{}", termion::clear::All)?;
    stdout.flush()?;
    for evt in stdin.events() {
        match evt? {
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
            }
            _ => {}
        }
        write!(stdout, "{}", termion::clear::All)?;
        let start = Instant::now();
        let mut stream = set.search(Subsequence::new(&query)).into_stream();
        let mut i = 0;
        while let Some(key) = stream.next() {
            write!(stdout, "{}{}", Goto(1, 2 + i), std::str::from_utf8(key)?)?;
            i += 1;
            if i > MAX_MATCHES {
                break;
            }
        }
        let elapsed = start.elapsed();
        write!(stdout, "{}{}us", Goto(1, 3 + i), elapsed.as_micros())?;
        write!(stdout, "{}{}", Goto(1, 1), query)?;
        stdout.flush()?;
    }
    Ok(())
}
