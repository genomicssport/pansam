use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

/*
*Author Gaurav Sablok
 */

pub fn fastid(pathsam: &str) -> Result<String, Box<dyn Error>> {
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
        let mutable = i.split("\t").filter(|x| !x.is_empty()).collect::<Vec<_>>();
        if mutable.is_empty() {
            continue;
        } else {
            limit.push(Limit {
                line: mutable[2].to_string(),
            });
        }
    }
    let mut sorted_id = File::create("samids.txt").expect("file not present");
    for i in limit.iter_mut() {
        writeln!(sorted_id, "{}", i.line).expect("not able to write th line");
    }
    Ok("The files have been written and the summary is given below".to_string())
}
