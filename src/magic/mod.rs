use magic::{Cookie, CookieFlags};

// Create a bioinformatics magic database library
pub fn create_magic_cookie() -> Cookie{
    let cookie = Cookie::open(CookieFlags::default()).unwrap();

    let _db_str :&str = _db_str();
    cookie.load(&vec!["/home/lee/src/any2bed/data/bioinfo.mgc"]).unwrap();
    return cookie;
}

fn _db_str() -> &'static str {
  return
  "# Borrowed from https://github.com/lindenb/magic
  #
  # ABI Sequencing file
  # http://www6.appliedbiosystems.com/support/software_community/ABIF_File_Format.pdf
  #
  0	string/b	ABIF	Applied Biosystems Genetic Analysis Data File 

  #
  #
  # CSI spec : http://samtools.github.io/hts-specs/CSIv1.pdf
  #
  0	string	@HD\tVN:1.0\\ SO:coordinate	SAM file v1.0 sorted on coordinates
  0	string	BAM\\1	BAM file v1.0
  0	string	BAI\\1	BAM .bai index file v1.0
  0	string	CSI\\1	BAM .csi index file v1.0
  #
  # UCSC BED: http://genome.ucsc.edu/FAQ/FAQformat.html#format1
  #
  0	regex	[^\t]+\t[0-9]+\t[0-9]+\n	BED Format : 3 columns
  0	regex	[^\t]+\t[0-9]+\t[0-9]+\t[^\t]*\n	BED Format : 4 columns
  0	regex	[^\t]+\t[0-9]+\t[0-9]+\t[^\t]*\t[0-9]+\n	BED Format : 5 columns
  0	regex	[^\t]+\t[0-9]+\t[0-9]+\t[^\t]*\t[0-9]+\t[+-]\n	BED Format : 6 columns
  0	regex	[^\t]+\t[0-9]+\t[0-9]+\t[^\t]*\t[0-9]+\t[+-]\t[0-9]+\n	BED Format : 7 columns
  0	regex	[^\t]+\t[0-9]+\t[0-9]+\t[^\t]*\t[0-9]+\t[+-]\t[0-9]+\t[0-9]+\n	BED Format : 8 columns
  0	regex	[^\t]+\t[0-9]+\t[0-9]+\t[^\t]*\t[0-9]+\t[+-]\t[0-9]+\t[0-9]+	BED Format
  >&0	regex	\t[0-9,]+\n	: 9 columns
  0	regex	[^\t]+\t[0-9]+\t[0-9]+\t[^\t]*\t[0-9]+\t[+-]\t[0-9]+\t[0-9]+	BED Format
  >&0	regex	\t[0-9,]+\t[0-9]+\n	: 10 columns
  0	regex	[^\t]+\t[0-9]+\t[0-9]+\t[^\t]*\t[0-9]+\t[+-]\t[0-9]+\t[0-9]+	BED Format
  >&0	regex	\t[0-9,]+\t[0-9]+\t[0-9,]+\n	: 11 columns
  0	regex	[^\t]+\t[0-9]+\t[0-9]+\t[^\t]*\t[0-9]+\t[+-]\t[0-9]+\t[0-9]+	BED Format
  >&0	regex	\t[0-9,]+\t[0-9]+\t[0-9,]+\t[0-9,]+\n	: 12 columns
  #
  # Big Bed file as specified in http://bioinformatics.oxfordjournals.org/content/26/17/2204/suppl/DC1
  #
  0	long	0x8789F2EB	BigBed File

  #
  # Big Wig file as specified in http://bioinformatics.oxfordjournals.org/content/26/17/2204/suppl/DC1
  #
  0	long	0x888FFC26	BigWig File

  #
  # Blast databases
  # See: http://selab.janelia.org/people/farrarm/blastdbfmtv4/BlastDbFormatV4.pdf 
  #
  0	belong	0x00000004	Blast Database Index File v4.0
  >4	belong	0x00000000	DNA
  >4	belong	0x00000001	Protein
  #https://samtools.github.io/hts-specs/CRAMv2.1.pdf
  #http://samtools.github.io/hts-specs/CRAMv3.pdf
  0	string	CRAM\\2\\1	CRAM 2.1 file
  0	string	CRAM\\3\0	CRAM 3.0 file#
  # Fasta Format
  #
  0	regex   \\>.*[\n\r].*\n	Fasta Sequence
  #!:mime text/fasta
  #
  # Fastq  : http://en.wikipedia.org/wiki/FASTQ_format
  #
  0	regex	@.*\n[ATGCNatgcn]*\n+.*\n[!-I]*\n	Fastq (Sanger)
  !:mime	text/fastq
  0	regex	@.*\n[ATGCNatgcn]*\n+.*\n[;-i]*\n	Fastq (Solexa)
  !:mime	text/fastq
  0	regex	@.*\n[ATGCNatgcn]*\n+.*\n[!-J]*\n	Fastq (Illumina 1.8)
  !:mime	text/fastq
  0	regex	@.*\n[ATGCNatgcn]*\n+.*\n[@-i]*\n	Fastq (Illumina 1.3)
  !:mime	text/fastq
  #
  # http://www.ensembl.org/info/website/upload/gff.html
  #
  0	regex	(#.*\n)*[^t]+\t*[^t]+\t[0-9]+\t[0-9]+\t[0-9eE\\.]+\t[+-]\t[012\\.]\t.*\n	GTF (General Feature Format)
  #
  # Hapmap 
  # ftp://ftp.ncbi.nlm.nih.gov/hapmap/genotypes/2009-01_phaseIII/00README.txt
  #
  0	string	rs#\\ alleles\\ chrom\\ pos\\ strand\\ assembly
  >&0	string	#\\ center\\ protLSID\\ assayLSID\\ panelLSID\\ QCcode HapMap Genotypes
  0	string	rs#\\ chrom\\ pos\\ strand\\ build\\ center\\ protLSID
  >&0	string	\\ assayLSID\\ panelLSID\\ QC_code\\ refallele\\ refallele_freq	HapMap Frequencies

  #
  # LiftOver chain https://genome.ucsc.edu/goldenPath/help/chain.html
  #
  0	regex	chain\\ [0-9]+\\ [^\t]+.*\n	UCSC chain file
  !:strength + 20
  #
  # http://cran.r-project.org/doc/manuals/r-release/R-ints.html
  #
  # See 'static void R_WriteMagic(FILE *fp, int number)' in src/main/saveload.c
  #
  0	string RDA1\n	R Version 1 - R Data, ASCII Format 
  0	string RDA2\n	R Version >=2 - R Data, ASCII Format 
  0	string RDB1\n	R Version 1 - R Data, Binary Format 
  0	string RDB2\n	R Version >=2 - R Data, Binary Format
  0	string RDX1\n	R Version 1 - R Data, XDR Format 
  0	string RDX2\n	R Version >=2 - R Data, XDR Format
  0	string	TBI\\1	Tabix index file v1.0
  #
  # twobit uses 2 magic's:
  #https://github.com/curoverse/get-evidence/blob/master/server/utils/twobit.py

  0	belong	0x1A412743	TwoBit Compressed Sequence, ASCII text
  0	belong	0x4327411A	TwoBit Compressed Sequence (rev. byte order), ASCII text
  #
  # UCSC annotation header
  # http://genome.ucsc.edu/goldenPath/help/customTrack.html
  #
  0	regex	browser.*\n\\(track.*\n\\)*	UCSC annotation header
  #>&0	indirect	x	\\b, contains
  0	string	##fileformat=VCFv4.1 VCF format 4.1
  0	string	BCF\\4	BCF file v1.0
  0	string	BCF\\2\\1	BCF file v2.0
  #
  # Wiggle https://genome.ucsc.edu/goldenPath/help/wiggle.html
  #
  0	regex fixedStep.*\n[0-9\\.eE+-]+\n	UCSC Wig File. Fixed Step
  0	regex variableStep.*\n[0-9]+\t[0-9\\.eE+-]+\n	UCSC Wig File. Variable Step
  ";
}

