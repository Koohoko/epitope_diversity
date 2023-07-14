use clap::{crate_description, crate_name, crate_version, arg, Command};
mod analyse;
mod utils;

fn main() {
    // interface and argument matching
    let matches = Command::new(crate_name!())
        .version(crate_version!())
        .author("Haogao Gu <koohoko@gmail.com>")
        .about(crate_description!())
        .arg(
            arg!(
                -f --bam_file <FILE> "Path of BAM file. Must be accompanied with the BAI index file in the same directory."
            )
            .required(true)
            .display_order(1)
        )
        .arg(
            arg!(
                -p --pos_file  <FILE> "Path to a GFF3 file specifying genomic positions of interest. Start/End positions should be 1-based rather than 0-based, and should correspond to the positions in the reference sequence used in SAM/BAM alignment."
            )
            .required(true)
            .display_order(2)
        )
        .arg(
            arg!(
                -o --out_file <FILE> "Path to write to the outfile, if \"-\" will write to stdout."
            )
            .required(false)
            .default_value("-")
            .display_order(3)
        )
        .arg(
            arg!(
                -v --verbose "Add this flag to also print text results to stderr."
            )
            .required(false)
            .display_order(4)
        )
        .get_matches();

    // get arguments
    let check_verbose:bool = match matches.occurrences_of("verbose") {
        0 => false,
        _ => true,
    };
    if check_verbose {eprintln!("{}", "### Job started! ###\n")};

    let bam_file_path = matches.value_of("bam_file").unwrap();
    if check_verbose {eprintln!("bam file: {}", bam_file_path)};

    let pos_file_path = matches.value_of("pos_file").expect("wrong value from pos_file");
    if check_verbose {eprintln!("pos file: {}", pos_file_path)};

    let mut out_file_path = "-";
    if let Some(file_path_output) = matches.value_of("out_file") {
        match file_path_output {
            "-" => {
                out_file_path = "-";
                if check_verbose {eprintln!("output to stdout")};
            },
            _ => {
                out_file_path = file_path_output;
                if check_verbose {eprintln!("Output file: {}", out_file_path)};
            }
        }
    }

    analyse::analyse(bam_file_path, pos_file_path, out_file_path, check_verbose);

    if check_verbose {eprintln!("{}", "\n### Job finished! ###")};
}


