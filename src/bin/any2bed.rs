use bio::io::fasta::Reader;
//use rust_htslib::{bam, bam::Read};

/*
fn bam2bed(filename:&str){

  let mut bam = bam::Reader::from_path(filename).unwrap();

  for p in bam.pileup() {
    let pileup = p.unwrap();
    println!("{}:{} depth {}", pileup.tid(), pileup.pos(), pileup.depth());

    for alignment in pileup.alignments() {
      if !alignment.is_del() && !alignment.is_refskip() {
        println!("Base {}", alignment.record().seq()[alignment.qpos().unwrap()]);
      }

      // mark indel start
      match alignment.indel() {
        bam::pileup::Indel::Ins(len) => println!("Insertion of length {} between this and next position.", len),
        bam::pileup::Indel::Del(len) => println!("Deletion of length {} between this and next position.", len),
        bam::pileup::Indel::None     => ()
      }
    }
  }

}
*/

fn main() {
  let filename = &"t/single.bam";
  //bam2bed(filename);
}
