#![allow(dead_code)]
use std::cmp::min;
use std::collections::HashMap;
use std::fs::File;
use std::io::Result;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;
use std::time::{Duration, Instant};
use text_io::read;
use progress_bar::*;

#[derive(Debug)]
struct GroupStats {
    group_count: usize,
    longest: usize,
    average: f32
}

fn load_words_from_file(file: impl AsRef<Path>) -> Result<Vec<String>> {
    BufReader::new(File::open(file)?).lines().collect()
}

fn word_to_int_array(word: &String) -> Vec<u32> {
    word.chars().map(|char| char as u32).collect()
}

fn ternary_to_grade(ternary: String) -> usize {
    let mut result: usize = 0;
    let base: usize = 3;
    for (i, j) in ternary.chars().enumerate() {
        result += base.pow(i as u32) * j.to_digit(10).unwrap() as usize;
    }
    result
}
// fn grade_word(guess: &Vec<u32>, word: &Vec<u32>) -> Result<u32> {
//     let mut clue: u32 = 0;
//     let base: u32 = 3;
//     for (i, j) in guess.iter().enumerate() {
//         if j == &word[i] {
//             clue = clue + 2 * base.pow((guess.len() - 1 - i) as u32);
//         } else if word.iter().any(|char| char == j) {
//             for (k, l) in word.iter().enumerate() {
//                 if l == j {
//                     if &guess[k] != l {
//                         clue = clue + base.pow((guess.len() - 1 - i) as u32);
//                     }
//                 }
//             }
//         }
//     }
//     Ok(clue)
// }

fn grade_word(guess: &Vec<u32>, word: &Vec<u32>) -> Result<u32> {
    let mut clue: u32 = 0;
    let base: u32 = 3;
    for (i, j) in guess.iter().enumerate() {
        if j == &word[i] {
            clue = clue + 2 * base.pow(i as u32);
        } else if word.iter().any(|char| char == j) {
            for (k, l) in word.iter().enumerate() {
                if l == j {
                    if &guess[k] != l {
                        clue = clue + base.pow(i as u32);
                    }
                }
            }
        }
    }
    Ok(clue)
}

fn get_grades(word: String, words: &Vec<String>) -> Result<Vec<u32>> {
    let word_array: Vec<u32> = word_to_int_array(&word);
    let words_array: Vec<Vec<u32>> = words
        .iter()
        .map(|i| word_to_int_array(i))
        .collect();
    words_array
        .iter()
        .map(|i| grade_word(&word_array, i))
        .collect()
}

fn get_emojis(grades: Vec<u32>) -> Result<Vec<String>> {
    grades
        .iter()
        .map(|i| grade_to_emojis(*i))
        .collect()
}

fn grade_to_emojis(grade: u32) -> Result<String> {
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

fn group_grades(grades: Vec<u32>) -> Result<HashMap<u8, Vec<usize>>> {
    let mut grouped_grades: HashMap<u8, Vec<usize>> = HashMap::new();
    for (i, j) in grades.iter().enumerate() {
        grouped_grades
            .entry(*j as u8)
            .and_modify(|e| {e.push(i)})
            .or_insert(vec![i]);
    }
    Ok(grouped_grades)
}

fn readable_single_group(grade: &u8, values: Vec<usize>, words: &Vec<String>) -> Result<(String, Vec<String>)> {
    Ok((
        grade_to_emojis(*grade as u32).unwrap(), 
        values.iter().map(|i| words[*i].clone()).collect::<Vec<String>>() 
    ))
}

fn readable_groups(groups: HashMap<u8, Vec<usize>>, words: &Vec<String>) -> Result<HashMap<String, Vec<String>>> {
    let mut readable_groups: HashMap<String, Vec<String>> = HashMap::new();
    for (i, j) in groups.iter() {
        readable_groups
            .entry(grade_to_emojis(*i as u32).unwrap())
            .or_insert(j.iter().map(|k| words[*k].clone()).collect::<Vec<String>>());
    }
    Ok(readable_groups)
}

fn group_stats(groups: &HashMap<u8, Vec<usize>>) -> Result<GroupStats> {
    let mut sum: usize = 0;
    let longest: usize = groups.values().map(|i| {
        let length = i.len();
        sum += length;
        length
    }).max().unwrap();
    // let longest: usize = groups.values().map(|i| i.len()).max().unwrap();
    // let sum: usize = groups.values().map(|i| i.len()).sum();
    Ok(GroupStats {
        group_count: groups.len(), 
        longest: longest, 
        average: (sum as f32 / groups.len() as f32)
    })
}

// TODO: the indexes returned are in terms of the wword pool not absolute in terms of all words
// Im not sure what the best approach is but this should probably be redone when i have the patience
fn group_and_grade(guess: String, words: &Vec<String>) -> Result<HashMap<u8, Vec<usize>>> {
    group_grades(get_grades(guess, words).unwrap())
}

fn print_groups(groups: &HashMap<u8, Vec<usize>>, words: &Vec<String>) {
    let readable_groups: HashMap<String, Vec<String>> = readable_groups(
        groups.clone(), 
        words
    ).unwrap();
    let group_stats: GroupStats = group_stats(&groups).unwrap();
    for i in readable_groups.iter() {
        println!(
            "{}: {:?}", 
            i.0, 
            i.1.split_at(min(10, i.1.len())).0
        );
    }
    println!("{:?}", group_stats);
}

fn print_single_group(grade: &u8, group: &Vec<usize>, words: &Vec<String>) {
    let readable_group: (String, Vec<String>) = readable_single_group(
        grade, 
        group.clone(), 
        words
    ).unwrap();
    println!(
        "{}: {:?}", 
        readable_group.0, 
        readable_group.1
    );
}

fn find_best_guess(guess_pool: &Vec<usize>, words: &Vec<String>) -> Result<(usize, f32)> {
    let guess_pool_words: Vec<String> = guess_pool.iter().map(|i| words[*i].clone()).collect();
    let mut best_guess: usize = 0;
    let mut smallest_avg: f32 = words.len() as f32;
    // 50 was chosen arbitrarily as the threashold where we always want guesses that are possible answers
    if guess_pool.len() >= 50 {
        init_progress_bar(words.len());
        set_progress_bar_action("Calculating", Color::White, Style::Normal);
        for (i, j) in words.iter().enumerate() {
            let i_groups: HashMap<u8, Vec<usize>> = group_grades(get_grades(j.clone(), &guess_pool_words).unwrap()).unwrap();
            let stats: GroupStats = group_stats(&i_groups).unwrap();
            if stats.average <= smallest_avg { 
                best_guess = i;
                smallest_avg = stats.average;
            }
            inc_progress_bar();
        }
    } else {
        init_progress_bar(guess_pool.len());
        set_progress_bar_action("Calculating", Color::White, Style::Normal);
        for (i, j) in guess_pool_words.iter().enumerate() {
            let i_groups: HashMap<u8, Vec<usize>> = group_grades(get_grades(j.clone(), &guess_pool_words).unwrap()).unwrap();
            let stats: GroupStats = group_stats(&i_groups).unwrap();
            if stats.average <= smallest_avg { 
                best_guess = guess_pool[i];
                smallest_avg = stats.average;
            }
            inc_progress_bar();
        } 
    }
    finalize_progress_bar();
    Ok((best_guess, smallest_avg))
}
fn perf_test(words: Vec<String>) {
    let guess_pool: Vec<usize> = (0..words.len()).collect();
    let mut perf_times: Vec<Duration> = Vec::new();
    for _ in 0..5 {
        let perf_start: Instant = Instant::now();
        let _ = find_best_guess(&guess_pool, &words);
        perf_times.push(perf_start.elapsed());
    }
    println!("Average time: {:?}", perf_times.iter().sum::<Duration>() / perf_times.len() as u32);
}

fn main() {
    let all_words: Vec<String> = load_words_from_file("assets/wordle_answer_words").unwrap();

    // perf_test(all_words.clone());
    
    println!("What is your first guess?");
    let first_guess: String = read!();
    println!("What grade did {} get?", first_guess);
    let first_grade: usize = ternary_to_grade(read!());

    let first_groups: HashMap<u8, Vec<usize>> = group_and_grade(
        first_guess.to_lowercase(), 
        &all_words
    ).unwrap();
    let first_single_group: &Vec<usize> = first_groups.get(&(first_grade as u8)).unwrap();
    let best_second_guess: (usize, f32) = find_best_guess(first_single_group, &all_words).unwrap();

    print_single_group(&(first_grade as u8), &first_single_group, &all_words);
    println!(
        "Your best second guess is \"{}\" with an average group size of {}",
        all_words[best_second_guess.0],
        best_second_guess.1
    );

    println!("What is your second guess?");
    let second_guess: String = read!();
    
    let second_words: Vec<String> = first_single_group.iter().map(|i| all_words[*i].clone()).collect();
    let second_groups: HashMap<u8, Vec<usize>> = group_and_grade(
        second_guess.to_lowercase(), 
        &second_words
    ).unwrap();
    print_groups(&second_groups, &second_words);

    println!("What grade did {} get?", second_guess);
    let second_grade: usize = ternary_to_grade(read!());

    let second_single_group: &Vec<usize> = second_groups.get(&(second_grade as u8)).unwrap();
    let best_third_guess: (usize, f32) = find_best_guess(second_single_group, &second_words).unwrap();
    print_single_group(&(second_grade as u8), &second_single_group, &second_words);
    println!(
        "Your best second guess is \"{}\" with an average group size of {}",
        second_words[best_third_guess.0],
        best_third_guess.1
    );
}
