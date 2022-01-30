extern crate clap;
use clap::{Arg, App};
use std::io::Read;
use std::io::BufReader;
use std::fs::File;

fn main() {
    let matches = App::new("rstrings")
                            .version("Version: 0.1.0")
                            .about("Finds n long sequences of printable characters in binary files.")
                            .arg(Arg::new("file")
                                .short('f')
                                .long("file")
                                .value_name("file")
                                .help("Sets the file to search in")
                                .required(true)
                                .takes_value(true))
                            .arg(Arg::new("offset")
                                .short('o')
                                .long("offset")
                                .value_name("offset")
                                .help("Shows the offset of the character sequences")
                                .takes_value(false))
                            .get_matches();
    
    let rstrings_file = matches.value_of("file").unwrap();
    let should_print_offset = match matches.occurrences_of("offset") {
        0 => false,
        1 | _ => true
    };
    let f = File::open(rstrings_file).expect("Failed opening file");
    let mut reader = BufReader::new(f);
    let mut buffer = Vec::new();

    reader.read_to_end(&mut buffer).expect("Failed reading file");
    find_and_print_printable_characters(buffer, 4, should_print_offset);
}

fn find_and_print_printable_characters(buffer: Vec<u8>, n: usize, should_print_offset: bool) {
    let mut potential_characters: Vec<char> = Vec::new();
    let mut offset = 0;

    for value in buffer {
        if value <= 126 && value >= 32 {
            potential_characters.push(value as char);
        } else {
            if potential_characters.len() >= n {
                if should_print_offset {
                    print!("{} ", offset - potential_characters.len());
                }
                print_vector_of_characters(&potential_characters);
                println!();
            }
            potential_characters.clear();
        }

        offset += 1;
    }
}

fn print_vector_of_characters (characters: &Vec<char>){
    for character in characters {
        print!("{}", character);
    }
}