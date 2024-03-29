use std::error::Error;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use progress_bar::{Color, finalize_progress_bar, inc_progress_bar, init_progress_bar, set_progress_bar_action, Style};
use rayon::prelude::*;
use crate::grade::Grade;
use crate::wordle::{AllWordles, Wordle};

pub struct Word {
    word: String
}

#[derive(Clone)]
pub struct AllWords {
    pub words: Vec<Word>
}

impl Word {
    pub fn new(word: String) -> Self {
        Word { word }
    }

    pub fn chars(&self) -> std::str::Chars<'_> {
        self.word.chars()
    }

    pub fn par_chars(&self) -> rayon::str::Chars<'_> {
        self.word.par_chars()
    }

    pub fn len(&self) -> usize {
        self.word.len()
    }

    pub fn get_grade(&self, guess: &Word) -> Grade {
        let mut yellows: Vec<usize> = Vec::new();
        let grade = guess.chars().rev().enumerate().fold(0, |acc, (i, c)| {
            if c == self.word.chars().rev().nth(i).unwrap() {
                acc + 2 * 3u32.pow(i as u32)
            } else {
                for (j, d) in self.word.chars().rev().enumerate() {
                    if c == d {
                        if guess.chars().rev().nth(j).unwrap() != d {
                            if !yellows.contains(&j) {
                                yellows.push(j);
                                return acc + 3u32.pow(i as u32);
                            }
                        }
                    }
                };
                acc
            }
        }

        );
        Grade::new_from_decimal(grade)
    }

    // pub fn get_decimal_word(&self) -> u32 {
    //     self.word.chars().rev().enumerate().fold(0, |acc, (i, c)| {
    //         acc + (c.to_digit(36).unwrap() - 9) * 26u32.pow(i as u32)
    //     })
    // }
}

// pub fn get_word_from_decimal(decimal: u32) -> Word {
//     let mut decimal = decimal;
//     let mut word = String::new();
//
//     while decimal > 0 {
//         word.push_str(&char::from_u32(decimal % 27 + 96).unwrap().to_string());
//         // word.push_str(&((decimal % 27) as u8 as char).to_string());
//         decimal /= 27;
//     }
//
//     Word { word: word.chars().rev().collect() }
// }

impl AllWords {
    pub fn new(word: Word) -> Self {
        AllWords { words: vec![word] }
    }

    pub fn new_from_vec(words: Vec<Word>) -> Self {
        AllWords { words }
    }

    pub fn push_word(&mut self, word: Word) {
        self.words.push(word);
    }

    pub fn load_from_file(path: &Path) -> Result<Self, Box<dyn Error>> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let mut words = Vec::new();
        for line in reader.lines() {
            words.push(Word::new(line?));
        }
        Ok(AllWords { words })
    }

    pub fn get_length(&self) -> usize {
        self.words.len()
    }

    pub fn get(&self, index: usize) -> &Word {
        self.words.get(index).unwrap()
    }

    pub fn grade_all(&self, guess: &Word) -> AllWordles {
        AllWordles::new_from_vec(
            self.words.par_iter()
                .map(|word|
                    Wordle::new(word, guess)
                ).collect()
        )
    }

    pub fn push(&mut self, word: Word) {
        self.words.push(word);
    }

    pub fn get_best_guess(&self) -> Word {
        init_progress_bar(self.words.len());
        set_progress_bar_action("Calculating best guess", Color::White, Style::Normal);
        let word = Word::new(
            self.words.par_iter()
                .map(|word| {
                    inc_progress_bar();
                    (self.grade_all(&word).group_by_grade(), word)
                }).min_by_key(|groups|
                groups.0.get_average_length() as u32
            ).unwrap().1.word.clone()
        );
        finalize_progress_bar();
        word
    }
}

impl Clone for Word {
    fn clone(&self) -> Self {
        Word { word: self.word.clone() }
    }
}

impl Display for AllWords {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for word in &self.words {
            write!(f, "{} ", word)?;
        }
        Ok(())
    }
}

impl Display for Word {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.word)
    }
}

impl PartialEq for Word {
    fn eq(&self, other: &Self) -> bool {
        self.word == other.word
    }
}
