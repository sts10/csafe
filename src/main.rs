use compound_word_checker::*;
use std::env;
use std::fs::File;
use std::io::Write;

fn main() {
    let args: Vec<String> = env::args().collect();
    let word_list_to_check_filename = &args[1];
    let compound_safe_list_output = if args.len() == 3 {
        args[2].to_string()
    } else {
        format!("{}.compound-safe", &word_list_to_check_filename)
    };

    let inputted_list = make_vec_from_file(word_list_to_check_filename);
    let unsafe_words: Vec<Vec<String>> = find_unsafe_words(&inputted_list);
    // unsafe_words is vector of vectors of words, one of which needs to be removed.
    let words_to_remove = find_fewest_words_to_remove(unsafe_words);
    println!("Found fewest words to remove as {:?}", words_to_remove);

    let safe_list = make_clean_list(words_to_remove, inputted_list);

    let mut f = File::create(&compound_safe_list_output).expect("Unable to create file");
    for i in &safe_list {
        writeln!(f, "{}", i).expect("Unable to write data to file");
    }

    println!("\n------------------------\n");
    let original_list_length = make_vec_from_file(word_list_to_check_filename).len();
    let clean_list_length = safe_list.len();
    println!(
        "You're inputted word list had {} words ({} bits per word).",
        original_list_length,
        log_base(2, original_list_length as f64)
    );
    println!();
    if clean_list_length == original_list_length {
        println!("I didn't find any problematic words. Your inputted word list appears to be compound-safe as is!");
    } else {
        println!(
            "The compound-safe list I made has {} words ({} bits per word). It's located at '{}'",
            clean_list_length,
            log_base(2, clean_list_length as f64),
            &compound_safe_list_output
        );
    }
}
