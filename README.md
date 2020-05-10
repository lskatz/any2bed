# any2bed

Turn any file into a bed file.
Currently supports gbk, fasta, sam, bam file formats.

## installation

    # after downloading
    cd any2bed
    cargo build --release
    cp -nv target/release/any2bed ~/bin/

## Usage

    any2bed --help

## quick start

    any2bed something.bam > out.bed

