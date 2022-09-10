## epitope_diversity

This tool can calculate the diversity (e.g. [Shannon entropy](https://en.wikipedia.org/wiki/Entropy_(information_theory)) and [nucleotide diversity](https://en.wikipedia.org/wiki/Nucleotide_diversity)) **at haplotype level** within a specifed region (e.g. epitope) from a NGS sample. It accepts a alignment BAM file, and a GFF3 file as input and returns a tsv result. The application of this tool can be extended to any genomic regions, not only for immunological epitopes. Written in Rust, fast-running and memory-efficient.

### Usage
```
epitope_diversity 0.1.1
Haogao Gu <koohoko@gmail.com>
A tool for estimating haplotype diversity of specific regions (e.g. epitopes) from a NGS alignment.

USAGE:
    epitope_diversity [OPTIONS] --bam_file <FILE> --pos_file <FILE>

OPTIONS:
    -f, --bam_file <FILE>      Path of BAM file. Must be accompanied with the BAI index file in the
                               same directory.
    -p, --pos_file <FILE>      Path to a GFF3 file specifying genomic positions of interest.
                               Start/End positions should be 1-based rather than 0-based, and should
                               correspond to the positions in the reference sequence used in SAM/BAM
                               alignment.
    -o, --out_file <NUMBER>    Path to write to the outfile, if "-" will write to stdout. [default:
                               -]
    -v, --verbose              Add this flag to also print text results to stderr.
    -h, --help                 Print help information
    -V, --version              Print version information
```

### Examples:
#### Example input files 
Please check [here](/examples/). These example data are retrieved from test files from [IRMA](https://wonder.cdc.gov/amd/flu/irma/).

#### Example command and example output
```
epitope_diversity -f ./examples/A_NP.bam -p "./examples/example.gff" -o -

seqid   start   end     num_of_full_cover_reads num_of_haplotypes    Shannon_entropy population_nucleotide_diversity
A_NP    1096    1112    28      10      1.924832680792314    0.024084633853541412
A_NP    100     120     16      8       2.216917186688699    0.022321428571428572
A_NP    10      1110    No haplotype    No haplotypeNo haplotype
```

### Installation
#### 1. Executable
Directly download executables from [Releases](https://github.com/Koohoko/epitope_diversity/releases).
#### 2. Install from source
1. Install Rust from [here](https://www.rust-lang.org/tools/install).
2. Download source code by `git clone https://github.com/Koohoko/epitope_diversity.git`.
3. Install with `cargo install --path epitope_diversity`.
4. You are ready to go.

### Details
The calculation is based on the formula in [this paper](https://www.sciencedirect.com/science/article/pii/S004268221630037X). Specifically, we used the excat "Shannon entropy" and "Population nucleotide diversity" without normalization/correction in that paper.

![Shannon entropy](https://ars.els-cdn.com/content/image/1-s2.0-S004268221630037X-fx4_lrg.jpg)
![Population nucleotide diversity](https://ars.els-cdn.com/content/image/1-s2.0-S004268221630037X-fx9_lrg.jpg)

### Changelog
- v0.1.1 add columns in the output file for number of haplotypes, and number of total full-cover reads.

### TODO
* [ ] None
---