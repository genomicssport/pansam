use std::fs::File;
use std::error::Error;
use std::io::{BufRead, BufReader, Write};

/*
 *Author Gaurav Sablok
*/

pub fn fastidrange(pathsam: &str, start: usize, end: usize) -> Result<String, Box<dyn Error>> {
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
        let mutable = i.split("\t").filter(|x| !x.is_empty()).collect::<Vec<_>>();
        if mutable.is_empty() {
            continue;
        } else if mutable[3].parse::<usize>().unwrap() >= start
            && mutable[3].parse::<usize>().unwrap() <= end
        {
            {
                upper_lines.push(UpperLimit {
                    line: mutable[2].to_string(),
                });
            }
        }
    }

    let mut rangeid = File::create("rust-samtoolsidranges.txt").expect("file not found");
    for i in upper_lines.iter() {
        write!(rangeid, "{}\n", i.line).expect("line not found");
    }

    Ok("The files have been written".to_string())
}
