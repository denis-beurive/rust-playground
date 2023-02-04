#!/usr/bin/perl
#
# Usage:
#
#   perl mapping.pl --help
#   perl mapping.pl --address=0x7ffff7d5c000
#   perl mapping.pl --path=map.txt --address=0x7ffff7d5c000
#
# The map file must be a copy/paste of the GDB output for the command "info proc map".
# For example:
#
#   process 4700
#   Mapped address spaces:
#
#           Start Addr           End Addr       Size     Offset  Perms  objfile
#       0x555555554000     0x55555555a000     0x6000        0x0  r--p   /home/denis/Documents/github/rust-playground/variables/target/debug/variables
#       0x55555555a000     0x555555591000    0x37000     0x6000  r-xp   /home/denis/Documents/github/rust-playground/variables/target/debug/variables
#       0x555555591000     0x55555559f000     0xe000    0x3d000  r--p   /home/denis/Documents/github/rust-playground/variables/target/debug/variables
#       0x5555555a0000     0x5555555a3000     0x3000    0x4b000  r--p   /home/denis/Documents/github/rust-playground/variables/target/debug/variables
#       0x5555555a3000     0x5555555a4000     0x1000    0x4e000  rw-p   /home/denis/Documents/github/rust-playground/variables/target/debug/variables
#       0x5555555a4000     0x5555555c5000    0x21000        0x0  rw-p   [heap]
#       0x7ffff7d5c000     0x7ffff7d5f000     0x3000        0x0  rw-p
#       0x7ffff7d5f000     0x7ffff7d87000    0x28000        0x0  r--p   /usr/lib/x86_64-linux-gnu/libc.so.6
#       0x7ffff7d87000     0x7ffff7f1c000   0x195000    0x28000  r-xp   /usr/lib/x86_64-linux-gnu/libc.so.6
#       0x7ffff7f1c000     0x7ffff7f74000    0x58000   0x1bd000  r--p   /usr/lib/x86_64-linux-gnu/libc.so.6
#       0x7ffff7f74000     0x7ffff7f78000     0x4000   0x214000  r--p   /usr/lib/x86_64-linux-gnu/libc.so.6
#       0x7ffff7f78000     0x7ffff7f7a000     0x2000   0x218000  rw-p   /usr/lib/x86_64-linux-gnu/libc.so.6
#       0x7ffff7f7a000     0x7ffff7f87000     0xd000        0x0  rw-p
#       0x7ffff7f87000     0x7ffff7f8a000     0x3000        0x0  r--p   /usr/lib/x86_64-linux-gnu/libgcc_s.so.1
#       0x7ffff7f8a000     0x7ffff7fa1000    0x17000     0x3000  r-xp   /usr/lib/x86_64-linux-gnu/libgcc_s.so.1
#       0x7ffff7fa1000     0x7ffff7fa5000     0x4000    0x1a000  r--p   /usr/lib/x86_64-linux-gnu/libgcc_s.so.1
#       0x7ffff7fa5000     0x7ffff7fa6000     0x1000    0x1d000  r--p   /usr/lib/x86_64-linux-gnu/libgcc_s.so.1
#       0x7ffff7fa6000     0x7ffff7fa7000     0x1000    0x1e000  rw-p   /usr/lib/x86_64-linux-gnu/libgcc_s.so.1
#       0x7ffff7fb8000     0x7ffff7fb9000     0x1000        0x0  ---p
#       0x7ffff7fb9000     0x7ffff7fbd000     0x4000        0x0  rw-p
#       0x7ffff7fbd000     0x7ffff7fc1000     0x4000        0x0  r--p   [vvar]
#       0x7ffff7fc1000     0x7ffff7fc3000     0x2000        0x0  r-xp   [vdso]
#       0x7ffff7fc3000     0x7ffff7fc5000     0x2000        0x0  r--p   /usr/lib/x86_64-linux-gnu/ld-linux-x86-64.so.2
#       0x7ffff7fc5000     0x7ffff7fef000    0x2a000     0x2000  r-xp   /usr/lib/x86_64-linux-gnu/ld-linux-x86-64.so.2
#       0x7ffff7fef000     0x7ffff7ffa000     0xb000    0x2c000  r--p   /usr/lib/x86_64-linux-gnu/ld-linux-x86-64.so.2
#       0x7ffff7ffb000     0x7ffff7ffd000     0x2000    0x37000  r--p   /usr/lib/x86_64-linux-gnu/ld-linux-x86-64.so.2
#       0x7ffff7ffd000     0x7ffff7fff000     0x2000    0x39000  rw-p   /usr/lib/x86_64-linux-gnu/ld-linux-x86-64.so.2
#       0x7ffffffde000     0x7ffffffff000    0x21000        0x0  rw-p   [stack]
#   0xffffffffff600000 0xffffffffff601000     0x1000        0x0  --xp   [vsyscall]

use strict;
use warnings FATAL => 'all';
use Getopt::Long;
use bignum qw/hex/;
use Data::Dumper;
use constant SECTIONS_DEFAULT_PATH => 'map.txt';
use constant K_START => 'start';
use constant K_STOP => 'stop';
use constant K_HEAP => 'heap';
use constant K_STACK => 'stack';


sub help {
    print("Look for the heap and the stack within the output of the GDB command \"info proc map\"\n\n");
    print("[perl] mapping.pl [--path=<path to map file>] [--address=<address to look at>] [--verbose]\n");
    print("[perl] mapping.pl --help\n\n");
    printf("Default map file: \"%s\"\n", SECTIONS_DEFAULT_PATH);
}

# Parse the command line.
my $cli_mapping_path = SECTIONS_DEFAULT_PATH;
my $cli_verbose = undef;
my $cli_help = undef;
my $cli_address = undef;
if (! GetOptions (
    'path=s'    => \$cli_mapping_path,
    'help'      => \$cli_help,
    'address=s' => \$cli_address,
    'verbose'   => \$cli_verbose)) {
    print("Invalid command line!\n\n");
    help();
    exit(1);
}
if (defined $cli_help) {
    help();
    exit(0);
}

# Open map file.
printf("Open map file \"%s\"\n", $cli_mapping_path) if (defined $cli_verbose);
my $mapping_fd;
if (! open($mapping_fd, '<', $cli_mapping_path)) {
    printf("Cannot open the map file \"%s\": %s\n", $cli_mapping_path, $!);
    exit(1);
}

# Parse the map file.
my %data = ();
while (<$mapping_fd>) {
    chomp;
    # printf("%s\n", $_);
    if ($_ =~ m/^\s+(0x[0-9a-f]+)\s+(0x[0-9a-f]+)\s+(0x[0-9a-f]+)\s+(0x[0-9a-f]+)\s+([a-z\-]+)\s+\[([^\]]+)\]\s*$/) {
        next unless ($6 eq 'heap' || $6 eq 'stack');
        my $location = $6 eq 'heap' ? &K_HEAP : &K_STACK;
        my $start = $1;
        my $stop = $2;
        printf("%-5s: %-18s -> %-18s\n", $location, $start, $stop) if (defined $cli_verbose);
        if (exists($data{$location})) {
            printf("Unexpected error: location \"%s\" found twice!\n", $location);
            exit(1);
        }
        $data{$location} = {
            &K_START => hex($start),
            &K_STOP  => hex($stop) - hex("0x1")
        };
    }
}

printf("HEAP:  [0x%X, 0x%X]\n", $data{&K_HEAP}->{&K_START}, $data{&K_HEAP}->{&K_STOP}) if (exists $data{&K_HEAP});
printf("STACK: [0x%X, 0x%X]\n\n", $data{&K_STACK}->{&K_START}, $data{&K_STACK}->{&K_STOP});
printf("Search HEAP:  find 0x%X, 0x%X, \"expr\"\n", $data{&K_HEAP}->{&K_START}, $data{&K_HEAP}->{&K_STOP})  if (exists $data{&K_HEAP});
printf("Search STACK: find 0x%X, 0x%X, \"expr\"\n\n", $data{&K_STACK}->{&K_START}, $data{&K_STACK}->{&K_STOP});
exit(0) unless(defined $cli_address);

$cli_address = hex($cli_address);
if ($cli_address >= $data{&K_HEAP}->{&K_START} and $cli_address <= $data{&K_HEAP}->{&K_STOP}) {
    printf("0x%X is in the heap\n", $cli_address);
} elsif ($cli_address >= $data{&K_STACK}->{&K_START} and $cli_address <= $data{&K_STACK}->{&K_STOP}) {
    printf("0x%X is in the stack\n", $cli_address);
} else {
    printf("0x%X is neither in the heap nor in the stack\n", $cli_address);
}

