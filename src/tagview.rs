use colored::*;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

/*
*Author Gaurav Sablok

* */

pub fn tagview(
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

    for i in selectedreads_view.iter() {
        for j in i.iter() {
            if *j.to_string() == *"A" {
                print!("{}", j.color("blue").bold());
            } else if *j.to_string() == *"T" {
                print!("{}", j.color("red").bold());
            } else if *j.to_string() == *"G" {
                print!("{}", j.color("yellow").bold());
            } else if *j.to_string() == *"C" {
                print!("{}", j.color("white").bold());
            } else {
                continue;
            }
        }
        println!();
    }

    Ok("The coloured display is as follows:".to_string())
}
