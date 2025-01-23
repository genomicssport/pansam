use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

/*
*Author Gaurav Sablok

* */
pub fn capture(
    pathsam: &str,
    pathlist: &str,
    pathfasta: &str,
    upstream: usize,
    downstream: usize,
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

    Ok("The files have been written and the summary is given below".to_string())
}
