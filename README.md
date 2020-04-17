# any2bed
Convert files to bed format

Inspired by the functionality in `makeRanges.pl` in the Lyve-SET project.
The name is inspired by another project any2fasta.

# Quick start

Usage: `any2bed *.fasta *.sam *.vcf.gz > output.bed`

# Dependencies

* perl with multithreading (installed on most systems)
* zgrep (installed on most systems)

# Installation

    cd ~/bin
    git clone https://github.com/lskatz/any2bed
    cp -nv ~/bin/any2bed/scripts/any2bed ~/bin

