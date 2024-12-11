/*
    Liber Primus
    Timothy Ayers taters79@protonmail.com

    Terminal application to aid in the attempting dechiperment of the Liber Primus. Project used to learn the Rust language

    Crate dependencies:
        - primality-test    v0.3.0
        - terminal          v0.2.1


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
#[command(about = "Provides utilities to iterate on applying decryptions on a per page basis", long_about = None)]
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

/* END COMMAND LINE ARG */


/* HASH MAPS TO CONVERT LATIN TO RUNE AND RUNE TO LATIN */
/*
#[derive(Eq, Debug)]
struct RuneEntry {
    rune: char,
    value: u32,
}

impl PartialEq for RuneEntry {
    fn eq(&self, other: &RuneEntry) -> bool {
        self.rune == other.rune
    }
}

impl RuneEntry {
    fn new(rune: char, value: u32) -> RuneEntry {
        RuneEntry { rune: rune, value: value}
    }
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

// Generate HashMap to assist in calculating the GPSum of a given word
/*
lazy_static! {
    static ref LATINMAP: HashMap<String, u32> = {
        let mut map = HashMap::<String, u32>::new();
        map.insert("F".to_string(),     2);
        map.insert("U".to_string(),     3);
        map.insert("TH".to_string(),    5);
        map.insert("O".to_string(),     7);
        map.insert("R".to_string(),     11);
        map.insert("C".to_string(),     13);
        map.insert("K".to_string(),     13);
        map.insert("G".to_string(),     17);
        map.insert("W".to_string(),     19);
        map.insert("H".to_string(),     23);
        map.insert("N".to_string(),     29);
        map.insert("I".to_string(),     31);
        map.insert("J".to_string(),     37);
        map.insert("EO".to_string(),    41);
        map.insert("P".to_string(),     43);
        map.insert("X".to_string(),     47);
        map.insert("S".to_string(),     53);
        map.insert("Z".to_string(),     53);
        map.insert("T".to_string(),     59);
        map.insert("B".to_string(),     61);
        map.insert("E".to_string(),     67);
        map.insert("M".to_string(),     71);
        map.insert("L".to_string(),     73);
        map.insert("NG".to_string(),    79);
        map.insert("ING".to_string(),   79);
        map.insert("OE".to_string(),    83);
        map.insert("D".to_string(),     89);
        map.insert("A".to_string(),     97);
        map.insert("AE".to_string(),    101);
        map.insert("Y".to_string(),     103);
        map.insert("IA".to_string(),    107);
        map.insert("IO".to_string(),    107);
        map.insert("EA".to_string(),    109);
        map
    };
}
*/
/* END HASH MAPS - PULL OUT INTO LIB/CRATE */


fn main() {
    
    let args = Cli::parse();

    match &args.command {
        Commands::Decrypt { page } => { println!("Loading page {} in decryption mode:", page); }
        Commands::GPSum { word } => {
            println!("GP Sum is: {}", gpsum_latin(&word.to_ascii_uppercase() ));
            return; 
        }
        Commands::Prime { number } => { return; }
    }

    
    //println!("{} {}", RUNEMAP[&'\u{16A0}'].latin, RUNEMAP[&'\u{16A0}'].value);
    //println!("{}", LATINMAP[&"F".to_string()]);
    //println!("{}", GPSET.get(&2).unwrap().rune);

    /* 
        TODO: (tja) - handle command line args
        --help, display all help options
        --gpsum, take text and calculate the GP sum
        --prime, check whether a number are prime, also check for emirp...
        no args, dump into menu
    */

    //let mut app_state = AppState::MENU;
    //let mut buffer = String::new();
    //while app_state != AppState::EXIT {
                
        //for entry in &GP {
        //    buffer.push(entry.0);
        //}

        //println!("{:?}", buffer);

        /*match app_state {
            AppState::MENU => app_state = AppState::LP,
            AppState::LP => app_state = AppState::EXIT,
            AppState::EXIT => return
        }*/    
    //}
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