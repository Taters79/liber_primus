/*
    Liber Primus
    Timothy Ayers taters79@protonmail.com

    Terminal application to aid in the attempting dechiperment of the Liber Primus. Project used to learn the Rust language


    TODO: (tja)
        - Need other prime math functions, Totient etc..

*/

use std::collections::HashMap;
use clap::{Parser, Subcommand};
use lazy_static::lazy_static;

/*
    COMMAND LINE ARG 
*/

#[derive(Parser, Debug)]
#[command(name = "lp - Liber Primus")]
#[command(version = "0.0.1")]
#[command(about = "Provides utilities to check the gpsum on a given latin word, find the primality of a given number, and also apply decryption algorithms to a LP page.", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Decrypt { page: u32 },
    GPSum { word: String, },
    Prime { number: u32, }
}


/* END COMMAND LINE ARG */


/*
// Application state
#[derive(Debug, PartialEq, Eq)]
enum AppState {
    MENU,
    LP,
    EXIT
}

enum RuneColor {
    WHITE,
    YELLOW,
    RED
}
*/

#[derive(Eq, Debug)]
struct LatinEntry {
    latin: String,
    value: u32,
}

impl PartialEq for LatinEntry {
    fn eq(&self, other: &LatinEntry) -> bool {
        self.latin == other.latin
    }
}

impl LatinEntry {
    fn new(latin: String, value: u32 ) -> LatinEntry {
        LatinEntry { latin: latin, value: value}
    }
}


// Generate HashMap to go from Rune to Latin - Used for decryption/analysis
lazy_static! {
    static ref RUNEMAP: HashMap<char, LatinEntry> = {
        let mut map = HashMap::<char, LatinEntry>::new();
        map.insert('\u{16A0}', LatinEntry::new("F".to_string(), 2));
        map
    };
}



fn main() {
    
    let args = Cli::parse();

    match &args.command {
        Commands::Decrypt { page } => { decrypt(page); }
        Commands::GPSum { word } => {
            println!("GP Sum is: {}", gpsum_latin(&word.to_ascii_uppercase() ));
            return; 
        }
        Commands::Prime { number } => { return; }
    }

    
    /* 
        TODO: (tja) - handle command line args
        --help, display all help options
        --gpsum, take text and calculate the GP sum
        --prime, check whether a number are prime, also check for emirp...
        no args, dump into menu
    */
    
}

fn decrypt(page: &u32) {
   println!("Loading page {} in decryption mode:", page);  
}

fn gpsum_latin(word: &String) -> u32 {
    let mut sum = 0;
    //println!("{}", word);
    let mut iter = word.chars().peekable();

    while let Some(c) = iter.next() {
        match c {
            'F' => sum += 2,
            'U' => sum += 3,
            'T' => {
                if *iter.peek().unwrap() == 'H' {
                    sum += 5;
                    iter.next();
                    continue;
                }
                sum += 59;
            },
            'O' => {
                if *iter.peek().unwrap() == 'E' {
                    sum += 83;
                    iter.next();
                    continue;
                }
                sum += 7
            },
            'R' => sum += 11,
            'C' => sum += 13,
            'K' => sum += 13,
            'G' => sum += 17,
            'W' => sum += 19,
            'H' => sum += 23,
            'N' => {
                if *iter.peek().unwrap() == 'G' {
                    sum += 79;
                    iter.next();
                    continue;
                } 
                sum += 29;
            },
            'I' => {
                if *iter.peek().unwrap() == 'A' || *iter.peek().unwrap() == 'O' {
                    sum += 107;
                    iter.next();
                    continue;
                }
                else if *iter.peek().unwrap() == 'N' {
                    iter.next();
                    let temp = iter.next();

                    if temp == None {
                        println!("No G");
                        // no G, add the I and N values
                        sum += 29 + 31;
                        continue;
                    }
                    else if temp.unwrap() == 'G' {
                        sum += 79;
                        continue;
                    }
                    
                }
                sum += 31
            },
            'J' => sum += 37,
            'E' => {
                if *iter.peek().unwrap() == 'O' {
                    sum += 41;
                    iter.next();
                    continue;
                }
                else if *iter.peek().unwrap() == 'A' {
                    sum += 109;
                    iter.next();
                    continue;
                }
                sum += 67;
            },
            'P' => sum += 43,
            'X' => sum += 47,
            'S' => sum += 53,
            'Z' => sum += 53,
            'B' => sum += 61,
            'M' => sum += 71,
            'L' => sum += 73,
            'D' => sum += 89,
            'A' => {
                if *iter.peek().unwrap() == 'E' {
                    sum += 101;
                    iter.next();
                    continue;
                }
                sum += 97;
            },
            'Y' => sum += 103,
            _ => continue
        }
    }
    
    sum
}