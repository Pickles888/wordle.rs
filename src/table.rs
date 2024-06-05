use super::words::{WordleString, CharState::*};
use std::{collections::HashMap, io::stdout};
use crossterm::{
    execute, 
    style::{
        Color,
        Colors,
        Print,
        SetColors,
    },
};

pub struct InputTable {
    pub lines: Vec<WordleString>,
    pub max: usize,
}

impl InputTable {
    pub fn new(max_guesses: usize) -> InputTable {
        InputTable {
            lines: Vec::new(),
            max: max_guesses,
        }
    }

    pub fn push_line(&mut self, line: WordleString) -> Result<(), String> {
        if self.lines.len() < self.max {
            self.lines.push(line);
            Ok(())
        } else {
            Err(String::from("Tried to push above the maximum lines"))
        }
    }

    pub fn print(&self) -> Result<(), std::io::Error> {
        let colors = HashMap::from([
            (Correct, Color::Green),
            (Incorrect, Color::Red),
            (Exists, Color::Yellow),
            (None, Color::Black),
        ]);

        self.lines.iter().for_each(|a| {
            a.iter().for_each(|b| { 
                execute!(
                    stdout(),
                    SetColors(Colors::new(
                        Color::Reset, 
                        *colors.get(&b.state).unwrap_or_else(||
                            panic!("{} contained an invalid state", b.char)
                        )
                    )),
                    Print(b.char),
                ).err();
            });

            println!();
        });

        Ok(())
    }
}
