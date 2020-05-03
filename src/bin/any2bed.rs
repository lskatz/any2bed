// to read from files
use std::fs::File;
use std::io::stdout;

//RUST-BIO
//use std::io;
use bio::io::fasta;
use bio::io::gff;
use bio::io::bed;

use gb_io::reader::SeqReader;

// argument parsing
use clap::{Arg, App};

fn fasta2bed(filename:&str) -> Vec<bed::Record>{

    // Initialize the vector of bed entries
    let mut range :Vec<bed::Record> = Vec::new();

    // open the file and start off the fasta reader
    let file = File::open(filename).unwrap();
    let reader = fasta::Reader::new(file);

    // parse each entry
    for record in reader.records() {
        let record          = record.unwrap();
        let sequence :&[u8] = record.seq();
        let id              = record.id();
        let length          = sequence.len();

        let mut bed_record  = bed::Record::new();
        bed_record.set_chrom(id);
        bed_record.set_start(0 as u64);
        bed_record.set_end(length as u64 -1);

        // Save the bed entry into the range vector
        range.push(bed_record); 
    }

    return range;
}

fn gff2bed(filename:&str) -> Vec<bed::Record>{

    // Initialize the vector of bed entries
    let mut range :Vec<bed::Record> = Vec::new();

    // open the file and start off the fasta reader
    let file = File::open(filename).unwrap();
    let mut reader = gff::Reader::new(file, gff::GffType::GFF3);

    // parse each entry
    for record in reader.records() {
        let record          = record.unwrap();
        let id              = record.seqname();
        let start           = record.start();
        let end             = record.end();

        let mut bed_record  = bed::Record::new();
        bed_record.set_chrom(id);
        bed_record.set_start(*start as u64 - 1);
        bed_record.set_end(*end as u64 -1);

        // Save the bed entry into the range vector
        range.push(bed_record); 
    }

    return range;
}
fn gbk2bed(filename:&str) -> Vec<bed::Record>{

    // Initialize the vector of bed entries
    let mut range :Vec<bed::Record> = Vec::new();

    // open the file and start off the fasta reader
    let file = File::open(filename).unwrap();
    let reader = SeqReader::new(file);
    for seq in reader {
        let seq    = seq.unwrap();
        let length = seq.len();
        let name   = seq.name.unwrap();

        let mut bed_record  = bed::Record::new();
        bed_record.set_chrom(&name);
        bed_record.set_start(0);
        bed_record.set_end(length as u64 -1);

        range.push(bed_record);
    }

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
        .arg(Arg::with_name("gff")
             .short("g")
             .long("gff")
             .takes_value(true)
             .value_name("gff file")
             .multiple(true)
             .help("A gff file path"))
        .arg(Arg::with_name("gbk")
             .short("g")
             .long("gbk")
             .takes_value(true)
             .value_name("genbank file")
             .multiple(true)
             .help("A genbank file path"))
        .get_matches();
    
    // use stdout as the output file
    let stdout = stdout();
    let handle = stdout.lock();
    let mut writer = bed::Writer::new(handle);

    // Parse fasta files
    let fasta_filenames = matches.values_of("fasta").unwrap();
    for filename in fasta_filenames {
        let range = fasta2bed(filename);
        for record in range {
            writer.write(&record).expect("ERROR: could not write to file");
        }
    }

    // Parse gff files
    let gff_filenames = matches.values_of("gff").unwrap();
    for filename in gff_filenames {
        let range = gff2bed(filename);
        for record in range {
            writer.write(&record).expect("ERROR: could not write to file");
        }
    }

}

