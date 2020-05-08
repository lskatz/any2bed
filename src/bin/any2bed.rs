// to read from files
use std::fs::File;
use std::str::from_utf8;
use std::io::stdout;

// for file extension stuff
use std::path::Path;
use std::ffi::OsStr;

//RUST-BIO
use bio::io::fasta;
use bio::io::gff;
use bio::io::bed;

//GenBank files
use gb_io::reader::SeqReader;

//htslib is for vcf, bam, sam
use rust_htslib::{bam, bam::Read};

// argument parsing
use clap::{Arg, App};

// Get the extension of a filename
fn get_extension_from_filename(filename: &str) -> Option<&str> {
    Path::new(filename)
        .extension()
        .and_then(OsStr::to_str)
}

fn fasta2bed(filename:&str) -> Vec<bed::Record>{

    // Initialize the vector of bed entries
    let mut range :Vec<bed::Record> = Vec::new();

    // open the file and start off the fasta reader
    let file = File::open(filename).expect("Open a fasta file");
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
    let file = File::open(filename).expect("Open a gff file");
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
    let file = File::open(filename).expect("Open a gbk file");
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

fn bam2bed(filename:&str) -> Vec<bed::Record>{

    // Initialize the vector of bed entries
    let mut range :Vec<bed::Record> = Vec::new();

    let mut bam = bam::Reader::from_path(filename).expect("Open bam file");

    // Get header information from the bam
    let header = bam.header();

    // loop through all reference sequences
    for i in 0..header.target_count(){
      // Get characteristics for each sequence
      let name   = from_utf8(header.tid2name(i)).expect("Sequence name");
      let length = header.target_len(i).expect("Sequence length");

      // start off the bed record
      let mut bed_record = bed::Record::new();
      bed_record.set_chrom(name);
      bed_record.set_start(0);
      bed_record.set_end(length as u64 -1);

      range.push(bed_record);
    }

    return range;

}

fn main() {

    // TODO get version, author information from Cargo.toml
    let matches = App::new("any2bed")
        .arg(Arg::with_name("file")
             .value_name("file")
             .multiple(true)
             .help("A fasta/gff/gbk/bam file path"))
        .get_matches();
    
    // use stdout as the output file
    let stdout = stdout();
    let handle = stdout.lock();
    let mut writer = bed::Writer::new(handle);

    let filenames = matches.values_of("file").unwrap();
    for filename in filenames {
      let extension = get_extension_from_filename(filename).expect("File extension");

      // The ranges for this filename
      // The way we get the ranges depends on the file exension.
      let range :Vec<bed::Record> = match extension {
          "fasta"            => {
              fasta2bed(&filename)
          },
          "gbk"        => {
              gbk2bed(&filename)
          },
          "gff"          => {
              gff2bed(&filename)
          },
          "bam"          => {
              bam2bed(&filename)
          }
          _     => {
              panic!("ERROR: I don't know what extension {} is from filename {}", extension, filename)
          }
      };
      for r in range {
          writer.write(&r).expect("ERROR: could not write to file");
      }
    }
}

