use crate::words::{Words, WordleString, WordleChar};
use crate::table::InputTable;

pub mod words;
pub mod table;

fn main() {
    let word: WordleString = Words::make("word_list")
        .expect("Could not read word list")
        .get_rand()
        .expect("Could not get random number")
        .chars()
        .map(WordleChar::new)
        .collect();

    let mut input_table = InputTable::new(6);

    input_table.push_line(word).unwrap();
    input_table.print().unwrap();
}
