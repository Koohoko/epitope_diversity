## epitope_diversity

This tool is used to calculate the diversity of specific epitope population within a NGS sample. It accepts a alignment BAM file as input and returns Shannon entropy and nucleotide diversity of specified genomic regions.

### Usage
```
epitope_diversity 0.1.0
Haogao Gu <koohoko@gmail.com>
A tool for estimating epitope diversity of specific regions from a NGS alignment, written in Rust.

USAGE:
    epitope_diversity [OPTIONS] --bam_file <FILE> --pos_file <FILE>

OPTIONS:
    -f, --bam_file <FILE>      Path of BAM file. Must be accompanied with the BAI index file in the
                               same directory.
    -p, --pos_file <FILE>      Path to a GFF3 file
                               (https://github.com/The-Sequence-Ontology/Specifications/blob/master/gff3.md)
                               specifying genomic positions of interest. Start/End positions should
                               be 1-based rather than 0-based, and should correspond to the
                               positions in the reference sequence used in SAM/BAM alignment.
    -o, --out_file <NUMBER>    Path to write to the outfile, if "-" will write to stdout. [default:
                               -]
    -v, --verbose              Add this flap to also print text results to stderr.
    -h, --help                 Print help information
    -V, --version              Print version information
```

### Examples:
```
epitope_diversity -f ./examples/A_NP.bam -p "./examples/example.gff" -o -

seqid   start   end     Shannon_entropy population_nucleotide_diversity
A_NP    1096    1112    0.9432729241850654      0.008239030777416173
A_NP    100     120     0.34073390040305973     0.001586620825134043
A_NP    10      1110    No haplotype    No haplotype

```

### Installation
#### Executable
Directly download executables from [Releases](https://github.com/Koohoko/epitope_diversity/releases).
#### Install from source
1. Install Rust from [here](https://www.rust-lang.org/tools/install).
2. Download source code by `git clone https://github.com/Koohoko/epitope_diversity.git`.
3. Install with `cargo install --path epitope_diversity`.
4. You are ready to go.

---