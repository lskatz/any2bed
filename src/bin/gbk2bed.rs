// to read from files
use std::fs::File;
//use std::io::prelude::*;

//RUST-BIO
//use std::io;
use bio::io::fasta;

use gb_io::reader::SeqReader;

// argument parsing
use clap::{Arg, App};

fn gbk2bed(filename:&str) -> Vec<String>{

    // Initialize the vector of bed entries
    let mut range :Vec<String> = Vec::new();

    // open the file and start off the fasta reader
    let file = File::open(filename).unwrap();
    let reader = SeqReader::new(file);
    for seq in reader {
        let seq    = seq.unwrap();
        let length = seq.len();
        let name   = seq.name.unwrap();
        println!("{} => {}", name, length);
    }

    return range;
}

fn main() {

    let matches = App::new("any2bed")
        .arg(Arg::with_name("gbk")
             .short("g")
             .long("gbk")
             .takes_value(true)
             .value_name("genbank file")
             .multiple(true)
             .help("A genbank file path"))
         .get_matches();

    // Parse fasta files
    for filename in matches.values_of("gbk").unwrap() {
        let range = gbk2bed(filename);
        println!("{:?}", range);
    }

}
