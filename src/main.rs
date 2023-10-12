mod printing;
use std::error::Error;
use std::fmt::Display;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::time::Duration;
use progress_bar::{Color, finalize_progress_bar, inc_progress_bar, init_progress_bar, set_progress_bar_action, Style};
use rayon::prelude::*;

#[derive(Clone, Copy)]
struct Grade {
    grade: u32
}

struct Word {
    word: String
}

#[derive(Clone)]
struct Wordle {
    word: Word,
    grade: Grade
}

#[derive(Clone)]
struct Group {
    grade: Grade,
    words: AllWords,
    length: usize
}

#[derive(Clone)]
struct AllWords {
    words: Vec<Word>
}

#[derive(Clone)]
struct AllWordles {
    wordles: Vec<Wordle>
}

#[derive(Clone)]
struct AllGroups {
    groups: Vec<Group>
}

impl Grade {
    fn new_from_decimal(decimal: u32) -> Self {
        Grade { grade: decimal }
    }

    fn new_from_ternary(ternary: &str) -> Self {
        Grade {
            grade: ternary.chars().rev().enumerate().fold(0, |acc, (i, c)| {
                acc + c.to_digit(10).unwrap() * 3u32.pow(i as u32)
            })
        }
    }

    fn decimal_to_ternary(&self) -> String {
        let mut decimal = self.grade;
        let mut ternary = String::new();

        while decimal > 0 {
            ternary.push_str(&(decimal % 3).to_string());
            decimal /= 3;
        }

        ternary.chars().rev().collect()
    }

    fn get_color_boxes(&self) -> String {
        let mut color_boxes = String::new();
        let mut i: u32 = self.grade;
        for _ in 0..5 {
            match i % 3 {
                0 => color_boxes.push_str("⬛"),
                1 => color_boxes.push_str("🟨"),
                2 => color_boxes.push_str("🟩"),
                _ => color_boxes.push_str("?")
            }
            i = i / 3;
        }
        color_boxes.chars().rev().collect()
    }


}

impl Word {
    fn new(word: String) -> Self {
        Word { word }
    }

    fn get_grade(&self, guess: &str) -> Grade {
        let mut grade = 0;
        let mut yellows: Vec<usize> = Vec::new();
        guess.chars().rev().enumerate().for_each(|(i, c)| {
            if c == self.word.chars().rev().nth(i).unwrap() {
                grade += 2 * 3u32.pow(i as u32);
            } else {
                for (j, d) in self.word.chars().rev().enumerate() {
                    if c == d {
                        if guess.chars().rev().nth(j).unwrap() != d {
                            if !yellows.contains(&j) {
                                grade += 3u32.pow(i as u32);
                                yellows.push(j);
                                break;
                            }
                        }
                    }
                };
            }
        });
        Grade::new_from_decimal(grade)
    }
}

impl Wordle {
    fn new(word: &Word, guess: &str) -> Self {
        let grade = word.get_grade(guess);
        let word = Word::new(word.word.clone());
        Wordle { word, grade }
    }
}

impl Group {
    fn new(grade: Grade, words: AllWords) -> Self {
        let length = words.get_length();
        Group { grade, words, length }
    }
}

impl AllWords {
    fn new(word: Word) -> Self {
        AllWords { words: vec![word] }
    }

    fn load_from_file(path: &Path) -> Result<Self, Box<dyn Error>> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let mut words = Vec::new();
        for line in reader.lines() {
            words.push(Word::new(line?));
        }
        Ok(AllWords { words })
    }

    fn get_length(&self) -> usize {
        self.words.len()
    }

    fn grade_all(&self, guess: &str) -> AllWordles {
        AllWordles {
            wordles: self.words.par_iter().map(|word| Wordle::new(word, guess)).collect()
        }
    }

    fn push(&mut self, word: Word) {
        self.words.push(word);
    }

    fn get_best_guess(&self) -> Word {
        init_progress_bar(self.words.len() as usize);
        set_progress_bar_action("Calculating best guess", Color::White, Style::Normal);
        let word = Word::new(
            self.words.par_iter()
                .map(|word| {
                    inc_progress_bar();
                    (self.grade_all(&word.word).group_by_grade(), word)
                }).min_by_key(|groups|
                groups.0.get_average_length() as u32
            ).unwrap().1.word.clone()
        );
        finalize_progress_bar();
        word
    }
}

impl AllWordles {
    fn get_length(&self) -> usize {
        self.wordles.len()
    }

    fn group_by_grade(&self) -> AllGroups {
        let mut groups: Vec<Group> = Vec::new();
        for wordle in &self.wordles {
            let mut found = false;
            for group in &mut groups {
                if group.grade.grade == wordle.grade.grade {
                    group.words.push(Word::new(wordle.word.word.clone()));
                    group.length += 1;
                    found = true;
                    break;
                }
            }
            if !found {
                groups.push(
                    Group {
                        grade: wordle.grade.clone(),
                        words: AllWords::new(Word::new(wordle.word.word.clone())),
                        length: 1
                    }
                );
            }
        }
        AllGroups { groups }
    }
}

impl AllGroups {
    fn get_length(&self) -> usize {
        self.groups.len()
    }

    fn get_average_length(&self) -> f32 {
        let mut sum = 0;
        for group in &self.groups {
            sum += group.length;
        }
        sum as f32 / self.groups.len() as f32
    }

    fn get_longest_group(&self) -> &Group {
        self.groups.par_iter().max_by_key(|group| group.length).unwrap()
    }

    fn get_group_from_grade(&self, grade: Grade) -> &Group {
        self.groups.par_iter().find_first(|group| group.grade.grade == grade.grade).unwrap()
    }
}

fn perf_test(words: &AllWords) {
    let mut perf_times: Vec<Duration> = Vec::new();
    for _ in 0..5 {
        let perf_start = std::time::Instant::now();
        let best_guess = words.get_best_guess();
        perf_times.push(perf_start.elapsed());
    }
    println!("Average time: {:?}", perf_times.iter().sum::<Duration>() / perf_times.len() as u32);
}

fn main() {
    let mut words: AllWords = AllWords::load_from_file("assets/all_wordle_words".as_ref()).unwrap();
    let mut best_guess = Word::new("".to_string());

    perf_test(&words);

    for _ in 0..6 {
        println!("What is your guess?");
        let mut guess = String::new();
        std::io::stdin().read_line(&mut guess).unwrap();
        let guess = guess.trim();
        if guess.len() != 5 {
            println!("Guess must be 5 letters long");
            continue;
        } else if guess.len() == 0 {
            let guess = best_guess.word.clone();
        }
        let wordles = words.grade_all(guess);
        let groups = wordles.group_by_grade();

        println!("What is the grade?");
        let mut grade = String::new();
        std::io::stdin().read_line(&mut grade).unwrap();
        let grade = Grade::new_from_ternary(grade.trim());
        words = groups.get_group_from_grade(grade).words.clone();

        if words.get_length() == 1 {
            println!("The word is {}", words.get_best_guess());
            break;
        } else {
            best_guess = words.get_best_guess();
            println!("Best guess: {}, with {} words left", best_guess, words.get_length());
        }
    }
}

