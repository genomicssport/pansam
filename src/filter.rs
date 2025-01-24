use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

/*
 *Author Gaurav Sablok
*/

pub fn filter(pathsam: &str, start: usize) -> Result<String, Box<dyn Error>> {
    #[derive(Debug, Clone, PartialEq, PartialOrd)]
    pub struct Limit {
        pub line: String,
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
    let mut limit: Vec<Limit> = Vec::new();
    for i in limit_lines.iter() {
        let mutable = i.split("\t").filter(|x| *x != "").collect::<Vec<_>>();
        if mutable.len() == 0 {
            continue;
        }
        if mutable[3].parse::<usize>().unwrap() == start {
            limit.push(Limit {
                line: mutable.join(" ").to_string(),
            });
        };
    }
    let mut samtools_start = File::create("sorted_selected-start.sam").expect("file not present");
    for i in limit.iter_mut() {
        writeln!(samtools_start, "{}\n", i.line).expect("not able to write th line");
    }
    Ok("The files have been written and the summary is given below".to_string())
}

pub fn srange(pathsam: &str, start: usize, end: usize) -> Result<String, Box<dyn Error>> {
    #[derive(Debug, Clone, PartialEq, PartialOrd)]
    pub struct UpperLimit {
        pub line: String,
    }

    let fileopen = File::open(pathsam).expect("file not present");
    let fileread = BufReader::new(fileopen);
    let mut upper_lines: Vec<UpperLimit> = Vec::new();

    let mut lines = Vec::new();
    for i in fileread.lines() {
        let line = i.expect("line not found");
        if !line.starts_with("@") {
            let iden = line;
            lines.push(iden);
        }
    }

    for i in lines.iter() {
        let mutable = i.split("\t").filter(|x| *x != "").collect::<Vec<_>>();
        if mutable.len() == 0 {
            continue;
        }
        if mutable[3].parse::<usize>().unwrap() >= start
            && mutable[3].parse::<usize>().unwrap() <= end
        {
            upper_lines.push(UpperLimit {
                line: mutable.join(" ").to_string(),
            });
        };
    }

    let mut commonjoin: Vec<_> = Vec::new();

    for i in upper_lines.iter_mut() {
        commonjoin.push(i.line.clone());
    }

    let mut samtools_range = File::create("samtools-range.sam").expect("file not found");
    for i in commonjoin.iter() {
        writeln!(samtools_range, "{}\n", i).expect("line not found");
    }

    Ok("The files have been written".to_string())
}
