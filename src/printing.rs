use std::fmt::{Display, Formatter};
use crate::{AllGroups, AllWordles, AllWords, Grade, Group, Word, Wordle};

impl Display for Grade {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.get_color_boxes())
    }
}

impl Display for Word {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.word)
    }
}

impl Display for Wordle {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.word, self.grade)
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

impl Display for AllWordles {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for wordle in &self.wordles {
            write!(f, "{}\n", wordle)?;
        }
        Ok(())
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

impl Clone for Word {
    fn clone(&self) -> Self {
        Word { word: self.word.clone() }
    }
}
