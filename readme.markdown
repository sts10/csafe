# CSafe

**Compound Passphrase List Safety Checker**

This command line tool checks whether a given word list (such as a diceware word list) has any words that can be combined to make another word on the list. This is very much **a toy project**, so I'd heavily caution against trusting it for real-world security applications. 


I also have written [a blog post](https://sts10.github.io/2021/04/24/revisiting-compund-safety.html) about this tool. 

<!-- Initially I wanted to make sure that no two words in [the EFF's long diceware word list](https://www.eff.org/deeplinks/2016/07/new-wordlists-random-passphrases) could be combined to make another word on the list. The tool here is generalized to check any such word list. -->

## Disclosures!

I am not a professional developer, researcher, or statistician, and frankly I'm pretty fuzzy on some of this math. This code/theory/explanation could be very wrong (but hopefully not harmful?). If you think it could be wrong or harmful, please create an issue! 

Further disclosures: see "Caveat" section below.

## Related projects by me that may interest you

[Tidy](https://github.com/sts10/tidy) is a Rust command-line tool that cleans and combines word lists. Notably, it can also optionally remove ["prefix" words](https://en.wikipedia.org/wiki/Prefix_code). It's my understanding that a word list that does not have any prefix words is "compound-safe", as I define that term below. And while removing all prefix words may remove more words than strictly necessary to guarantee compound-safety, it's a simpler concept to understand and check for, and thus likely better for actually vetting a word list for use without word separators than this project.

Also, CSafe is an updated version of [this project](https://github.com/sts10/compound-passphrase-list-safety-checker) if you want to check that out.

## What is "compound-safety"? 

First of all, I made up the term. (Maybe there's a better, more established term out there...)

Basically, a passphrase word list is "compound-safe" if it does NOT contain any pairs of words that can be combined such that they can be guessed in two distinct ways within the same word-length space. This includes instances in which two words can be combined and form another word on the list.

I heard of this potential issue in [this YouTube video](https://youtu.be/Pe_3cFuSw1E?t=8m36s). 

## Brief examples of compound safety violations

**Example #1**: If a word list included "under", "dog", and "underdog" as three separate words, it would NOT be compound-safe, since "under" and "dog" can be combined to make the word "underdog". A user not using spaces between words might get a passphrase that included the character string "underdog" as two words, but a brute-force attack would guess it as one word. Therefore this word list would NOT be compound-safe.

**Example #2**: Let's say a word list included "paper", "paperboy", "boyhood", and "hood". A user not using punctuation between words might get the following two words next to each other in a passphrase: "paperboyhood", which would be able to be brute-force guessed as both `[paperboy][hood]` and `[paper][boyhood]`. Therefore this word list would NOT be compound-safe. 

Another way to think about example 2: if, for every pair of words, you mash them together, there must be only ONE way to split them apart and make two words on the list. "paperboyhood" can be split in two ways. It is an _ambiguous_ pairing. This is how I approached the issue when writing the code for this project.

## Why is the compound-safety of a passphrase word list notable? 

Let's say we're using the word list described above, which has "under", "dog" and "underdog" in it. A user might randomly get "under" and "dog" in a row, for example in the six-word passphrase "crueltyfrailunderdogcyclingapostle". The user might assume they had six words worth of entropy. But really, an attacker brute forcing their way through five-word passphrases would eventually crack the passphrase. 

Likewise if we got the 6-word phrase "divingpaperboyhoodemployeepastelgravity", an attacker running through six-word combinations would have two chances of guessing "paperboyhood" rather than one.

**It's important to note** that if the passphrase has any punctuation (for example, a period, comma, hyphen, space) between words, both of these issues go away completely. If our passphrase is "cruelty under dog daylight paper boyhood": (1) an attacker who tries "underdog" as the third word does not get a match, (2) and the attacker likewise does not get a match if "paperboy" is guessed in the fifth slot and "hood is guessed as the sixth.

## What this tool does

This tool takes a word list (as a file) as an input. It assumes there's one word per line. It then searches the given list for compound-unsafe words.

Next, it attempts to find the smallest number of words that need to be removed in order to make the given word list "compound-safe". In our example above, if we remove any one of the four words ("paper", "paperboy", "boyhood", and "hood") the list becomes safe from this particular compounding. The program tries to choose which word to remove based on maximizing how many other compounding each word would "solve".

Finally, it prints out this new, shorter, compound-safe (csafe) list to a new text file, called output. In this way it makes word lists "compound-safe" (or at least more safe -- see "Known issue" and "Caveat" sections below).

## How to use this tool to check a word list

### Installation

1. [Install Rust](https://www.rust-lang.org/tools/install) if you haven't already
2. Run: `cargo install --git https://github.com/sts10/csafe --branch main`

### Usage

```text
USAGE:
    csafe [FLAGS] [OPTIONS] <input word list>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information
    -v, --verbose    Give verbose output

OPTIONS:
    -c, --compound <compound-path>    Write discovered compound words to specified file
    -o, --output <output-path>        Write new, compound-safe list to specified file

ARGS:
    <input word list>    Filepath of word list to make compound-safe
```

If you don't provide an output destination with the `-o` flag, csafe will write the compound-safe list to a file next to your inputted list with the same filename plus a `.csafe` extension at the end.

## An example use-case

At one time, the password manager 1Password used [this list of 18,328 words](https://github.com/agilebits/crackme/blob/master/doc/AgileWords.txt) to generate passphrases for users. The list is not compound safe, though this is NOT a security issue for 1Password, since the app's UI prevents users from creating passphrases without punctuation between words.

However, since it is a "real world" passphrase list and it's not compound-safe, it makes for a good demonstration for csafe. Given [that list](https://github.com/sts10/csafe/blob/main/word_lists/agile_words.txt), csafe was able to make [a compound-safe version](https://github.com/sts10/csafe/blob/main/word_lists/agile_words.txt.csafe) by only removing 1,540 words, leaving 16,789 words on the list. Note that the safer but more drastic approach of [removing all prefix words leaves you with just 15,190 words](https://github.com/sts10/prefix-safety-checker/blob/master/word_lists/agile_words.txt.no-prefix).

Again: 1Password's software, as far as I know, does NOT allow users to generate random passphrase without punctuation between words. Users _must_ choose to separate words with a period, hyphen, space, comma, or underscore. So these findings do NOT constitute a security issue with 1Password.

## Caveats / known issues

This project only looks for "two-word compounding", where two words, mashed together, can be read in more than one way. But it's conceivable to me that there a possibility of a three-word compounding -- where three words can be combined and the split two different ways. This tool does NOT currently check for this, so I can't actually guarantee that the lists outputted by the tool are completely compound-safe. 

This another reason to more simply remove all prefix words, as [the EFF word list creator apparently did](https://www.eff.org/deeplinks/2016/07/new-wordlists-random-passphrases). You can remove all prefix words from a list with another tool I wrote called [Tidy](https://github.com/sts10/tidy).

## Thanks 

Huge thanks to [@wezm](https://github.com/wezm) for help speeding up the program by orders of magnitude. 

## Running tests and benchmarks

`cargo test` runs a few basic tests. 

`cargo bench` uses [the Criterion crate](https://crates.io/crates/criterion) to benchmark the main unsafe word search function, located in `src/lib.rs`. If you're trying to help speed this project up (which would be much appreciated!) this will hopefully be useful to you.

## To do

- [X] Use structopt to make it a proper CLI
- [ ] Use multiple threads to speed up the process. 
- [ ] Make the command line text output during the process cleaner and more professional-looking.

## Lingering questions

### Probability of getting a non-safe pair

Given a word list that is not compound-safe, how can we calculate the probability of generating a non-safe pair in a passphrase of a given length (say, 6 words)?

I can hazard a guess here: I put forward that it's the number of unsafe contenders divided by the square of the length of the original word list. Using CSafe's `-c` option, I found that the Agile list gives us 393,400 contenders. Note that only half of these are unique, but I think for the math below that's OK -- we still use the total number of contenders.

Now my contention is that **each contender also represents a possible unsafe two-word combination**. How many possible two-word combinations are there? I'm pretty sure that's just the square of the length of the original word list.

Evaluating `393400/(18328*18328)` gives us a 0.117% chance of getting an unsafe word combination when we put two words next to each other (without punctuation!). If we assume the user is generating a 6-word passphrase that means there will be 5 two-word combinations, which means there's about a 0.586% chance that the passphrase has at least one unsafe compounding. Again, I'm just spit-balling here -- I'm not yet confident enough in this formula to write it into the program. I suppose this could be tested empirically... maybe another program for another day. Work on any of this, as an issue or pull request, is welcome.

<!-- 2. Given this probability, does it make sense, or is it useful, to calculate a revised bits-per-word measure of the list? (For the record I think this would be harmful, but I pose it here for inspiration.) -->

<!-- 3. If a word list has no prefix words, is it definitely compound-safe? Assuming yes. -->

