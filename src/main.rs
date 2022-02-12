use clap::Parser;
use std::io::Read;
use std::io::BufReader;
use std::fs::File;
use colored::Colorize;

fn main() {
    #[derive(Parser, Debug)]
    #[clap(version = "0.1.0", about = "Finds n long sequences of printable characters in binary files.")]
    struct Args {
        /// Name of the file to read from
        file: String,

        /// Shows index of the first character in the sequence
        #[clap(short = 'i', long = "show-index")]
        show_index: bool,

        /// Choosing this will result printing output without colorizing
        #[clap(long = "no-color")]
        show_color: bool,
        
        /// Specifies minimum length of the printable characters sequence
        #[clap(short = 'n', long = "sequence-length", default_value_t = 4)]
        sequence_length: usize
    }

    let args = Args::parse();
    let f = File::open(args.file).expect("Failed opening file");
    let mut reader = BufReader::new(f);
    let mut buffer = Vec::new();

    reader.read_to_end(&mut buffer).expect("Failed reading file");
    let sequence_indexes = return_printable_characters_indexes(&buffer, args.sequence_length);
    let (start_sequence_indexes, end_sequence_indexes) = sequence_indexes;
    for i in 0..start_sequence_indexes.len() {
        if args.show_index {
            if !args.show_color {
                let offset = format!("{}", start_sequence_indexes[i]).yellow();
                print!("{} ", offset);
            } else {
                print!("{} ", start_sequence_indexes[i]);
            }
        }
        for j in start_sequence_indexes[i]..end_sequence_indexes[i] {
            if !args.show_color {
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
    let mut index = 0;
    let mut start_sequence_index = Vec::new();
    let mut end_sequence_index = Vec::new();

    for value in buffer {
        if *value <= 126 && *value >= 32 {
            length += 1;
        } else {
            if length >= n {
                start_sequence_index.push(index - length);
                end_sequence_index.push(index);
            }
            length = 0;
        }
        index += 1;
    }

    (start_sequence_index, end_sequence_index)
}