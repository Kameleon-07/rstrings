use clap::Parser;
use std::io::{prelude::*, BufReader};
use std::fs::File;

const GREEN: &str = "\x1b[1;32m";
const RESET: &str = "\x1b[0m";

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
        no_color: bool,
        
        /// Specifies minimum length of the printable characters sequence
        #[clap(short = 'n', long = "sequence-length", default_value_t = 4)]
        sequence_length: usize
    }

    let args = Args::parse();
    let f = File::open(args.file).expect("Failed opening file");
    let mut reader = BufReader::new(f);
    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer).expect("Failed reading file");

        let mut length = 0;
        let mut printable_characters_sequence = String::new();
        let mut sequence_start = 0;

        for (index, value) in buffer.iter().enumerate() {
            if *value <= 126 && *value >= 32 {
                if length == 0 {
                    sequence_start = index
                }
                length += 1;
                printable_characters_sequence.push(*value as char);
            } else {
                if length >= args.sequence_length {
                    if args.show_index {
                        printable_characters_sequence.insert_str(0, format!("{} ", sequence_start).as_str());
                    }

                    printable_characters_sequence.insert_str(0, GREEN);
                    printable_characters_sequence.push_str(RESET);

                    if args.no_color {
                        for _i in 0..8 {
                            printable_characters_sequence.remove(0);
                        }
                    }
                    println!("{}", printable_characters_sequence);
                }

                length = 0;
                printable_characters_sequence = String::new();
            }
        }
}