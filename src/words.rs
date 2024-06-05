use std::{fs, io};
use rand::prelude::*;

pub struct Words { pub words: Vec<String> }

impl Words {
    pub fn make(filename: &str) -> Result<Words, io::Error> {
        let file_string: Vec<String> = 
            fs::read_to_string(filename)?.lines().map(|a| a.to_string()).collect();

        Ok(Words { words: file_string })
    }

    pub fn get_rand(&self) -> Result<String, io::Error> {
       Ok(self
           .words
           .get(rand::thread_rng().gen_range(0..self.words.len()))
           .ok_or(io::Error::new(
                   io::ErrorKind::Other, 
                   "Could not get random word"
            ))?.to_string()
        )
    }
}

#[derive(Hash, Eq, PartialEq)]
pub enum CharState {
   Correct,
   Incorrect,
   Exists,
   None,
}

pub struct WordleChar {
    pub char: char,
    pub state: CharState,
}

impl WordleChar {
    pub fn new(char: char) -> WordleChar {
        WordleChar {
            char,
            state: CharState::None,
        }
    }
}

pub type WordleString = Vec<WordleChar>;
