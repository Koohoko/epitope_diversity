use crate::utils;
use rust_htslib::bam::{Read};
use std::{collections::HashMap, io::{self, BufWriter, Write}, path::Path};
use std::fs::File;

pub fn analyse(bam_file_path:&str, pos_file_path:&str, out_file_path:&str, check_verbose:bool){
    // read bam file, return the bam object
    let mut bam_reader = utils::reader::read_bam(bam_file_path);

    // read pos file
    let mut pos_reader = utils::reader::read_pos(pos_file_path);

	//out put results to tsv
	let mut writer: BufWriter<Box<dyn Write>> = match out_file_path {
		"-" => {
			let stdout = io::stdout();
    		let lock = stdout.lock();
			BufWriter::new(Box::new(lock))
		},
		_ => {
			let path = Path::new(out_file_path);
        	BufWriter::new(Box::new(File::create(path).unwrap()))
		},
	};
	writer.write_all(b"seqid\tstart\tend\tnum_of_full_cover_reads\tnum_of_haplotypes\tShannon_entropy\tpopulation_nucleotide_diversity\n").unwrap();

	// loop through the positions, and calculate the diversity metrics
	for record in pos_reader.records() {
		let rec = record.ok().expect("Error reading record.");
		let rec_name = rec.seqname();
		let rec_start = (rec.start().clone() - 1) as i64;
		let rec_end = (rec.end().clone() - 1) as i64;
		if check_verbose {
			eprintln!("------------------------------------------------------------");
			eprintln!("start: {}", rec.start());
			eprintln!("end: {}", rec.end());	
		}
				
		writer.write_fmt(format_args!("{}\t{}\t{}\t", rec_name, rec_start+1, rec_end+1));
		
		// bam_reader.fetch((rec_name));
		bam_reader.fetch((rec_name, rec_start, rec_end));
		let mut hm_haplotype = HashMap::new();

		'outer: for read in
			bam_reader.rc_records().map(|x| x.expect("Failure parsing Bam file")) {
			let read_start_pos = read.pos();
			if read_start_pos > rec_start { continue; }
			let read_end_pos = read.cigar().end_pos();
			if read_end_pos < rec_end { continue; }

			// check the CIGAR
			// eprintln!("Pos: {}; Seq len: {}; end pos: {}; CIGAR: {:?}", read.pos(), read.seq_len(), read.cigar().end_pos(), read.cigar().take());

			let ref_positions = rec_start..(rec_end+1);
			let mut seq  = Vec::new();
			for pos_i in ref_positions {
				let ref_pos = read.cigar().read_pos(pos_i as u32, false, false).unwrap();
				match ref_pos {
					None => {continue 'outer;},
					// None => {seq.push(78u8);},
					_ => {
						unsafe{
							let base = read.seq().encoded_base_unchecked(ref_pos.unwrap() as usize);
							seq.push(base);
						}
					}
				}
			}
			
			let n = hm_haplotype.entry(seq).or_insert(0u32);
			*n += 1;

		}
		
		let hap_counts:Vec<&u32> = hm_haplotype.values().collect();
		let total_hap_counts:u32 = hap_counts.clone().into_iter().sum();
		
		// shannon entropy
		let hap_p_vec: Vec<(&Vec<u8>, f64)> = hm_haplotype.iter().map(|(x,&y)|(x, (y as f64)/(total_hap_counts as f64))).collect();
		let mut hs: f64 = hap_p_vec.iter().map(|(_, y)| (y)*f64::log2(*y )).sum();
		hs = -hs;
		if check_verbose { eprintln!("Shannon entropy: {:?}", hs);}

		// nucleotide diversity
		// https://www.sciencedirect.com/science/article/pii/S004268221630037X
		let num_unique_haplotypes = hap_p_vec.len();
		if num_unique_haplotypes == 0 { 
			writer.write_fmt(format_args!("{0}\t{0}\t{0}\n", "No haplotype"));
			continue;
		}
		let epitope_len = hap_p_vec[1].0.len();
		let mut hpi = 0f64;

		for i in 1..num_unique_haplotypes {
			let seq_1_info = hap_p_vec[i];
			for j in 0..(num_unique_haplotypes-1){
				if j < i {
					let seq_2_info = hap_p_vec[j];
					let mut num_nt_diff = 0f64;
					for k in 0..epitope_len {
						if seq_1_info.0[k] != seq_2_info.0[k] {
							num_nt_diff += 1f64;
						}
					}
					hpi = hpi + (seq_1_info.1 * (num_nt_diff as f64) * seq_2_info.1);
				}
			}
		}
		hpi = hpi/(epitope_len as f64);

		if check_verbose{
			eprintln!("tnum_of_full_cover_reads: {:?}", total_hap_counts); 
			eprintln!("num_unique_haplotypes: {:?}", num_unique_haplotypes); 
			eprintln!("Population nucleotide diversity: {:?}", hpi); 
		}
		writer.write_fmt(format_args!("{}\t{}\t{}\t{}\n", total_hap_counts, num_unique_haplotypes, hs, hpi));

	}


}	