use fxhash::FxHashSet;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;

#[derive(Default, Debug, PartialEq)]
pub struct Ambiguity {
    pub root_word: String,
    pub second_word: String,
    pub head: String,
    pub tail: String,
}

pub fn find_unsafe_ambiguities(list: &FxHashSet<String>, verbose: bool) -> Vec<Ambiguity> {
    let mut unsafe_ambiguities: Vec<Ambiguity> = vec![];
    let mut count = 0;
    let mut mashed_word = String::new();
    for root_word in list {
        count += 1;
        if verbose {
            println!("Checking {} (word {} of {})", root_word, count, list.len());
        }
        let root_word_length = root_word.len();
        for second_word in list {
            mashed_word.clear();
            mashed_word.push_str(root_word);
            mashed_word.push_str(second_word);
            for i in 0..mashed_word.len() {
                if i == root_word_length {
                    continue;
                }
                if i == 0 && list.contains(&mashed_word) {
                    if verbose {
                        println!("Found a mashed whole word on list: {}", mashed_word);
                    }
                    unsafe_ambiguities.push(Ambiguity {
                        root_word: root_word.to_string(),
                        second_word: second_word.to_string(),
                        head: mashed_word.to_string(),
                        ..Default::default()
                    });
                    // I don't know if I can break here or I need to keep checking
                    // this mashed_word for additional ambiguities...
                }
                // This check is to deal with strange characters like accented vowels
                let head = match mashed_word.get(0..i) {
                    Some(head) => head,
                    None => continue,
                };
                let tail = match mashed_word.get(i..mashed_word.len()) {
                    Some(tail) => tail,
                    None => continue,
                };
                if list.contains(head) && list.contains(tail) {
                    let ambiguity_for_removal = Ambiguity {
                        root_word: root_word.to_string(),
                        second_word: second_word.to_string(),
                        head: head.to_string(),
                        tail: tail.to_string(),
                    };
                    if verbose {
                        println!("Adding ambiguity {:?}", ambiguity_for_removal);
                    }
                    unsafe_ambiguities.push(ambiguity_for_removal);
                }
            }
        }
    }
    unsafe_ambiguities
}

use std::collections::HashMap;
pub fn find_fewest_words_to_remove(unsafe_ambiguities: Vec<Ambiguity>) -> FxHashSet<String> {
    // First make a hashmap of appearance counts of all unsafe words
    let mut flat_vec = vec![];
    for ambiguity in &unsafe_ambiguities {
        flat_vec.push(ambiguity.root_word.to_string());
        flat_vec.push(ambiguity.second_word.to_string());
        flat_vec.push(ambiguity.head.to_string());
        if !ambiguity.tail.is_empty() {
            flat_vec.push(ambiguity.tail.to_string());
        }
    }

    let mut counts_hashmap: HashMap<String, usize> = HashMap::new();
    for word in &flat_vec {
        counts_hashmap
            .entry(word.to_string())
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }

    let mut words_to_remove = FxHashSet::default();
    'outer: for ambiguity in &unsafe_ambiguities {
        let ambiguity_as_vec = if ambiguity.tail.is_empty() {
            vec![
                ambiguity.root_word.to_string(),
                ambiguity.second_word.to_string(),
                ambiguity.head.to_string(),
            ]
        } else {
            vec![
                ambiguity.root_word.to_string(),
                ambiguity.second_word.to_string(),
                ambiguity.head.to_string(),
                ambiguity.tail.to_string(),
            ]
        };
        // First, check if any ambiguities are already in the words_to_remove
        for word in &ambiguity_as_vec {
            if words_to_remove.contains(word) {
                continue 'outer;
            }
        }
        // if not, look for high-scoring word of ambiguity for removal
        let mut current_highest_score = 0;
        let mut word_to_remove: String = ambiguity_as_vec[0].to_string();
        for word in ambiguity_as_vec {
            if counts_hashmap[&word] > current_highest_score {
                current_highest_score = counts_hashmap[&word];
                word_to_remove = word.to_string();
            }
        }
        words_to_remove.insert(word_to_remove);
    }
    words_to_remove
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

pub fn make_set_from_file(filename: &Path) -> FxHashSet<String> {
    let f = File::open(filename).unwrap();
    let file = BufReader::new(&f);
    file.lines()
        .collect::<Result<FxHashSet<_>, _>>()
        .expect("unable to read word list")
}

pub fn make_clean_list(
    words_to_remove: FxHashSet<String>,
    original_list: &FxHashSet<String>,
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
