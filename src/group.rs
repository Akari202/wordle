use std::fmt::{Display, Formatter};
use rayon::prelude::*;
use crate::grade::Grade;
use crate::word::{AllWords, Word};

#[derive(Clone)]
pub struct Group {
    grade: Grade,
    words: AllWords,
    length: usize
}

#[derive(Clone)]
pub struct AllGroups {
    groups: Vec<Group>
}

impl Group {
    pub fn new(grade: Grade, words: AllWords) -> Self {
        let length = words.get_length();
        Group { grade, words, length }
    }

    pub fn get_grade(&self) -> &Grade {
        &self.grade
    }

    pub fn push_word(&mut self, word: Word) {
        self.words.push_word(word);
        self.length += 1;
    }

    pub fn get_words(&self) -> &AllWords {
        &self.words
    }
}

impl AllGroups {
    pub fn new(groups: Vec<Group>) -> Self {
        AllGroups { groups }
    }

    pub fn get_length(&self) -> usize {
        self.groups.len()
    }

    pub fn get_average_length(&self) -> f32 {
        let mut sum = 0;
        for group in &self.groups {
            sum += group.length;
        }
        sum as f32 / self.groups.len() as f32
    }

    pub fn get_longest_group(&self) -> &Group {
        self.groups.par_iter().max_by_key(|group| group.length).unwrap()
    }

    pub fn get_group_from_grade(&self, grade: Grade) -> &Group {
        self.groups.par_iter().find_first(|group| group.grade.get_grade() == grade.get_grade()).unwrap()
    }
}

impl Display for Group {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} [{}]: {}", self.grade, self.length, self.words)
    }
}

impl Display for AllGroups {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for group in &self.groups {
            write!(f, "{}\n", group)?;
        }
        Ok(())
    }
}
