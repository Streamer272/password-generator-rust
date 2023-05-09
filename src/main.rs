use std::io::Write;

use colored::*;
use rand::Rng;

const QUESTION_LOWERCASE: &str = "Include lowercase letters?";
const QUESTION_UPPERCASE: &str = "Include uppercase letters?";
const QUESTION_NUMBERS: &str = "Include numbers?";
const QUESTION_SPECIAL: &str = "Include special?";
const HINT_YES: &str = "[Y/n]";
const HINT_NO: &str = "[y/N]";
const ANSWER_YES: &str = "Yes";
const ANSWER_NO: &str = "No";

fn ask(out: &mut bool, question: &str, default: bool) {
    print!("{} {} ", question, if default { HINT_YES } else { HINT_NO });
    std::io::stdout().flush().expect("to flush");
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).expect("a line");

    let first = line.as_bytes()[0] as char;
    *out = match first {
        '\n' => default,
        'Y' | 'y' => true,
        'N' | 'n' => false,
        _ => panic!("wrong input"),
    };

    println!(
        "\r{} {}",
        question,
        if *out {
            ANSWER_YES.green()
        } else {
            ANSWER_NO.red()
        }
    );
}

fn main() {
    let length = 16;
    let mut generated = String::new();
    let mut include_lowercase = true;
    let mut include_uppercase = true;
    let mut include_numbers = true;
    let mut include_special = true;

    ask(&mut include_lowercase, QUESTION_LOWERCASE, true);
    ask(&mut include_uppercase, QUESTION_UPPERCASE, true);
    ask(&mut include_numbers, QUESTION_NUMBERS, true);
    ask(&mut include_special, QUESTION_SPECIAL, false);

    let mut characters = Vec::<char>::new();
    if include_lowercase {
        ('a'..='z').for_each(|char| characters.push(char));
    }
    if include_uppercase {
        ('A'..='Z').for_each(|char| characters.push(char));
    }
    if include_numbers {
        ('0'..='9').for_each(|char| characters.push(char));
    }
    if include_special {
        ('!'..='/').for_each(|char| characters.push(char));
    }

    let mut thread = rand::thread_rng();
    for _ in 0..length {
        let random = thread.gen_range(0..characters.len());
        generated.push(characters[random]);
    }

    println!("{}", generated);
}
