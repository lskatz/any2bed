#!/usr/bin/env perl
use strict;
use warnings;
use Test::More tests=>1;
use Data::Dumper;
use File::Basename qw/basename/;
use FindBin qw/$RealBin/;

$ENV{PATH} = "$RealBin/../scripts:$ENV{PATH}";

subtest 'single-entry' => sub{

  my $expected = join("\t", qw(ThisIsAnId 0 15));
  my $fasta = "$RealBin/single.fasta";
  my $sam   = "$RealBin/single.sam";
  my $vcf   = "$RealBin/single.vcf";
  my $bed   = "$RealBin/single.bed";

  # Test on a fasta file
  my $testName = "any2bed on fasta";
  system("any2bed $fasta > $bed");
  if($?){
    fail($testName);
  } else {
    open(my $fh, $bed) or die "ERROR: could not read $bed: $!";
    my $line = <$fh>;
    chomp($line);
    close $fh;

    is($line, $expected, $testName);
  }

  $testName = "any2bed on sam";
  $expected = join("\t", qw(ref 0 44));
  system("any2bed $sam > $bed");
  if($?){
    fail($testName);
  } else {
    open(my $fh, $bed) or die "ERROR: could not read $bed: $!";
    my $line = <$fh>;
    chomp($line);
    close $fh;

    is($line, $expected, $testName);
  }

  $testName = "any2bed on vcf";
  $expected = join("\t", qw(NC_001416.1 0 48501));
  system("any2bed $vcf > $bed");
  if($?){
    fail($testName);
  } else {
    open(my $fh, $bed) or die "ERROR: could not read $bed: $!";
    my $line = <$fh>;
    chomp($line);
    close $fh;

    is($line, $expected, $testName);
  }

};


