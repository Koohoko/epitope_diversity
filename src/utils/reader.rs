use std::fs::File;
use bio::io::gff::{Reader, GffType};

pub fn read_bam(bam_file_path:&str) -> rust_htslib::bam::IndexedReader {
	let mut bam_reader = rust_htslib::bam::IndexedReader::from_path(bam_file_path).unwrap();
	bam_reader
}

pub fn read_pos(pos_file_path:&str) -> Reader<File> {
	let mut pos_reader = Reader::from_file(pos_file_path, GffType::GFF3).unwrap();
	pos_reader
}