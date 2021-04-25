use csafe::*;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use structopt::StructOpt;

/// CSafe
#[derive(StructOpt, Debug)]
#[structopt(name = "csafe")]
struct Opt {
    /// Give verbose output
    #[structopt(short = "v", long = "verbose")]
    verbose: bool,

    /// Print new, compound-safe list to file
    #[structopt(short = "o", long = "output")]
    output_path: Option<String>,

    /// Word list to make compound-safe
    #[structopt(name = "input word list", parse(from_os_str))]
    input_path: PathBuf,
}

fn main() {
    let opt = Opt::from_args();
    // let args: Vec<String> = env::args().collect();
    // let word_list_to_check_filename = &args[1];
    let output_dest = match opt.output_path {
        Some(file_path) => file_path,
        None => format!("{:?}.csafe", &opt.input_path.to_owned().into_os_string()),
    };

    let inputted_list = make_set_from_file(&opt.input_path);
    let unsafe_words: Vec<Contenders> = find_unsafe_words(&inputted_list, opt.verbose);
    // unsafe_words is vector of vectors of words, one of which needs to be removed.
    let words_to_remove = find_fewest_words_to_remove(unsafe_words);
    println!("Found fewest words to remove as {:?}", words_to_remove);

    let safe_list = make_clean_list(words_to_remove, &inputted_list);

    let mut f = File::create(&output_dest).expect("Unable to create file");
    for i in &safe_list {
        writeln!(f, "{}", i).expect("Unable to write data to file");
    }

    println!("\n------------------------\n");
    let original_list_length = &inputted_list.len();
    let clean_list_length = safe_list.len();
    println!(
        "The word list you inputted had {} words ({} bits per word).",
        original_list_length,
        log_base(2, *original_list_length as f64)
    );
    println!();
    if clean_list_length == *original_list_length {
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
