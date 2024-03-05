use std::fmt::{Display, Formatter};
use crate::grade::Grade;
use crate::group::{AllGroups, Group};
use crate::word::{AllWords, Word};
use rayon::prelude::*;

#[derive(Clone)]
pub struct Wordle {
    word: Word,
    grade: Grade
}

#[derive(Clone)]
pub struct AllWordles {
    wordles: Vec<Wordle>
}

impl Wordle {
    pub fn new(word: &Word, guess: &Word) -> Self {
        let grade = word.get_grade(guess);
        let word = word.clone();
        Wordle { word, grade }
    }
}


impl AllWordles {
    pub fn new(word: Word) -> Self {
        AllWordles { wordles: vec![Wordle::new(&word, &Word::new("".to_owned()))] }
    }

    pub fn new_from_vec(wordles: Vec<Wordle>) -> Self {
        AllWordles { wordles }
    }

    pub fn get_length(&self) -> usize {
        self.wordles.len()
    }

    pub fn group_by_grade(&mut self) -> AllGroups {
        let mut groups: Vec<Group> = Vec::new();
        self.wordles.sort_by_key(|wordle| wordle.grade.get_grade());
        for wordle in &self.wordles {
            let mut found = false;
            for group in &mut groups {
                if group.get_grade() == &wordle.grade {
                    group.push_word(wordle.word.clone());
                    found = true;
                    break;
                }
            }
            if !found {
                groups.push(
                    Group::new(
                        wordle.grade.clone(),
                        AllWords::new(wordle.word.clone())
                    )
                );
            }
        }
        AllGroups::new(groups)
    }
}

impl Display for Wordle {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.word, self.grade)
    }
}

impl Display for AllWordles {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for wordle in &self.wordles {
            write!(f, "{}\n", wordle)?;
        }
        Ok(())
    }
}

