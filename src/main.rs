mod grade;
mod word;
mod group;
mod wordle;

use crate::grade::Grade;
use std::error::Error;
use std::fmt::Display;
use std::io::BufRead;
use std::time::Duration;
use rayon::prelude::*;
use crate::word::{AllWords, Word};

fn perf_test() {
    // let words: AllWords = AllWords::load_from_file("assets/all_wordle_words".as_ref()).unwrap();
    let words: AllWords = AllWords::load_from_file("assets/wordle_answer_words".as_ref()).unwrap();
    let mut perf_times: Vec<Duration> = Vec::new();
    for _ in 0..2 {
        let perf_start = std::time::Instant::now();
        let _ = words.get_best_guess();
        perf_times.push(perf_start.elapsed());
        // println!("Time: {:?}", perf_start.elapsed());
    }
    let average_perf_time: Duration = perf_times.par_iter().sum::<Duration>() / perf_times.len() as u32;
    let per_word_average: Duration = average_perf_time / words.get_length() as u32;
    println!(
        "\nAverage time: {:?} Per word average: {:?} \n{:?}",
        average_perf_time,
        per_word_average,
        perf_times
    );
}

fn main() {
    let mut words: AllWords = AllWords::load_from_file("assets/all_wordle_words".as_ref()).unwrap();
    let mut best_guess = Word::new("".to_string());

    // perf_test();
    // return;

    for _ in 0..6 {
        println!("What is your guess?");
        let mut guess: String = String::new();
        std::io::stdin().read_line(&mut guess).unwrap();
        let mut guess: Word = Word::new(guess.trim().to_owned());
        if guess.len() == 0 {
            guess = best_guess;
        } else if guess.len() != 5 {
            println!("Guess must be 5 letters long");
            continue;
        }
        let wordles = words.grade_all(&guess);
        let groups = wordles.group_by_grade();

        println!("What is the grade?");
        let mut grade = String::new();
        std::io::stdin().read_line(&mut grade).unwrap();
        let grade = Grade::new_from_ternary(grade.trim());
        words = groups.get_group_from_grade(grade).get_words().clone();

        if words.get_length() == 1 {
            println!("The word is {}", words.get_best_guess());
            break;
        } else {
            best_guess = words.get_best_guess();
            println!("Best guess: {}, with {} words left", best_guess, words.get_length());
        }
    }
}

