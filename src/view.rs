use colored::Colorize;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::process::Command;

/*
*Author Gaurav Sablok

* */
pub fn alignview(
    pathsam: &str,
    pathlist: &str,
    pathfasta: &str,
    upstream: usize,
    downstream: usize,
    prankaligner: &str,
) -> Result<String, Box<dyn Error>> {
    #[derive(Debug, Clone, PartialEq, PartialOrd)]
    pub struct Limit {
        pub headerline: String,
        pub line: String,
        pub startcoordinate: usize,
    }

    #[derive(Debug, Clone, PartialEq, PartialOrd)]
    pub struct Fastaline {
        pub header: String,
        pub sequence: String,
    }

    let fileopen = File::open(pathsam).expect("file not present");
    let fileread = BufReader::new(fileopen);
    let mut limit_lines = Vec::new();
    for i in fileread.lines() {
        let line = i.expect("line not found");
        if !line.starts_with("@") {
            let iden = line;
            limit_lines.push(iden);
        }
    }

    let stringfile = File::open(pathlist).expect("file not present");
    let stringfileread = BufReader::new(stringfile);
    let mut stringsearch: Vec<String> = Vec::new();
    for i in stringfileread.lines() {
        let id = i.expect("line not found");
        stringsearch.push(id)
    }

    let mut fasta_storage: Vec<Fastaline> = Vec::new();
    let mut fasta_header: Vec<String> = Vec::new();
    let mut fasta_sequence: Vec<String> = Vec::new();

    let fastaopen = File::open(pathfasta).expect("file not present");
    let fastaread = BufReader::new(fastaopen);
    for i in fastaread.lines() {
        let line = i.expect("file not present");
        if line.starts_with(">") {
            fasta_header.push(line.replace(">", ""))
        }
        if !line.starts_with(">") {
            fasta_sequence.push(line)
        }
    }
    for i in 0..fasta_header.len() {
        fasta_storage.push(Fastaline {
            header: fasta_header[i].clone(),
            sequence: fasta_sequence[i].clone(),
        })
    }

    let mut limit: Vec<Limit> = Vec::new();
    for i in limit_lines.iter() {
        let mutable = i.split("\t").filter(|x| !x.is_empty()).collect::<Vec<_>>();
        if mutable.is_empty() {
            continue;
        } else {
            limit.push(Limit {
                headerline: mutable[2].to_string(),
                line: mutable[9].to_string(),
                startcoordinate: mutable[3].parse::<usize>().unwrap(),
            });
        }
    }

    let mut subset: Vec<Fastaline> = Vec::new();

    for i in 0..stringsearch.len() {
        for j in fasta_storage.iter() {
            if j.header == stringsearch[i] {
                subset.push(Fastaline {
                    header: j.header.clone(),
                    sequence: j.sequence.clone(),
                })
            }
        }
    }

    #[derive(Debug, Clone, PartialEq, PartialOrd)]
    pub struct LimitFinal {
        pub header: String,
        pub read: String,
        pub sequence: String,
        pub upstream: String,
        pub downstream: String,
        pub upstream_region_downstream: String,
    }

    let mut final_struct: Vec<LimitFinal> = Vec::new();

    for i in limit.iter() {
        for j in subset.iter() {
            if j.header == i.headerline {
                final_struct.push(LimitFinal {
                    header: j.header.clone(),
                    read: i.line.clone(),
                    sequence: j.sequence.clone(),
                    upstream: j.sequence[i.startcoordinate - upstream..i.startcoordinate]
                        .to_string(),
                    downstream: j.sequence[i.startcoordinate..i.startcoordinate + downstream]
                        .to_string(),
                    upstream_region_downstream: j.sequence
                        [i.startcoordinate - upstream..i.startcoordinate + downstream]
                        .to_string(),
                })
            }
        }
    }

    let mut upstream_write = File::create("selected-ids-upstream.fasta").expect("file not present");
    for i in final_struct.iter_mut() {
        write!(upstream_write, ">{}\n{}\n", i.header, i.upstream)
            .expect("not able to write th line");
    }

    let mut downstream_write =
        File::create("selected-ids-downstream.fasta").expect("file not present");
    for i in final_struct.iter_mut() {
        write!(downstream_write, ">{}\n{}\n", i.header, i.downstream)
            .expect("not able to write th line");
    }

    let mut reads_write = File::create("selected-ids-reads.fasta").expect("file not present");
    for i in final_struct.iter_mut() {
        write!(reads_write, ">{}\n{}\n", i.header, i.read).expect("not able to write th line");
    }

    let mut upstream_region_downstream_write =
        File::create("selected-ids-upstream-region-downstream.fasta").expect("file not present");
    for i in final_struct.iter_mut() {
        write!(
            upstream_region_downstream_write,
            ">{}\n{}\n",
            i.header, i.upstream_region_downstream
        )
        .expect("not able to write th line");
    }

    let _aligned = Command::new(prankaligner)
        .arg("-d=selected-ids-reads.fasta")
        .arg("-o=selected-ids-reads-aligned.fasta")
        .output()
        .expect("failed to execute");

    let _aligned = Command::new(prankaligner)
        .arg("-d=selected-ids-upstream.fasta")
        .arg("-o=selected-ids-upstream-aligned.fasta")
        .output()
        .expect("failed to execute");

    let _aligned = Command::new(prankaligner)
        .arg("-d=selected-ids-downstream.fasta")
        .arg("-o=selected-ids-downstream-aligned.fasta")
        .output()
        .expect("failed to execute");

    let _aligned = Command::new(prankaligner)
        .arg("-d=selected-ids-upstream-region-downstream.fasta")
        .arg("-o=selected-ids-upstream-region-downstream-aliged.fasta")
        .output()
        .expect("failed to execute");

    let upstream_aligned =
        File::open("selected-ids-upstream-aligned.fasta.best.fas").expect("File not found");
    let downstream_aligned =
        File::open("selected-ids-downstream-aligned.fasta.best.fas").expect("File not found");
    let upstream_region_downstream =
        File::open("selected-ids-upstream-region-downstream-aliged.fasta.best.fas")
            .expect("File not found");

    #[derive(Debug, Clone, PartialEq, PartialOrd)]
    pub struct Embedded {
        pub header: String,
        pub sequence: String,
    }

    let mut upstream_header: Vec<String> = Vec::new();
    let mut upstream_sequence: Vec<String> = Vec::new();
    let upstream_read = BufReader::new(upstream_aligned);
    for i in upstream_read.lines() {
        let line = i.expect("line not present");
        if line.starts_with(">") {
            upstream_header.push(line);
        } else if !line.starts_with(">") {
            upstream_sequence.push(line);
        }
    }

    let mut upstream_embedded: Vec<Embedded> = Vec::new();

    for i in 0..upstream_header.len() {
        upstream_embedded.push(Embedded {
            header: upstream_header[i].clone(),
            sequence: upstream_sequence[i].clone(),
        })
    }

    let mut upstream_finalholdseq_multivector = Vec::new();
    let mut upstream_finalholdid_multivector: Vec<String> = Vec::new();
    for i in 0..upstream_header.len() {
        let mut intermediatehold = Vec::new();
        for j in upstream_sequence[i].chars() {
            intermediatehold.push(j);
        }
        upstream_finalholdseq_multivector.push(intermediatehold);
        upstream_finalholdid_multivector.push(upstream_header[i].clone());
    }

    println!("The upstream colour coded alignments are: ");
    for i in 0..upstream_finalholdseq_multivector.len() - 1 {
        for j in 0..upstream_finalholdseq_multivector[0].len() {
            if upstream_finalholdseq_multivector[i][j].to_string()
                == upstream_finalholdseq_multivector[i + 1][j].to_string()
            {
                print!(
                    "{}{}",
                    upstream_finalholdseq_multivector[i][j]
                        .to_string()
                        .blue()
                        .bold(),
                    upstream_finalholdseq_multivector[i + 1][j]
                        .to_string()
                        .blue()
                        .bold()
                )
            } else if upstream_finalholdseq_multivector[i][j].to_string()
                != upstream_finalholdseq_multivector[i + 1][j].to_string()
            {
                print!(
                    "{}{}",
                    upstream_finalholdseq_multivector[i][j]
                        .to_string()
                        .red()
                        .bold(),
                    upstream_finalholdseq_multivector[i + 1][j]
                        .to_string()
                        .white()
                        .bold()
                )
            } else {
                continue;
            }
        }
        println!();
    }

    let mut downstream_header: Vec<String> = Vec::new();
    let mut downstream_sequence: Vec<String> = Vec::new();
    let downstream_read = BufReader::new(downstream_aligned);
    for i in downstream_read.lines() {
        let line = i.expect("line not present");
        if line.starts_with(">") {
            downstream_header.push(line);
        } else if !line.starts_with(">") {
            downstream_sequence.push(line);
        }
    }

    let mut downstream_embedded: Vec<Embedded> = Vec::new();

    for i in 0..downstream_header.len() {
        downstream_embedded.push(Embedded {
            header: downstream_header[i].clone(),
            sequence: downstream_sequence[i].clone(),
        })
    }

    let mut downstream_finalholdseq_multivector = Vec::new();
    let mut downstream_finalholdid_multivector: Vec<String> = Vec::new();
    for i in 0..downstream_header.len() {
        let mut intermediatehold = Vec::new();
        for j in upstream_sequence[i].chars() {
            intermediatehold.push(j);
        }
        downstream_finalholdseq_multivector.push(intermediatehold);
        downstream_finalholdid_multivector.push(upstream_header[i].clone());
    }

    println!("The downstream colour coded alignments are: ");
    for i in 0..downstream_finalholdseq_multivector.len() - 1 {
        for j in 0..downstream_finalholdseq_multivector[0].len() {
            if downstream_finalholdseq_multivector[i][j].to_string()
                == downstream_finalholdseq_multivector[i + 1][j].to_string()
            {
                print!(
                    "{}{}",
                    downstream_finalholdseq_multivector[i][j]
                        .to_string()
                        .blue()
                        .bold(),
                    downstream_finalholdseq_multivector[i + 1][j]
                        .to_string()
                        .blue()
                        .bold()
                )
            } else if downstream_finalholdseq_multivector[i][j].to_string()
                != downstream_finalholdseq_multivector[i + 1][j].to_string()
            {
                print!(
                    "{}{}",
                    downstream_finalholdseq_multivector[i][j]
                        .to_string()
                        .red()
                        .bold(),
                    downstream_finalholdseq_multivector[i + 1][j]
                        .to_string()
                        .white()
                        .bold()
                )
            } else {
                continue;
            }
        }
        println!();
    }

    let mut upstream_region_downstream_header: Vec<String> = Vec::new();
    let mut upstream_region_downstream_sequence: Vec<String> = Vec::new();
    let upstream_region_downstream_read = BufReader::new(upstream_region_downstream);
    for i in upstream_region_downstream_read.lines() {
        let line = i.expect("line not present");
        if line.starts_with(">") {
            upstream_region_downstream_header.push(line);
        } else if !line.starts_with(">") {
            upstream_region_downstream_sequence.push(line);
        }
    }

    let mut upstream_region_downstream_embedded: Vec<Embedded> = Vec::new();

    for i in 0..upstream_region_downstream_header.len() {
        upstream_region_downstream_embedded.push(Embedded {
            header: upstream_region_downstream_header[i].clone(),
            sequence: upstream_region_downstream_sequence[i].clone(),
        })
    }

    let mut upstream_region_downstream_finalholdseq_multivector = Vec::new();
    let mut upstream_region_downstream_finalholdid_multivector: Vec<String> = Vec::new();
    for i in 0..upstream_region_downstream_header.len() {
        let mut intermediatehold = Vec::new();
        for j in upstream_region_downstream_sequence[i].chars() {
            intermediatehold.push(j);
        }
        upstream_region_downstream_finalholdseq_multivector.push(intermediatehold);
        upstream_region_downstream_finalholdid_multivector.push(upstream_header[i].clone());
    }

    println!("The upstream_region_downstream colour coded alignments are: ");
    for i in 0..upstream_region_downstream_finalholdseq_multivector.len() - 1 {
        for j in 0..upstream_region_downstream_finalholdseq_multivector[0].len() {
            if upstream_region_downstream_finalholdseq_multivector[i][j].to_string()
                == upstream_region_downstream_finalholdseq_multivector[i + 1][j].to_string()
            {
                print!(
                    "{}{}",
                    upstream_region_downstream_finalholdseq_multivector[i][j]
                        .to_string()
                        .blue()
                        .bold(),
                    upstream_region_downstream_finalholdseq_multivector[i + 1][j]
                        .to_string()
                        .blue()
                        .bold()
                )
            } else if upstream_region_downstream_finalholdseq_multivector[i][j].to_string()
                != upstream_region_downstream_finalholdseq_multivector[i + 1][j].to_string()
            {
                print!(
                    "{}{}",
                    upstream_region_downstream_finalholdseq_multivector[i][j]
                        .to_string()
                        .red()
                        .bold(),
                    upstream_region_downstream_finalholdseq_multivector[i + 1][j]
                        .to_string()
                        .white()
                        .bold()
                )
            } else {
                continue;
            }
        }
        println!();
    }

    Ok("The files have been written and the summary is given below".to_string())
}
