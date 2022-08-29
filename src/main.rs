#![allow(unused)]

use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;
use log::info;
use anyhow::Context;
use anyhow::Result;
use indicatif::ProgressBar;
use clap::Parser;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(flatten)]
    verbose: clap_verbosity_flag::Verbosity,
    #[clap(value_parser = valid_guess)]
    guess_one: String
}

fn valid_guess(guess: &str) -> Result<String, String> {
    if guess.chars().count() == 5 as usize {
        Ok(guess.to_lowercase())
    } else {
        Err(format!("Guess must be 5 letters"))
    }
}

fn load_words_from_file(file: impl AsRef<Path>) -> io::Result<Vec<String>> {
    BufReader::new(File::open(file)?).lines().collect()
}

fn word_to_int_array(word: &String) -> Vec<u32>{
    word.chars().map(|char| char as u32).collect()
}

fn grade_word(guess: &Vec<u32>, word: &Vec<u32>) -> Result<u32, String> {
    let mut clue: u32 = 0;
    let base: u32 = 3;
    for (i, j) in guess.iter().enumerate() {
        if j == &word[i] {
            clue = clue + 2 * base.pow((guess.len() - 1 - i) as u32);
        } else if word.iter().any(|char| char == j) {
            for (k, l) in word.iter().enumerate() {
                if l == j {
                    if &guess[k] != l {
                        clue = clue + base.pow((guess.len() - 1 - i) as u32);
                    }
                }
            }
        }
    }
    Ok(clue)
}

fn get_grades(word: String, words: Vec<String>) -> Result<Vec<u32>, String> {
    let word_array = word_to_int_array(&word);
    let words_array: Vec<Vec<u32>> = words
        .iter()
        .map(|i| word_to_int_array(i))
        .collect();
    words_array
        .iter()
        .map(|i| grade_word(&word_array, i))
        .collect()
}

fn get_emojis(grades: Vec<u32>) -> Result<Vec<String>, String> {
    grades
        .iter()
        .map(|i| grade_to_emojis(*i))
        .collect()
}

fn grade_to_emojis(grade: u32) -> Result<String, String> {
    let mut emojis: String = "".to_string();
    let mut i = grade;
    for _ in 0..5 {
        match i % 3 {
            0 => emojis.push_str("⬛"),
            1 => emojis.push_str("🟨"),
            2 => emojis.push_str("🟩"),
            _ => emojis.push_str("?")
        }
        i = i / 3;
    }
    Ok(emojis)
}

fn main() {
    let args = Cli::parse();
    env_logger::Builder::new()
        .filter_level(args.verbose.log_level_filter())
        .init();
    let first_grade = get_emojis(
        get_grades(
            args.guess_one,
            load_words_from_file("assets/wordle_answer_words").unwrap()
        ).unwrap()
    ).unwrap();
    info!("{:?}", first_grade);
}
