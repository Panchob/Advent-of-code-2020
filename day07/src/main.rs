use std::{
    fs::File,
    io::{prelude::*, BufReader},
    collections::HashMap,
    path::Path,
};


fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn parse_rules(lines: Vec<String>) -> HashMap<String, Vec<String>>{
    lines.iter().map(|line|{
        let i = line.replace("bags", "").replace("bag", "");
        let mut s = i.split(" contain ");
        let key = s.next().unwrap().replace(" ", "");
        let r = s.next().unwrap().trim_end_matches(".");
        let rules : Vec<String> = if r == "no other " {
            vec![]
        } else {
            r.split(", ").map(|k| k.replace(" ", "").to_string()).collect()
        };
        (String::from(key), rules)

    }).collect()
}


fn main() {
    let lines = lines_from_file("input.txt");
    let rules = parse_rules(lines);

    for(k,v) in rules {
        println!("\n{} ->", k);
        for i in v {
            println!("{}", i);
        }
    }

}
