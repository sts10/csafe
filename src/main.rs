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

    /// Write new, compound-safe list to specified file
    #[structopt(short = "o", long = "output")]
    output_path: Option<String>,

    /// Write discovered compound words to specified file
    #[structopt(short = "c", long = "compound")]
    compound_path: Option<String>,

    /// Filepath of word list to make compound-safe
    #[structopt(name = "input word list", parse(from_os_str))]
    input_path: PathBuf,
}

fn main() {
    let opt = Opt::from_args();
    let output_dest = match opt.output_path {
        Some(file_path) => file_path,
        None => format!("{}.csafe", &opt.input_path.to_str().unwrap()),
    };
    println!("output_dest is {}", output_dest);

    let inputted_list = make_set_from_file(&opt.input_path);
    let unsafe_words_contenders: Vec<Contenders> =
        find_unsafe_word_contenders(&inputted_list, opt.verbose);

    print_contenders_if_has_path(opt.compound_path, &unsafe_words_contenders);

    let words_to_remove = find_fewest_words_to_remove(unsafe_words_contenders);
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
        "The word list you inputted had {} words ({} bits per word).\n",
        original_list_length,
        log_base(2, *original_list_length as f64)
    );
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

fn print_contenders_if_has_path(
    compound_path: Option<String>,
    unsafe_words_contenders: &[Contenders],
) {
    if let Some(path) = compound_path {
        // unsafe_words_contenders.sort_by(|a, b| a.root_word.cmp(&b.root_word));
        let mut f = File::create(&path).expect("Unable to create file");
        for contender in unsafe_words_contenders {
            if contender.tail.is_empty() {
                writeln!(
                    f,
                    "{}|{} can make {}",
                    contender.root_word, contender.second_word, contender.head
                )
                .expect("Unable to write data to file");
            } else {
                writeln!(
                    f,
                    "{}|{} can make {}|{}",
                    contender.root_word, contender.second_word, contender.head, contender.tail
                )
                .expect("Unable to write data to file");
            }
        }
    }
}
