use colored::*;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

/*
*Author Gaurav Sablok

* */

pub fn readsview(
    pathsam: &str,
    genomestart: usize,
    genomeend: usize,
) -> Result<String, Box<dyn Error>> {
    #[derive(Debug, Clone, PartialEq, PartialOrd)]
    pub struct SelectedReads {
        pub line: String,
    }

    let fileopen = File::open(pathsam).expect("file not present");
    let fileread = BufReader::new(fileopen);
    let mut selected_reads: Vec<SelectedReads> = Vec::new();

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
        if mutable[3].parse::<usize>().unwrap() >= genomestart
            && mutable[3].parse::<usize>().unwrap() <= genomeend
        {
            selected_reads.push(SelectedReads {
                line: mutable[9].to_string(),
            });
        }
    }

    let mut selectedreads_view: Vec<Vec<_>> = Vec::new();

    for i in selected_reads.iter() {
        let readcapture = i.line.clone();
        let mut readchar: Vec<_> = Vec::new();
        for i in readcapture.chars() {
            readchar.push(i.to_string());
        }
        selectedreads_view.push(readchar)
    }

    for i in 0..selectedreads_view.len() - 2 {
        let line = selectedreads_view[i].join("");
        let linenext = selectedreads_view[i + 1].join("");
        let linelast = selectedreads_view[i + 2].join("");
        println!(
            "{}\n{}\n{}",
            line.color("blue"),
            linenext.color("yellow"),
            linelast.color("red")
        );
    }

    Ok("The coloured display is as follows:".to_string())
}
