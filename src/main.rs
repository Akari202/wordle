#![allow(unused)]

use std::fs;
use std::io;
use std::fs::File;
use std::io::{BufRead, BufWriter};
use std::io::BufReader;
use std::path::Path;
use std::ptr::null;
use clap::Parser;
use anyhow::Context;
use anyhow::Result;
use env_logger::Target::Stdout;
use indicatif::ProgressBar;
use serde::Serialize;
use serde::Deserialize;

#[macro_use]
extern crate log;
extern crate core;

/// A simple but powerful tool for the online game Wordle
#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(flatten)]
    verbose: clap_verbosity_flag::Verbosity,
    #[clap(value_parser = valid_guess)]
    guess_one: String,
    /// Clue represented in ternary
    #[clap(value_parser = valid_clue)]
    clue_one: i32
}

fn valid_guess(guess: &str) -> Result<String, String> {
    if guess.chars().count() == 5 as usize {
        Ok(guess.to_lowercase())
    } else {
        Err(format!("Guess must be a 5 letter word"))
    }
}

fn valid_clue(clue_tern: &str) -> Result<i32, String> {
    let clue_dec = tern_to_dec(clue_tern);
    if  clue_dec <= 242 {
        Ok(clue_dec)
    } else {
        Err(format!("Clue must be greater than 0 and at most 22222"))
    }
}

fn tern_to_dec(num_tern: &str) -> i32 {
    let mut num_dec = 0;
    let base: i32 = 3;
    for (place, i) in num_tern.chars().enumerate() {
        num_dec = num_dec + i.to_digit(10).unwrap() as i32 * base.pow(place as u32);
    }
    num_dec
}

fn load_words_from_file(filename: impl AsRef<Path>) -> io::Result<Vec<String>> {
    BufReader::new(File::open(filename)?).lines().collect()
}

fn words_to_int_arrays(words: &Vec<String>) -> Vec<Vec<u32>> {
    words.iter().map(|word| word.chars().map(|char| char as u32).collect::<Vec<u32>>()).collect()
}

fn compare_int_array(guess: &Vec<u32>, word: &Vec<u32>) -> i32 {
    let mut clue: [char; 5] = ['0', '0', '0', '0', '0'];
    for (i, j) in guess.iter().enumerate() {
        if *j == word[i] {
            clue[i] = '2';
        }
    }
    let mut clue_dec = 0;
    let base: i32 = 3;
    for (place, i) in clue.iter().enumerate() {
        clue_dec = clue_dec + i.to_digit(10).unwrap() as i32 * base.pow(place as u32);
    }
    clue_dec
}

fn generate_full_table() -> Vec<Vec<i32>>{
    info!("Cached data not found, precomputing all guesses");
    let all_words = load_words_from_file("assets/all_wordle_words").unwrap();
    let int_array_words = words_to_int_arrays(&all_words);
    let len = all_words.len();
    let word_len = int_array_words[0].len();
    info!("{} words loaded", len);
    info!("This process is highly unoptimized and incredibly slow. \
    It is 1am and brute force iteration is easy");
    let mut pattern_matrix = vec![vec![243; len]; len];
    let progress_bar = ProgressBar::new(len as u64);
    for (i, j) in int_array_words.iter().enumerate() {
        for (k, l) in int_array_words.iter().enumerate() {
            pattern_matrix[i][k] = compare_int_array(j, l);
        }
        progress_bar.inc(1);
    }
    pattern_matrix
}

fn save_pattern_matrix(pattern_matrix: Vec<Vec<i32>>) -> io::Result<()> {
    fs::write("assets/pattern_matrix.ron", ron::to_string(&pattern_matrix).unwrap())
}

fn main() -> Result<()> {
    let args = Cli::parse();
    env_logger::Builder::new()
        .filter_level(args.verbose.log_level_filter())
        .target(Stdout)
        .init();
    info!("starting up");
    info!("First guess: {}", args.guess_one);
    info!("First clue: {}", args.clue_one);
    let pattern_matrix = generate_full_table();
    save_pattern_matrix(pattern_matrix);
    return Ok(());
}