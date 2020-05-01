// to read from files
use std::fs::File;
//use std::io::prelude::*;

//RUST-BIO
//use std::io;
use bio::io::fasta;

// argument parsing
use clap::{Arg, App};

fn fasta2bed(filename:&str) -> Vec<String>{

    // Initialize the vector of bed entries
    let mut range :Vec<String> = Vec::new();

    // open the file and start off the fasta reader
    let file = File::open(filename).unwrap();
    let reader = fasta::Reader::new(file);

    // parse each entry
    for record in reader.records() {
        let record          = record.unwrap();
        let sequence :&[u8] = record.seq();
        let id              = record.id();
        let length          = sequence.len();
        let length_string    = length.to_string();

        let bed_string       = vec![id, "1", &length_string].join("\t");

        // Save the bed entry into the range vector
        range.push(bed_string); 
    }

    return range;
}

fn gff2bed(filename:&str) -> Vec<String>{
    let range = vec![];
    return range;
}

fn main() {

    let matches = App::new("any2bed")
        .arg(Arg::with_name("fasta")
             .short("f")
             .long("fasta")
             .takes_value(true)
             .value_name("fasta file")
             .multiple(true)
             .help("A fasta file path"))
         .get_matches();

    // Parse fasta files
    for filename in matches.values_of("fasta").unwrap() {
        let range = fasta2bed(filename);
        println!("{:?}", range);
    }

    // Parse gff files
    for filename in matches.values_of("gff").unwrap() {
        let range = gff2bed(filename);
        println!("{:?}", range);
    }
}
