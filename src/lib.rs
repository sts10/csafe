use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

pub fn find_unsafe_words(list: &HashSet<String>) -> Vec<Vec<String>> {
    let mut unsafe_words: Vec<Vec<String>> = vec![];
    let mut count = 0;
    for root_word in list {
        count += 1;
        println!("Checking {} (word {} of {})", root_word, count, list.len());
        let root_word_length = root_word.len();
        for second_word in list {
            let mashed_word = root_word.to_owned().to_owned() + second_word;
            for i in 0..mashed_word.len() {
                if i == root_word_length {
                    continue;
                }
                if i == 0 && list.contains(&mashed_word) {
                    println!("Found a mashed whole word ");
                    unsafe_words.push(vec![
                        root_word.to_string(),
                        second_word.to_string(),
                        mashed_word.to_string(),
                    ]);
                    // I don't know if I can break here or I need to keep checking
                    // this mashed_word... Think it's safe to break
                    break;
                }
                let first_part = &mashed_word[0..i];
                let second_part = &mashed_word[i..mashed_word.len()];
                // Honestly not sure about these &&s
                if (first_part.trim() != "" && is_on_list(first_part, &list))
                    && (second_part.trim() != "" && is_on_list(second_part, &list))
                {
                    let contenders_for_removal = vec![
                        root_word.to_string(),
                        second_word.to_string(),
                        first_part.to_string(),
                        second_part.to_string(),
                    ];
                    println!("Adding contenders {:?}", contenders_for_removal);
                    unsafe_words.push(contenders_for_removal);
                    break;
                }
            }
        }
    }
    unsafe_words
}

use std::collections::{HashMap, HashSet};
pub fn find_fewest_words_to_remove(unsafe_words: Vec<Vec<String>>) -> HashSet<String> {
    // First make a hashmap of appearance counts of all unsafe words
    let flat_vec = unsafe_words
        .clone() // not great, but gets it to compile
        .into_iter()
        .flatten()
        .collect::<Vec<String>>();

    let mut counts_hashmap: HashMap<String, usize> = HashMap::new();
    for word in &flat_vec {
        counts_hashmap
            .entry(word.to_string())
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }

    let mut words_to_remove = HashSet::new();
    'outer: for removal_contenders in &unsafe_words {
        // First, check if any contenders are already in the words_to_remove
        for word in removal_contenders {
            if words_to_remove.contains(word) {
                continue 'outer;
            }
        }
        // if not, look for high-scoring word of the contenders for removal
        let mut current_highest_score = 0;
        let mut word_to_remove = &removal_contenders[0];
        for word in removal_contenders {
            if counts_hashmap[word] > current_highest_score {
                current_highest_score = counts_hashmap[word];
                word_to_remove = &word;
            }
        }
        words_to_remove.insert(word_to_remove.to_string());
    }
    words_to_remove
}

fn is_on_list(target_word: &str, list: &HashSet<String>) -> bool {
    list.contains(target_word)
}

pub fn make_vec_from_file(filename: &str) -> Vec<String> {
    let mut word_list: Vec<String> = [].to_vec();
    let f = File::open(filename).unwrap();
    let file = BufReader::new(&f);
    for line in file.lines() {
        let l = line.unwrap();
        word_list.push(l);
    }
    word_list
}

pub fn make_set_from_file(filename: &str) -> HashSet<String> {
    let f = File::open(filename).unwrap();
    let file = BufReader::new(&f);
    file.lines()
        .collect::<Result<HashSet<_>, _>>()
        .expect("unable to read word list")
}

pub fn make_clean_list(
    words_to_remove: HashSet<String>,
    original_list: HashSet<String>,
) -> Vec<String> {
    let mut clean_words = original_list
        .difference(&words_to_remove)
        .map(|s| s.to_owned())
        .collect::<Vec<_>>();
    clean_words.sort();
    clean_words
}

pub fn log_base(base: u64, n: f64) -> f64 {
    let base_as_float: f64 = base as f64;
    (n.ln() / base_as_float.ln()) as f64
}
