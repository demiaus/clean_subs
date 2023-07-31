use std::io::prelude::*;
use std::env;
use std::fs;
use std::fs::File;

/* TODO:
- without arguments find first .srt file from folder
- name output .srt to match video file in folder
*/

fn main() -> std::io::Result<()> {

    let args: Vec<String> = env::args().collect();
    if args.len() > 2 {
        panic!("Too many arguments");
    }

    if args.len() == 1 {
        todo!("autofinding .srt not implemented");
    }

    let filename = &args[1];
    let mut input = File::open(filename)?;
    fs::rename(filename, format!("{}.bak", filename))?;
    let mut output = File::create(format!("{}", filename))?;
    let mut contents = String::new();
    input.read_to_string(&mut contents)?;

    let open = ['<', '[', '('];
    let closed = ['>', ']', ')'];
    let mut stack = Vec::new();
    let mut new = String::new();
    let lines = ["- ", "♪♪" ];


    for c in contents.chars() {

        if open.contains(&c) {
            stack.push(c);
            continue;
        }

        if closed.contains(&c) {
            stack.pop();
            continue;
        }

        if stack.is_empty() {
            new.push(c);
        }
    }

    for line in new.lines() {
        if !lines.contains(&line) {
            println!("{}", line);
            writeln!(&mut output, "{}", line)?;
        }
    }

    Ok(())

}
