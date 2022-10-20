#!/usr/bin/perl
#
# Usage:
#
# $ perl delta.pl 0x555555591000 0x5555555a4000 0x5555555c5000 0x7ffff7d5f000
# sorted values:
#   0x555555591000
#   0x5555555A4000
#   0x5555555C5000
#   0x7FFFF7D5F000
# deltas:
#   0x555555591000 -> 0x5555555A4000 => 77824 (0x13000)
#   0x5555555A4000 -> 0x5555555C5000 => 135168 (0x21000)
#   0x5555555C5000 -> 0x7FFFF7D5F000 => 46912358686720 (0x2AAAA279A000)

use strict;
use warnings FATAL => 'all';
use bignum qw/hex/;
use Data::Dumper;

my @values = map { hex($_) } @ARGV;
my @sorted = sort { $a <=> $b } @values;

print("sorted values:\n");
foreach my $value (@sorted) {
    printf("  0x%X\n", $value);
}

print("deltas:\n");
for my $i (1..$#sorted) {
    my $d = $sorted[$i] - $sorted[$i-1];
    printf("  0x%X -> 0x%X => %d (0x%X)\n", $sorted[$i-1], $sorted[$i], $d, $d);
}


