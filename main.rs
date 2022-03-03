use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let words = load_words();
    // println!("{:?}, {}", words, words.len());
    let mut filtered_words = words;
    let guesses = [
        // new_guess('q',2,Status::None),
        new_guess('w',2,Status::None),
        new_guess('e',2,Status::None),
        new_guess('r',2,Status::None),
        new_guess('t',2,Status::None),
        new_guess('y',2,Status::None),
        new_guess('u',2,Status::None),
        new_guess('i',2,Status::Match),
        new_guess('o',1,Status::Misplace),
        new_guess('t',4,Status::None),
        new_guess('a',2,Status::None),
        new_guess('s',0,Status::None),
    ];
    for guess in guesses.iter() {
        filtered_words = filter_words(&guess.letter, &guess.index, &guess.status, &filtered_words);
    }
    println!("{:?}, {}", filtered_words, filtered_words.len());
    // let filtered_words_2 = filter_words(&'l',&3,&Status::Misplace,&filtered_words);
    // println!("{:?}, {}", filtered_words_2, filtered_words_2.len());
    // let filtered_words_3 = filter_words(&'a',&0,&Status::None,&words);
    // println!("{:?}, {}", filtered_words_3, filtered_words_3.len());
    // println!("{},{}",words.len(), filtered_words.len() + filtered_words_2.len() + filtered_words_3.len())
}

fn load_words() -> Vec<String> {
    let mut words = vec!();
    match File::open("./words.txt") {
        Ok(file) =>
            for maybe_line in io::BufReader::new(file).lines() {
                if let Ok(line) = maybe_line {
                    words.push(line);
                }
            },
        Err(why) => panic!("Failed because {}", why),
    }
    return words;
}

fn filter_words(letter:&char, index:&usize, status:&Status, words:&Vec<String>) -> Vec<String> {
    return words.iter()
        .filter(|word| match status {
            Status::Match => word.chars().nth(*index).unwrap() == *letter,
            Status::Misplace => word.chars().nth(*index).unwrap() != *letter
                && word.contains(&char::to_string(&letter)),
            Status::None => !word.contains(&char::to_string(&letter))
        }).cloned()
        .collect();
}

// fn does_word_match(guess:&Vec<Guess>, word:&String) -> bool {
//     return true;
// }

struct Guess {
    letter:char,
    index:usize,
    status:Status
}

fn new_guess(letter:char,index:usize,status:Status) -> Guess {
    Guess{
        letter:letter,
        index:index,
        status:status
    }
}

enum Status {
    Match,
    Misplace,
    None
}