#![allow(dead_code)]

use regex::Regex;
use std::{collections::HashMap, io};

// Given a list of integers, use a vector and return the median (when sorted,
// the value in the middle position) and mode (the value that occurs most often; a hash
// map will be helpful here) of the list.
fn median(list: &Vec<i32>) -> f32 {
    let mut copy = list.clone();
    copy.sort();

    let len = copy.len();
    if len % 2 == 0 {
        (copy[(len / 2) - 1] + copy[len / 2]) as f32 / 2.0
    } else {
        copy[len / 2] as f32
    }
}

fn mode(list: &Vec<i32>) -> Vec<i32> {
    if list.is_empty() {
        return vec![];
    }

    let mut counts = HashMap::new();

    for item in list {
        let count = counts.entry(*item).or_insert(0);
        *count += 1;
    }

    let max_count = counts.values().max().copied().unwrap();

    let max_vals = counts
        .iter()
        .filter(|(_, v)| **v == max_count)
        .map(|(k, _)| *k);
    let mut max_vals = Vec::from_iter(max_vals);
    max_vals.sort();

    max_vals
}

// Convert strings to pig latin. The first consonant of each word is moved to the
// end of the word and “ay” is added, so “first” becomes “irst-fay.”
// Words that start with a vowel have “hay” added to the end instead (“apple” becomes “apple-hay”).
// Keep in mind the details about UTF-8 encoding!
fn convert_to_pig_latin(s: &mut String) {
    if s.is_empty() {
        return;
    }

    let (first_index, first_char) = s.char_indices().next().unwrap();
    match first_char {
        'a' | 'e' | 'i' | 'o' | 'u' => s.push_str("-hay"),
        consonant if consonant.is_ascii_alphabetic() => {
            s.remove(first_index);
            s.push_str(&format!("-{}ay", consonant));
        }
        _ => (),
    }
}

// Using a hash map and vectors, create a text interface to allow a user to add
// employee names to a department in a company. For example, “Add Sally to Engineering” or
// “Add Amir to Sales.” Then let the user retrieve a list of all people in a department or
// all people in the company by department, sorted alphabetically.
fn company_app() {
    let mut department_members: HashMap<String, Vec<String>> = HashMap::new();
    let add_re = Regex::new(r"^Add ([[:alpha:]]+) to ([[:alpha:]]+)").unwrap();
    let list_re = Regex::new(r"^List ([[:alpha:]]+)").unwrap();

    loop {
        println!("\nWhat do you want to do?");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Hey don't do that!");
        let input = input.trim();

        if let Some(caps) = add_re.captures(input) {
            if let (Some(name), Some(department)) = (caps.get(1), caps.get(2)) {
                let department = department.as_str().to_string().to_lowercase();
                let name = name.as_str().to_string();

                let members = department_members.entry(department).or_insert(Vec::new());
                members.push(name);
                members.sort();

                continue;
            }
        }

        if let Some(caps) = list_re.captures(input) {
            if let Some(department) = caps.get(1) {
                let department = department.as_str();

                match department_members.get(&department.to_lowercase()) {
                    Some(members) => {
                        println!("Members of {}: {:?}", department, members);
                        continue;
                    }
                    None => break,
                }
            }
        }

        break;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn median_odd_length_array() {
        assert_eq!(0.0, median(&vec![1, 0, -1]));
    }

    #[test]
    fn median_even_length_array() {
        assert_eq!(1.5, median(&vec![1, 2, 0, 3]));
    }

    #[test]
    fn single_mode() {
        assert_eq!(vec![1], mode(&vec![1, 1, 0]));
    }

    #[test]
    fn multiple_modes() {
        assert_eq!(vec![0, 1], mode(&vec![1, 1, 0, 0]));
    }

    #[test]
    fn pig_latin_starts_with_consonant() {
        let mut word = String::from("first");
        convert_to_pig_latin(&mut word);
        assert_eq!(String::from("irst-fay"), word);
    }

    #[test]
    fn pig_latin_starts_with_vowel() {
        let mut word = String::from("apple");
        convert_to_pig_latin(&mut word);
        assert_eq!(String::from("apple-hay"), word);
    }
}
