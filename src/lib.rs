use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

use fxhash::FxHashSet;

#[derive(Default, Debug, PartialEq)]
pub struct Contenders {
    pub root_word: String,
    pub second_word: String,
    pub head: String,
    pub tail: String,
}

pub fn find_unsafe_words(list: &FxHashSet<String>) -> Vec<Contenders> {
    let mut unsafe_words: Vec<Contenders> = vec![];
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
                    unsafe_words.push(Contenders {
                        root_word: root_word.to_string(),
                        second_word: second_word.to_string(),
                        head: mashed_word.to_string(),
                        ..Default::default()
                    });
                    // I don't know if I can break here or I need to keep checking
                    // this mashed_word... Think it's safe to break
                    break;
                }
                let head = &mashed_word[0..i];
                let tail = &mashed_word[i..mashed_word.len()];
                if (head.trim() != "" && list.contains(head))
                    && (tail.trim() != "" && list.contains(tail))
                {
                    let contenders_for_removal = Contenders {
                        root_word: root_word.to_string(),
                        second_word: second_word.to_string(),
                        head: head.to_string(),
                        tail: tail.to_string(),
                    };
                    println!("Adding contenders {:?}", contenders_for_removal);
                    unsafe_words.push(contenders_for_removal);
                    break;
                }
            }
        }
    }
    unsafe_words
}

use std::collections::{HashMap};
pub fn find_fewest_words_to_remove(unsafe_words: Vec<Contenders>) -> FxHashSet<String> {
    // First make a hashmap of appearance counts of all unsafe words
    let mut flat_vec = vec![];
    for contenders in &unsafe_words {
        flat_vec.push(contenders.root_word.to_string());
        flat_vec.push(contenders.second_word.to_string());
        flat_vec.push(contenders.head.to_string());
        if contenders.tail != "" {
            flat_vec.push(contenders.tail.to_string());
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
    'outer: for removal_contenders in &unsafe_words {
        let removal_contenders_as_vec = if removal_contenders.tail == "" {
            vec![
                removal_contenders.root_word.to_string(),
                removal_contenders.second_word.to_string(),
                removal_contenders.head.to_string(),
            ]
        } else {
            vec![
                removal_contenders.root_word.to_string(),
                removal_contenders.second_word.to_string(),
                removal_contenders.head.to_string(),
                removal_contenders.tail.to_string(),
            ]
        };
        // First, check if any contenders are already in the words_to_remove
        for word in &removal_contenders_as_vec {
            if words_to_remove.contains(word) {
                continue 'outer;
            }
        }
        // if not, look for high-scoring word of the contenders for removal
        let mut current_highest_score = 0;
        let mut word_to_remove: String = removal_contenders_as_vec[0].to_string();
        for word in removal_contenders_as_vec {
            if counts_hashmap[&word] > current_highest_score {
                current_highest_score = counts_hashmap[&word];
                word_to_remove = word.to_string();
            }
        }
        // words_to_remove.insert(word_to_remove.to_string());
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

pub fn make_set_from_file(filename: &str) -> FxHashSet<String> {
    let f = File::open(filename).unwrap();
    let file = BufReader::new(&f);
    file.lines()
        .collect::<Result<FxHashSet<_>, _>>()
        .expect("unable to read word list")
}

pub fn make_clean_list(
    words_to_remove: FxHashSet<String>,
    original_list: FxHashSet<String>,
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
