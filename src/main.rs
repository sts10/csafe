use clap::Parser;
use csafe::*;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

/// Checks passphrase word lists for words that can be combined such that they can be guessed in two distinct ways
#[derive(Parser, Debug)]
#[clap(name = "csafe", version)]
struct Args {
    /// Give verbose output
    #[clap(short = 'v', long = "verbose")]
    verbose: bool,

    /// Write new, compound-safe list to specified file
    #[clap(short = 'o', long = "output")]
    output_path: Option<String>,

    /// Write discovered ambiguities to specified file
    #[clap(short = 'a', long = "ambiguities")]
    ambiguities_path: Option<String>,

    /// Filepath of word list to make compound-safe
    #[clap(name = "input word list", required = true)]
    input_path: PathBuf,
}

fn main() {
    let opt = Args::parse();
    let output_dest = match opt.output_path {
        Some(file_path) => file_path,
        None => format!("{}.csafe", &opt.input_path.to_str().unwrap()),
    };
    println!("output_dest is {}", output_dest);

    let inputted_list = make_set_from_file(&opt.input_path);
    let unsafe_ambiguities: Vec<Ambiguity> = find_unsafe_ambiguities(&inputted_list, opt.verbose);

    print_ambiguities_if_has_path(opt.ambiguities_path, &unsafe_ambiguities);

    let words_to_remove = find_fewest_words_to_remove(unsafe_ambiguities);
    if opt.verbose {
        println!("Found fewest words to remove as {:?}", words_to_remove);
    }

    let safe_list = make_clean_list(words_to_remove, &inputted_list);

    let mut f = File::create(&output_dest).expect("Unable to create file");
    for word in &safe_list {
        writeln!(f, "{}", word).expect("Unable to write data to file");
    }

    println!("\n------------------------\n");
    let original_list_length = inputted_list.len();
    let clean_list_length = safe_list.len();
    println!(
        "The word list you inputted had {} words ({} bits per word).\n",
        original_list_length,
        log_base(2, original_list_length as f64)
    );
    if clean_list_length == original_list_length {
        println!("I didn't find any problematic words. Your inputted word list appears to be compound-safe as is!");
    } else {
        println!(
            "The compound-safe list I made has {} words ({} bits per word). It's located at '{}'",
            clean_list_length,
            log_base(2, clean_list_length as f64),
            &output_dest
        );
    }
}

fn print_ambiguities_if_has_path(
    ambiguities_path: Option<String>,
    unsafe_ambiguities: &[Ambiguity],
) {
    if let Some(path) = ambiguities_path {
        let mut f = File::create(path).expect("Unable to create file");
        for ambiguity in unsafe_ambiguities {
            if ambiguity.tail.is_empty() {
                writeln!(
                    f,
                    "{}|{} can combine to make {}",
                    ambiguity.root_word, ambiguity.second_word, ambiguity.head
                )
                .expect("Unable to write data to file");
            } else {
                writeln!(
                    f,
                    "{}|{} can make {}|{}",
                    ambiguity.root_word, ambiguity.second_word, ambiguity.head, ambiguity.tail
                )
                .expect("Unable to write data to file");
            }
        }
    }
}
