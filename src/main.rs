extern crate clap;
use clap::{Arg, App};
use std::io::Read;
use std::io::BufReader;
use std::fs::File;
use colored::Colorize;

fn main() {
    let matches = App::new("rstrings")
                            .version("Version: 0.1.0")
                            .about("Finds n long sequences of printable characters in binary files.")
                            .arg(Arg::new("file")
                                .value_name("file")
                                .help("Sets the file to search in")
                                .required(true)
                                .takes_value(true)
                                .index(1))
                            .arg(Arg::new("show-index")
                                .short('i')
                                .long("show-index")
                                .value_name("index")
                                .help("Shows the offset of the character sequences")
                                .takes_value(false))
                            .arg(Arg::new("no-color")
                                .long("no-color")
                                .value_name("no-color")
                                .help("Disables colorizing output")
                                .takes_value(false))
                            .arg(Arg::new("n")
                                .value_name("n")
                                .help("Specifies minimum length of the sequence of printable characters")
                                .takes_value(true)
                                .short('n'))
                            .get_matches();
    
    let rstrings_file = matches.value_of("file").unwrap();
    let length_of_sequence = matches.value_of("n").unwrap_or("4").parse::<usize>().unwrap();
    let should_print_offset = match matches.occurrences_of("show-index") {
        0 => false,
        1 | _ => true
    };

    let should_colorize = match matches.occurrences_of("no-color") {
        0 => true,
        1 | _ => false
    };

    let f = File::open(rstrings_file).expect("Failed opening file");
    let mut reader = BufReader::new(f);
    let mut buffer = Vec::new();

    reader.read_to_end(&mut buffer).expect("Failed reading file");
    let sequence_indexes = return_printable_characters_indexes(&buffer, length_of_sequence);
    let (start_sequence_indexes, end_sequence_indexes) = sequence_indexes;
    for i in 0..start_sequence_indexes.len() {
        if should_print_offset {
            if should_colorize {
                let offset = format!("{}", start_sequence_indexes[i]).red();
                print!("{} ", offset);
            } else {
                print!("{} ", start_sequence_indexes[i]);
            }
        }
        for j in start_sequence_indexes[i]..end_sequence_indexes[i] {
            if should_colorize {
                let output = format!("{}", buffer[j] as char).bold().green();
                print!("{}", output);
            } else {
                print!("{}", buffer[j] as char);
            }
        }
        println!();
    }
    
    
}

fn return_printable_characters_indexes(buffer: &Vec<u8>, n: usize) ->  (Vec<usize>, Vec<usize>) {
    let mut length = 0;
    let mut offset = 0;
    let mut start_sequence_index = Vec::new();
    let mut end_sequence_index = Vec::new();

    for value in buffer {
        if *value <= 126 && *value >= 32 {
            length += 1;
        } else {
            if length >= n {
                start_sequence_index.push(offset - length);
                end_sequence_index.push(offset);
            }
            length = 0;
        }
        offset += 1;
    }

    (start_sequence_index, end_sequence_index)
}